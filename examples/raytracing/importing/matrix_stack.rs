use mach::{Scalar, Transform, UnitQuat, UnitVec3D, Vec3D};

pub struct MatrixStack(Vec<(Transform, Vec3D)>);

impl MatrixStack {
    pub fn new() -> MatrixStack {
        MatrixStack(vec!((Transform::identity(), Vec3D::new(1.0, 1.0, 1.0))))
    }

    pub fn translate(&mut self, x: f64, y: f64, z: f64) {
        let top = self.top_mut();

        top.0 = top.0.translate(x as Scalar, y as Scalar, z as Scalar);
    }

    pub fn rotate(&mut self, x: f64, y: f64, z: f64, angle: f64) {
        let top = self.top_mut();

        top.0 = top.0.rotate(Vec3D::new(x as Scalar, y as Scalar, z as Scalar).normalize(), angle as Scalar);
    }

    pub fn scale(&mut self, x: f64, y: f64, z: f64) {
        let top = self.top_mut();

        top.0.translation.x *= x as Scalar;
        top.0.translation.y *= y as Scalar;
        top.0.translation.z *= z as Scalar;

        top.1.x *= x as Scalar;
        top.1.y *= y as Scalar;
        top.1.z *= z as Scalar;
    }

    pub fn rotation(&self) -> UnitQuat {
        self.top().0.rotation()
    }

    pub fn push(&mut self) {
        let top_clone = self.top().clone();
        self.0.push(top_clone);
    }

    pub fn pop(&mut self) {
        self.0.pop().unwrap();
    }

    pub fn scale_value(&self) -> Vec3D {
        self.top().1
    }

    pub fn apply_to(&self, point: Vec3D) -> Vec3D {
        let top = self.top();

        Vec3D::new(point.x * top.1.x, point.y * top.1.y, point.z * top.1.z) +
            top.0.rotation.rotate(point) + top.0.translation
    }

    pub fn apply_to_direction(&self, direction: UnitVec3D) -> UnitVec3D {
        let top = self.top();

        top.0.rotation.rotate(Vec3D::from(direction)).normalize()
    }

    fn top(&self) -> &(Transform, Vec3D) {
        self.0.last().expect("attempted to get the top of an empty matrix stack")
    }

    fn top_mut(&mut self) -> &mut (Transform, Vec3D) {
        self.0.last_mut().expect("attempted to get the top of an empty matrix stack")
    }
}
