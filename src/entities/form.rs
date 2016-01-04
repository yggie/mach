use maths::{Transform, Quat, Vect};
use shapes::Shape;

pub struct Form {
    shape: Box<Shape>,
    transform: Transform,
}

impl Form {
    pub fn new(shape: Box<Shape>, transform: Transform) -> Form {
        Form {
            shape: shape,
            transform: transform,
        }
    }

    #[inline]
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    #[inline]
    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    #[inline]
    pub fn shape(&self) -> &Shape {
        &*self.shape
    }

    #[inline]
    pub fn translation(&self) -> &Vect {
        &self.transform.translation
    }

    #[inline]
    pub fn translation_mut(&mut self) -> &mut Vect {
        &mut self.transform.translation
    }

    #[inline]
    pub fn rotation(&self) -> &Quat {
        &self.transform.rotation
    }

    #[inline]
    pub fn rotation_mut(&mut self) -> &mut Quat {
        &mut self.transform.rotation
    }

    #[inline]
    pub fn vertex(&self, index: usize) -> Vect {
        self.transform().apply_to_point(self.shape().vertex(index))
    }

    pub fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vect> + 'a> {
        let transform = self.transform.clone();
        let iterator = self.shape.vertices_iter()
            .map(move |v| transform.apply_to_point(v));

        return Box::new(iterator);
    }
}

#[macro_export]
macro_rules! form_field_accessors {
    (field_name: $field_name:ident) => {
        #[inline]
        pub fn form(&self) -> &Form {
            &self.$field_name
        }

        #[inline]
        pub fn form_mut(&mut self) -> &mut Form {
            &mut self.$field_name
        }

        #[inline]
        pub fn transform(&self) -> &Transform {
            self.$field_name.transform()
        }

        #[inline]
        pub fn transform_mut(&mut self) -> &mut Transform {
            self.$field_name.transform_mut()
        }

        #[inline]
        pub fn shape(&self) -> &Shape {
            self.$field_name.shape()
        }

        #[inline]
        pub fn translation(&self) -> &Vect {
            self.$field_name.translation()
        }

        #[inline]
        pub fn translation_mut(&mut self) -> &mut Vect {
            self.$field_name.translation_mut()
        }

        #[inline]
        pub fn rotation(&self) -> &Quat {
            self.$field_name.rotation()
        }

        #[inline]
        pub fn rotation_mut(&mut self) -> &mut Quat {
            self.$field_name.rotation_mut()
        }

        #[inline]
        pub fn vertex(&self, index: usize) -> Vect {
            self.$field_name.vertex(index)
        }

        #[inline]
        pub fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vect> + 'a> {
            self.$field_name.vertices_iter()
        }
    };
}