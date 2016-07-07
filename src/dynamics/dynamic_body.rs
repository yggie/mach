use dynamics::DynamicBodyExtension;
use collisions::{BodyData, CollisionBody};

pub trait DynamicBody: CollisionBody {
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

impl<E, T> DynamicBody for T where E: 'static, T: CollisionBody<Extension=DynamicBodyExtension<E>> {
    type Extension = E;

    #[inline(always)]
    fn dynamic_extension_data(&self) -> &DynamicBodyExtension<<Self as DynamicBody>::Extension> {
        CollisionBody::extension_data(self)
    }

    #[inline(always)]
    fn dynamic_extension_data_mut(&mut self) -> &mut DynamicBodyExtension<<Self as DynamicBody>::Extension> {
        CollisionBody::extension_data_mut(self)
    }

    #[inline(always)]
    fn split_dynamic_extension_mut(&mut self) -> (&mut BodyData<Self::Narrowphase>, &mut DynamicBodyExtension<<Self as DynamicBody>::Extension>) {
        CollisionBody::split_data_mut(self)
    }
}
