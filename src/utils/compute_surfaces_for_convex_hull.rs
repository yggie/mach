use maths::{ Vector, TOLERANCE };
use utils::Surface;

/// Computes a set of `Surfaces` for the point cloud provided. The computation
/// assumes that all points are on the convex hull of the point cloud.
pub fn compute_surfaces_for_convex_hull(vertices: &Vec<Vector>) -> Vec<Surface> {
    let mut surfaces: Vec<Surface> = Vec::new();
    let (initial_surface, mut available_nodes, mut free_edge_list) = initialize_surface(vertices);
    surfaces.push(initial_surface);

    while let Some(current_edge) = free_edge_list.pop() {
        let selection = select_best_node_for_edge(&available_nodes, &free_edge_list, current_edge);

        match selection {
            Some(Result::Free(list_index, vertex_index)) => {
                let (new_surface, new_edges) = new_surface_from_edge(vertices, current_edge, vertex_index);
                surfaces.push(new_surface);
                available_nodes[list_index].on_edge = true;

                for new_edge in new_edges.iter() {
                    free_edge_list.push(*new_edge);
                }
            },

            Some(Result::OnEdge(_, vertex_index)) => {
                let (new_surface, new_edges) = new_surface_from_edge(vertices, current_edge, vertex_index);
                surfaces.push(new_surface);

                for new_edge in new_edges.iter() {
                    let similar_edge_index = free_edge_list.iter()
                        .enumerate()
                        .find(|&(_, edge)| edge.has_similar_nodes(&new_edge))
                        .map(|(index, _)| index);

                    match similar_edge_index {
                        Some(index) => { free_edge_list.remove(index); },
                        None => { free_edge_list.push(*new_edge); },
                    }
                }

                for &current_vertex_index in current_edge.nodes.iter() {
                    let node_present_in_edges = free_edge_list.iter()
                        .any(|n| {
                            n.nodes[0] == current_vertex_index ||
                                n.nodes[1] == current_vertex_index
                        });

                    if !node_present_in_edges {
                        let index = available_nodes.iter()
                            .enumerate()
                            .find(|&(_, node)| {
                                node.index == current_vertex_index
                            })
                            .map(|(index, _)| index)
                            .unwrap();

                        available_nodes.remove(index);
                    }
                }
            }

            None => free_edge_list.insert(0, current_edge),
        }
    }

    return surfaces;
}

enum Result {
    Free(usize, usize),
    OnEdge(usize, usize),
}

#[derive(Clone, Copy)]
struct Node {
    index: usize,
    position: Vector,
    on_edge: bool,
}

impl Node {
    fn new(index: usize, position: Vector) -> Node {
        Node {
            index: index,
            position: position,
            on_edge: false,
        }
    }
}

#[derive(Clone, Copy)]
struct DirectedEdge {
    nodes: [usize; 2],
    up_vector: Vector,
    direction: Vector,
    point_on_edge: Vector,
}

impl DirectedEdge {
    fn new(vertices: &Vec<Vector>, surface: &Surface, index_0: usize, index_1: usize) -> DirectedEdge {
        let node_index_0 = surface.nodes[index_0];
        let node_index_1 = surface.nodes[index_1];

        let edge_vector = (vertices[node_index_1] - vertices[node_index_0]).normalize();
        let from_surface_centroid = vertices[node_index_0] - Surface::compute_centroid(&surface, vertices);
        let mut direction = edge_vector.cross(surface.normal).normalize();

        if direction.dot(from_surface_centroid.normalize()) < -TOLERANCE {
            direction = -direction;
        }

        DirectedEdge {
            nodes: [node_index_0, node_index_1],
            direction: direction,
            up_vector: surface.normal,
            point_on_edge: vertices[node_index_0],
        }
    }

    fn has_node(&self, node: Node) -> bool {
        self.nodes[0] == node.index || self.nodes[1] == node.index
    }

    fn shares_a_node_with(&self, edge: DirectedEdge) -> bool {
        self.nodes[0] == edge.nodes[0] ||
            self.nodes[0] == edge.nodes[1] ||
            self.nodes[1] == edge.nodes[0] ||
            self.nodes[1] == edge.nodes[1]
    }

    fn project_to_directed_plane(&self, vertex: Vector) -> (f32, f32) {
        let relative_position = vertex - self.point_on_edge;
        let ref_x = relative_position.dot(self.direction);
        let ref_y = relative_position.dot(self.up_vector);

        return (ref_x, ref_y);
    }

