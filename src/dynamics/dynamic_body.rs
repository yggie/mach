use dynamics::DynamicBodyExtension;
use collisions::{BodyData, CollisionObject};

pub trait DynamicBody: CollisionObject {
    type Extension: 'static;

    #[inline(always)]
    fn extension_data(&self) -> &<Self as DynamicBody>::Extension {
        self.dynamic_extension_data().extension_data()
    }

    #[inline(always)]
    fn extension_data_mut(&mut self) -> &mut <Self as DynamicBody>::Extension {
        self.dynamic_extension_data_mut().extension_data_mut()
    }

    fn dynamic_extension_data(&self) -> &DynamicBodyExtension<<Self as DynamicBody>::Extension>;
    fn dynamic_extension_data_mut(&mut self) -> &mut DynamicBodyExtension<<Self as DynamicBody>::Extension>;
    fn split_dynamic_extension_mut(&mut self) -> (&mut BodyData<Self::Narrowphase>, &mut DynamicBodyExtension<<Self as DynamicBody>::Extension>);
}

impl<E, O> DynamicBody for O where E: 'static, O: CollisionObject<Extension=DynamicBodyExtension<E>> {
    type Extension = E;

    #[inline(always)]
    fn dynamic_extension_data(&self) -> &DynamicBodyExtension<<Self as DynamicBody>::Extension> {
        CollisionObject::extension_data(self)
    }

    #[inline(always)]
    fn dynamic_extension_data_mut(&mut self) -> &mut DynamicBodyExtension<<Self as DynamicBody>::Extension> {
        CollisionObject::extension_data_mut(self)
    }

    #[inline(always)]
    fn split_dynamic_extension_mut(&mut self) -> (&mut BodyData<Self::Narrowphase>, &mut DynamicBodyExtension<<Self as DynamicBody>::Extension>) {
        CollisionObject::split_data_mut(self)
    }
}
