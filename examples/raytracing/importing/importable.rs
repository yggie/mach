extern crate regex;

use std::{io, fs};
use std::io::BufRead;

use mach::{Scalar, Vec3D};
use self::regex::Regex;

use raytracing::{Canvas, Color, DirectionalLight, PointLight, RayTracer, Renderer, SceneGeometry, SceneParams, SceneObject};
use raytracing::importing::MatrixStack;

pub trait Importable: Sized {
    fn import_from(&str) -> Result<Self, io::Error>;
}

impl<T> Importable for Renderer<T> where T: RayTracer {
    fn import_from(filename: &str) -> Result<Self, io::Error> {
        let f = try!(fs::File::open(filename));
        let file = io::BufReader::new(&f);

        let ignore_line = Regex::new(r"^\s*(?:#.*)?$").unwrap();
        let size_regex = Regex::new(r"^size\s+(\d+)\s+(\d+)\s*$").unwrap();
        let bounces_regex = Regex::new(r"^maxdepth\s+(\d+)\s*$").unwrap();
        let output_regex = Regex::new(r"^output\s+([^\s].+)$").unwrap();
        let camera_regex = Regex::new(r"^camera\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)$").unwrap();
        let max_vertices_regex = Regex::new(r"^maxverts\s+(\d+)$").unwrap();
        let max_vertex_norms_regex = Regex::new(r"^maxvertnorms\s+(\d+)$").unwrap();
        let sphere_regex = Regex::new(r"^sphere\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)$").unwrap();
        let vertex_regex = Regex::new(r"^vertex\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s*$").unwrap();
        let vertex_normal_regex = Regex::new(r"^vertexnormal\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s*$").unwrap();
        let triangle_regex = Regex::new(r"^tri\s+(\d+)\s+(\d+)\s+(\d+)\s*$").unwrap();
        let translate_regex = Regex::new(r"^translate\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s*$").unwrap();
        let rotate_regex = Regex::new(r"^rotate\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s*$").unwrap();
        let scale_regex = Regex::new(r"^scale\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s*$").unwrap();
        let push_regex = Regex::new(r"^pushTransform\s*$").unwrap();
        let pop_regex = Regex::new(r"^popTransform\s*$").unwrap();
        let directional_light_regex = Regex::new(r"^directional\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)$").unwrap();
        let point_light_regex = Regex::new(r"^point\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)$").unwrap();
        let attenuation_regex = Regex::new(r"^attenuation\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)$").unwrap();
        let ambient_regex = Regex::new(r"^ambient\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)$").unwrap();
        let diffuse_regex = Regex::new(r"^diffuse\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)$").unwrap();
        let specular_regex = Regex::new(r"^specular\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)$").unwrap();
        let emission_regex = Regex::new(r"^emission\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)\s+((?:-|\+)?\d+(?:\.\d+)?)$").unwrap();
        let shininess_regex = Regex::new(r"^shininess\s+((?:-|\+)?\d+(?:\.\d+)?)$").unwrap();

        let mut canvas = Canvas::default();
        let mut scene_params = SceneParams::default();
        let mut scene_object_prototype = SceneObject::default();

        let mut output_filename = None;

        let mut vertices: Vec<(f64, f64, f64)> = Vec::new();
        let mut vertices_with_normals: Vec<((f64, f64, f64), (f64, f64, f64))> = Vec::new();
        let mut matrix_stack = MatrixStack::new();

        for (index, line_result) in file.lines().enumerate() {
            match line_result.unwrap() {
                ref line if Regex::is_match(&ignore_line, line) => (),

                ref line if Regex::is_match(&size_regex, line) => {
                    let capture = size_regex.captures(line).unwrap();
                    canvas.set_size(
                        capture.at(1).unwrap().parse::<usize>().unwrap(),
                        capture.at(2).unwrap().parse::<usize>().unwrap(),
                    );
                },

                ref line if Regex::is_match(&bounces_regex, line) => {
                    let capture = bounces_regex.captures(line).unwrap();
                    scene_params.max_ray_bounces = capture.at(1).unwrap().parse::<usize>().unwrap();
                },

                ref line if Regex::is_match(&output_regex, line) => {
                    let capture = output_regex.captures(line).unwrap();
                    output_filename = Some(String::from(capture.at(1).unwrap()));
                },

                ref line if Regex::is_match(&camera_regex, line) => {
                    let capture = camera_regex.captures(line).unwrap();
                    let eye_x = capture.at(1).unwrap().parse::<Scalar>().unwrap();
                    let eye_y = capture.at(2).unwrap().parse::<Scalar>().unwrap();
                    let eye_z = capture.at(3).unwrap().parse::<Scalar>().unwrap();

                    let center_x = capture.at(4).unwrap().parse::<Scalar>().unwrap();
                    let center_y = capture.at(5).unwrap().parse::<Scalar>().unwrap();
                    let center_z = capture.at(6).unwrap().parse::<Scalar>().unwrap();

                    let up_x = capture.at(7).unwrap().parse::<Scalar>().unwrap();
                    let up_y = capture.at(8).unwrap().parse::<Scalar>().unwrap();
                    let up_z = capture.at(9).unwrap().parse::<Scalar>().unwrap();

                    let fov = capture.at(10).unwrap().parse::<Scalar>().unwrap();

                    canvas.set_camera_params(
                        Vec3D::new(eye_x, eye_y, eye_z),
                        Vec3D::new(center_x, center_y, center_z),
                        Vec3D::new(up_x, up_y, up_z),
                        fov,
                    );
                },

                ref line if Regex::is_match(&max_vertices_regex, line) => {
                    // do nothing
                },

                ref line if Regex::is_match(&max_vertex_norms_regex, line) => {
                    // do nothing
                },

                ref line if Regex::is_match(&sphere_regex, line) => {
                    let capture = sphere_regex.captures(line).unwrap();

                    let x = capture.at(1).unwrap().parse::<Scalar>().unwrap();
                    let y = capture.at(2).unwrap().parse::<Scalar>().unwrap();
                    let z = capture.at(3).unwrap().parse::<Scalar>().unwrap();

                    let raw_radius = capture.at(4).unwrap().parse::<Scalar>().unwrap();
                    let radius_x = matrix_stack.scale_value().x * raw_radius;
                    let radius_y = matrix_stack.scale_value().y * raw_radius;
                    let radius_z = matrix_stack.scale_value().z * raw_radius;

                    scene_params.objects.push(SceneObject {
                        position: matrix_stack.apply_to(Vec3D::new(x, y, z)),
                        rotation: matrix_stack.rotation(),
                        geometry: SceneGeometry::Ellipse(radius_x, radius_y, radius_z),
                        .. scene_object_prototype
                    });
                },

                ref line if Regex::is_match(&vertex_regex, line) => {
                    let capture = vertex_regex.captures(line).unwrap();

                    let x = capture.at(1).unwrap().parse::<f64>().unwrap();
                    let y = capture.at(2).unwrap().parse::<f64>().unwrap();
                    let z = capture.at(3).unwrap().parse::<f64>().unwrap();

                    vertices.push((x, y, z));
                },

                ref line if Regex::is_match(&vertex_normal_regex, line) => {
                    let capture = vertex_normal_regex.captures(line).unwrap();

                    let x = capture.at(1).unwrap().parse::<f64>().unwrap();
                    let y = capture.at(2).unwrap().parse::<f64>().unwrap();
                    let z = capture.at(3).unwrap().parse::<f64>().unwrap();

                    let nx = capture.at(4).unwrap().parse::<f64>().unwrap();
                    let ny = capture.at(5).unwrap().parse::<f64>().unwrap();
                    let nz = capture.at(6).unwrap().parse::<f64>().unwrap();

                    vertices_with_normals.push(((x, y, z), (nx, ny, nz)));
                },

                ref line if Regex::is_match(&triangle_regex, line) => {
                    let capture = triangle_regex.captures(line).unwrap();

                    let _index_0 = capture.at(1).unwrap().parse::<usize>().unwrap();
                    let _index_1 = capture.at(2).unwrap().parse::<usize>().unwrap();
                    let _index_2 = capture.at(3).unwrap().parse::<usize>().unwrap();

                    println!("WARN:: TRIANGLE NOT COMPLETE");
                },

                ref line if Regex::is_match(&translate_regex, line) => {
                    let capture = translate_regex.captures(line).unwrap();

                    let x = capture.at(1).unwrap().parse::<f64>().unwrap();
                    let y = capture.at(2).unwrap().parse::<f64>().unwrap();
                    let z = capture.at(3).unwrap().parse::<f64>().unwrap();

                    matrix_stack.translate(x, y, z);
                },

                ref line if Regex::is_match(&rotate_regex, line) => {
                    let capture = rotate_regex.captures(line).unwrap();

                    let x = capture.at(1).unwrap().parse::<f64>().unwrap();
                    let y = capture.at(2).unwrap().parse::<f64>().unwrap();
                    let z = capture.at(3).unwrap().parse::<f64>().unwrap();
                    let angle = capture.at(4).unwrap().parse::<f64>().unwrap();

                    matrix_stack.rotate(x, y, z, angle);
                },

                ref line if Regex::is_match(&scale_regex, line) => {
                    let capture = scale_regex.captures(line).unwrap();

                    let x = capture.at(1).unwrap().parse::<f64>().unwrap();
                    let y = capture.at(2).unwrap().parse::<f64>().unwrap();
                    let z = capture.at(3).unwrap().parse::<f64>().unwrap();

                    matrix_stack.scale(x, y, z);
                },

                ref line if Regex::is_match(&push_regex, line) => {
                    matrix_stack.push();
                },

                ref line if Regex::is_match(&pop_regex, line) => {
                    matrix_stack.pop();
                },

                ref line if Regex::is_match(&directional_light_regex, line) => {
                    let capture = directional_light_regex.captures(line).unwrap();

                    let x = capture.at(1).unwrap().parse::<Scalar>().unwrap();
                    let y = capture.at(2).unwrap().parse::<Scalar>().unwrap();
                    let z = capture.at(3).unwrap().parse::<Scalar>().unwrap();
                    let dir = Vec3D::from(matrix_stack.apply_to_direction(Vec3D::new(x, y, z).normalize()));

                    let r = capture.at(4).unwrap().parse::<f32>().unwrap();
                    let g = capture.at(5).unwrap().parse::<f32>().unwrap();
                    let b = capture.at(6).unwrap().parse::<f32>().unwrap();

                    let light = DirectionalLight::default()
                        .with_direction(dir.x, dir.y, dir.z)
                        .with_color(r, g, b);

                    scene_params.directional_lights.push(light);
                },

                ref line if Regex::is_match(&point_light_regex, line) => {
                    let capture = point_light_regex.captures(line).unwrap();

                    let x = capture.at(1).unwrap().parse::<Scalar>().unwrap();
                    let y = capture.at(2).unwrap().parse::<Scalar>().unwrap();
                    let z = capture.at(3).unwrap().parse::<Scalar>().unwrap();

                    let pos = matrix_stack.apply_to(Vec3D::new(x, y, z));

                    let r = capture.at(4).unwrap().parse::<f32>().unwrap();
                    let g = capture.at(5).unwrap().parse::<f32>().unwrap();
                    let b = capture.at(6).unwrap().parse::<f32>().unwrap();

                    let light = PointLight::default()
                        .with_position(pos.x, pos.y, pos.z)
                        .with_color(r, g, b);

                    scene_params.point_lights.push(light);
                },

                ref line if Regex::is_match(&attenuation_regex, line) => {
                    let capture = attenuation_regex.captures(line).unwrap();

                    let constant = capture.at(1).unwrap().parse::<Scalar>().unwrap();
                    let linear = capture.at(2).unwrap().parse::<Scalar>().unwrap();
                    let quadratic = capture.at(3).unwrap().parse::<Scalar>().unwrap();

                    scene_params.linear_attenuation = linear;
                    scene_params.constant_attenuation = constant;
                    scene_params.quadratic_attenuation = quadratic;
                },

                ref line if Regex::is_match(&ambient_regex, line) => {
                    let capture = ambient_regex.captures(line).unwrap();

                    let r = capture.at(1).unwrap().parse::<f32>().unwrap();
                    let g = capture.at(2).unwrap().parse::<f32>().unwrap();
                    let b = capture.at(3).unwrap().parse::<f32>().unwrap();

                    scene_object_prototype.ambient = Color::new(r, g, b);
                },

                ref line if Regex::is_match(&diffuse_regex, line) => {
                    let capture = diffuse_regex.captures(line).unwrap();

                    let r = capture.at(1).unwrap().parse::<f32>().unwrap();
                    let g = capture.at(2).unwrap().parse::<f32>().unwrap();
                    let b = capture.at(3).unwrap().parse::<f32>().unwrap();

                    scene_object_prototype.diffuse = Color::new(r, g, b);
                },

                ref line if Regex::is_match(&specular_regex, line) => {
                    let capture = specular_regex.captures(line).unwrap();

                    let r = capture.at(1).unwrap().parse::<f32>().unwrap();
                    let g = capture.at(2).unwrap().parse::<f32>().unwrap();
                    let b = capture.at(3).unwrap().parse::<f32>().unwrap();

                    scene_object_prototype.specular = Color::new(r, g, b);
                },

                ref line if Regex::is_match(&emission_regex, line) => {
                    let capture = emission_regex.captures(line).unwrap();

                    let r = capture.at(1).unwrap().parse::<f32>().unwrap();
                    let g = capture.at(2).unwrap().parse::<f32>().unwrap();
                    let b = capture.at(3).unwrap().parse::<f32>().unwrap();

                    scene_object_prototype.emission = Color::new(r, g, b);
                },

                ref line if Regex::is_match(&shininess_regex, line) => {
                    let capture = shininess_regex.captures(line).unwrap();

                    scene_object_prototype.shininess = capture.at(1).unwrap().parse::<f32>().unwrap();
                },

                line => {
                    println!("WARN:: UNHANDLED LINE, {}: {}", index, line);
                }
            }
        }

        return Ok(Renderer::new_with_file_output(
            canvas,
            T::from_scene_params(scene_params),
            &output_filename.unwrap(),
        ));
    }
}
