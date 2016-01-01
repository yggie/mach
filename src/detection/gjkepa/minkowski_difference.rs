use maths::Vect;
use entities::Form;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IndexPair(pub usize, pub usize);

#[derive(Clone)]
pub struct MinkowskiDifference<'a>(pub &'a Form, pub &'a Form);

impl<'a> MinkowskiDifference<'a> {
    pub fn vertex(&self, support_point: &IndexPair) -> Vect {
        let shapes = (self.0.shape(), self.1.shape());
        let transforms = (self.0.transform(), self.1.transform());

        return transforms.0.apply_to_point(shapes.0.vertex(support_point.0)) -
            transforms.1.apply_to_point(shapes.1.vertex(support_point.1));
    }

    pub fn support_index_pairs(&self, direction: &Vect) -> Vec<IndexPair> {
        let shapes = (self.0.shape(), self.1.shape());
        let transforms = (self.0.transform(), self.1.transform());

        let direction_in_body_coordinates = (
            transforms.0.apply_inverse_to_direction(*direction),
            transforms.1.apply_inverse_to_direction(-direction),
        );

        let support_point_index_iters = (
            shapes.0.support_indices_for(direction_in_body_coordinates.0),
            shapes.1.support_indices_for(direction_in_body_coordinates.1),
        );

        let mut support_index_pairs = Vec::new();

        for &index_0 in support_point_index_iters.0.iter() {
            for &index_1 in support_point_index_iters.1.iter() {
                support_index_pairs.push(IndexPair(index_0, index_1));
            }
        }

        return support_index_pairs;
    }
}