    fn has_similar_nodes(&self, edge: &DirectedEdge) -> bool {
        (self.nodes[0] == edge.nodes[0] && self.nodes[1] == edge.nodes[1]) ||
            (self.nodes[0] == edge.nodes[1] && self.nodes[1] == edge.nodes[0])
    }
}

fn initialize_surface(vertices: &Vec<Vector>) -> (Surface, Vec<Node>, Vec<DirectedEdge>) {
    let mut available_nodes: Vec<Node> = vertices.iter()
        .enumerate()
        .map(|(index, v)| Node::new(index, *v))
        .collect();

    let mut first_surface_option: Option<Surface> = None;
    'outer: for index_0 in (0..vertices.len()) {
        for index_1 in ((index_0 + 1)..vertices.len()) {
            for index_2 in ((index_1 + 1)..vertices.len()) {

                let trial_surface = Surface::new(vertices, index_0, index_1, index_2);
                let point_on_surface = vertices[index_0];
                let valid_surface = vertices.iter()
                    .all(|&vertex| {
                        trial_surface.normal.dot(point_on_surface - vertex) > -TOLERANCE
                    });

                if valid_surface {
                    first_surface_option = Some(trial_surface);
                    break 'outer;
                }
            }
        }
    }

    let surface = first_surface_option.unwrap();
    for i in (0..3) {
        let index = surface.nodes[i];
        available_nodes[index].on_edge = true;
    }

    let free_edge_list = vec!(
        DirectedEdge::new(vertices, &surface, 0, 1),
        DirectedEdge::new(vertices, &surface, 0, 2),
        DirectedEdge::new(vertices, &surface, 1, 2),
    );

    return (surface, available_nodes, free_edge_list);
}

fn select_best_node_for_edge(available_nodes: &Vec<Node>, edge_list: &Vec<DirectedEdge>, current_edge: DirectedEdge) -> Option<Result> {
    let filtered_nodes_with_gradients_iter = available_nodes.iter()
        .enumerate()
        .filter_map(|(node_index, node)| {
            let (ref_x, ref_y) = current_edge.project_to_directed_plane(node.position);
            let ref_distance = ref_x*ref_x + ref_y*ref_y;

            if (ref_x < -TOLERANCE && ref_y > -TOLERANCE) || ref_distance < TOLERANCE*TOLERANCE {
                return None;
            } else {
                let gradient = ref_y.atan2(ref_x + TOLERANCE);
                return Some((node_index, node, gradient));
            }
        });

    let mut max_gradient = -1e5;
    let mut node_with_max_gradient = None;
    for node_with_gradient in filtered_nodes_with_gradients_iter {
        let gradient: f32 = node_with_gradient.2;
        if gradient > max_gradient {
            max_gradient = gradient;
            node_with_max_gradient = Some(node_with_gradient);
        }
    }

    return node_with_max_gradient
        .and_then(|original| {
            let node = original.1;
            if node.on_edge {
                let is_valid_node_for_edge = edge_list.iter()
                    .any(|edge| {
                        edge.has_node(*node) && edge.shares_a_node_with(current_edge)
                    });

                if !is_valid_node_for_edge {
                    return None;
                }
            }

            return Some(original);
        })
        .map(|(node_index, node, _)| {
            if node.on_edge {
                Result::OnEdge(node_index, node.index)
            } else {
                Result::Free(node_index, node.index)
            }
        });
}

fn new_surface_from_edge(vertices: &Vec<Vector>, edge: DirectedEdge, vertex_index: usize) -> (Surface, [DirectedEdge; 2]) {
    let new_surface = Surface::new(vertices, vertex_index, edge.nodes[0], edge.nodes[1]);
    let mut new_edges: Vec<DirectedEdge> = Vec::new();
    let combinations = [
        (0, 1),
        (0, 2),
        (1, 2),
    ];

    for &(index_0, index_1) in combinations.iter() {
        let new_edge = DirectedEdge::new(vertices, &new_surface, index_0, index_1);

        if new_edge.has_similar_nodes(&edge) {
            continue;
        }

        new_edges.push(new_edge);
    }

    debug_assert!(new_edges.len() == 2, "the indices provided to generate the surface were not unique!");
    return (new_surface, [new_edges.remove(0), new_edges.remove(0)]);
}
