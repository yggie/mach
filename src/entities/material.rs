use maths::Matrix;
use shapes::Shape;

#[derive(Clone, Copy, Debug)]
enum MassDefinition {
    ConstantDensity(f32),
    ConstantMass(f32),
}

/// Represents a generic material object. In combination with a `Shape` object,
/// it can be used to compute the final properties of an entity.
#[derive(Clone, Copy, Debug)]
pub struct Material {
    mass_definition: MassDefinition,
    cor: f32,
}

impl Material {
    #[inline]
    fn default() -> Material {
        Material {
            mass_definition: MassDefinition::ConstantMass(1.0),
            cor: 0.9,
        }
    }

    /// Creates a new `Material` object with a constant density.
    pub fn new_with_density(density: f32) -> Material {
        debug_assert!(density > 0.0, "an entity cannot have negative density!");

        Material {
            mass_definition: MassDefinition::ConstantDensity(density),
            .. Material::default()
        }
    }

    /// Creates a new `Material` object with a constant mass.
    pub fn new_with_mass(mass: f32) -> Material {
        debug_assert!(mass > 0.0, "an entity cannot have negative mass!");

        Material {
            mass_definition: MassDefinition::ConstantMass(mass),
            .. Material::default()
        }
    }

    /// Creates a new `Material` from a base `Material` instance with the
    /// coefficient of restitution set to the value specified. This method can
    /// be chained.
    pub fn with_coefficient_of_restitution(self, cor: f32) -> Material {
        Material { cor: cor, .. self }
    }

    /// Computes the mass of a `Shape` if it was made from the `Material`.
    pub fn mass_of(&self, shape: &Shape) -> f32 {
        match self.mass_definition {
            MassDefinition::ConstantDensity(density) => density * shape.volume(),

            MassDefinition::ConstantMass(mass) => mass,
        }
    }

    /// Computes the density of a `Shape` if it was made from the `Material`.
    pub fn density_of(&self, shape: &Shape) -> f32 {
        match self.mass_definition {
            MassDefinition::ConstantDensity(density) => density,

            MassDefinition::ConstantMass(mass) => mass / shape.volume(),
        }
    }

    /// Computes the inertia tensor of a `Shape` if it was made from the
    /// `Material`.
    pub fn inertia_for(&self, shape: &Shape) -> Matrix {
        shape.inertia() * self.mass_of(shape)
    }

    /// Returns the coefficient of restitution associated with the `Material`.
    pub fn coefficient_of_restitution(&self) -> f32 {
        self.cor
    }
}
