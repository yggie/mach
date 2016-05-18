use mach::Scalar;

#[derive(Clone, Debug)]
pub enum SceneGeometry {
    Ellipse(Scalar, Scalar, Scalar),
}
