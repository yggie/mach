use core::Float;
use maths::Matrix;
use shapes::Shape;

#[derive(Clone, Copy, Debug)]
enum MassDefinition {
    ConstantDensity(Float),
    ConstantMass(Float),
}

/// Represents a generic material object. In combination with a `Shape` object,
/// it can be used to compute the final properties of an entity.
#[derive(Clone, Copy, Debug)]
pub struct Material {
    _mass_definition: MassDefinition,
    cor: Float,
    _friction_coefficient: Float,
}

impl Material {
    #[inline]
    fn default() -> Material {
        Material {
            _mass_definition: MassDefinition::ConstantMass(1.0),
            cor: 0.9,
            _friction_coefficient: 0.6,
        }
    }

    /// Creates a new `Material` object with a constant density.
    pub fn new_with_density(density: Float) -> Material {
        debug_assert!(density > 0.0, "an entity cannot have negative density!");

        Material {
            _mass_definition: MassDefinition::ConstantDensity(density),
            .. Material::default()
        }
    }

    /// Creates a new `Material` object with a constant mass.
    pub fn new_with_mass(mass: Float) -> Material {
        debug_assert!(mass > 0.0, "an entity cannot have negative mass!");

        Material {
            _mass_definition: MassDefinition::ConstantMass(mass),
            .. Material::default()
        }
    }

    /// Creates a new `Material` from a base `Material` instance with the
    /// coefficient of restitution set to the value specified. This method can
    /// be chained.
    pub fn with_coefficient_of_restitution(self, cor: Float) -> Material {
        Material { cor: cor, .. self }
    }

    /// Creates a new `Material` from a base `Material` instance with the
    /// coefficient of friction set to the value specified. This method can
    /// be chained.
    pub fn with_friction_coefficient(self, friction_coefficient: Float) -> Material {
        Material { _friction_coefficient: friction_coefficient, .. self }
    }

    /// Computes the mass of a `Shape` if it was made from the `Material`.
    pub fn mass_of(&self, shape: &Shape) -> Float {
        match self._mass_definition {
            MassDefinition::ConstantDensity(density) => density * shape.volume(),

            MassDefinition::ConstantMass(mass) => mass,
        }
    }

    /// Computes the density of a `Shape` if it was made from the `Material`.
    pub fn density_of(&self, shape: &Shape) -> Float {
        match self._mass_definition {
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
    pub fn coefficient_of_restitution(&self) -> Float {
        self.cor
    }

    /// Returns the friction coefficient associated with the `Material`.
    pub fn friction_coefficient(&self) -> Float {
        self._friction_coefficient
    }
}
