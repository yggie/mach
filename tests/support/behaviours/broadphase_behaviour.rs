macro_rules! assert_broadphase_behaviour {
    { $( $lines:item )+ } => {
        $( $lines )+

        mod broadphase_behaviour {
            use super::test_subject;

            use ID;
            use utils::EntityBuilder;
            use entities::{EntityStore, MachStore};
            use broadphase::Broadphase;

            fn validate<B: Broadphase>(input: B) -> B {
                input
            }

            #[test]
            fn it_generates_contact_candidates_for_intersecting_rigid_bodies() {
                let mut store = MachStore::new();
                let mut broadphase = validate(test_subject(&store));
                let template = EntityBuilder::from_store(&mut store)
                    .as_cube(1.0)
                    .with_translation(0.0, 0.0, 0.0);

                let number_of_contact_candidate_pairs = template.clone()
                    .create_rigid_body_and_notify_then_count(&mut broadphase);
                assert_eq!(number_of_contact_candidate_pairs, 0);

                let number_of_contact_candidate_pairs = template.clone()
                    .create_rigid_body_and_notify_then_count(&mut broadphase);
                assert_eq!(number_of_contact_candidate_pairs, 1);

                let number_of_contact_candidate_pairs = template.clone()
                    .create_rigid_body_and_notify_then_count(&mut broadphase);
                assert_eq!(number_of_contact_candidate_pairs, 3);

                let number_of_contact_candidate_pairs = template.clone()
                    .create_rigid_body_and_notify_then_count(&mut broadphase);
                assert_eq!(number_of_contact_candidate_pairs, 6);
            }

            #[test]
            fn it_does_not_generate_contact_candidates_for_intersecting_static_bodies() {
                let mut store = MachStore::new();
                let mut broadphase = validate(test_subject(&store));
                let template = EntityBuilder::from_store(&mut store)
                    .as_cube(1.0)
                    .with_translation(0.0, 0.0, 0.0);

                let number_of_contact_candidate_pairs = template.clone()
                    .create_static_body_and_notify_then_count(&mut broadphase);
                assert_eq!(number_of_contact_candidate_pairs, 0);

                let number_of_contact_candidate_pairs = template.clone()
                    .create_static_body_and_notify_then_count(&mut broadphase);
                assert_eq!(number_of_contact_candidate_pairs, 0);

                let number_of_contact_candidate_pairs = template.clone()
                    .create_rigid_body_and_notify_then_count(&mut broadphase);
                assert_eq!(number_of_contact_candidate_pairs, 2);
            }

            trait EntityBuilderExtension {
                type EntityStore: EntityStore;

                fn create_rigid_body_and_notify<B: Broadphase<EntityStore=Self::EntityStore>>(self, broadphase: &mut B) -> ID;
                fn create_rigid_body_and_notify_then_count<B: Broadphase<EntityStore=Self::EntityStore>>(self, broadphase: &mut B) -> usize;
                fn create_static_body_and_notify<B: Broadphase<EntityStore=Self::EntityStore>>(self, broadphase: &mut B) -> ID;
                fn create_static_body_and_notify_then_count<B: Broadphase<EntityStore=Self::EntityStore>>(self, broadphase: &mut B) -> usize;
            }

            impl<'a, ES> EntityBuilderExtension for EntityBuilder<'a, ES> where ES: EntityStore {
                type EntityStore = ES;

                fn create_rigid_body_and_notify<B: Broadphase<EntityStore=Self::EntityStore>>(self, broadphase: &mut B) -> ID {
                    let id = self.clone().create_rigid_body();
                    let store = &**self.entity_store();

                    broadphase.notify_body_created(store, &**store.find_body(id).unwrap());

                    return id;
                }

                fn create_rigid_body_and_notify_then_count<B: Broadphase<EntityStore=Self::EntityStore>>(self, broadphase: &mut B) -> usize {
                    let _id = self.clone().create_rigid_body_and_notify(broadphase);
                    let store = &**self.entity_store();

                    return broadphase.contact_candidate_pairs_iter(store).count();
                }

                fn create_static_body_and_notify<B: Broadphase<EntityStore=Self::EntityStore>>(self, broadphase: &mut B) -> ID {
                    let id = self.clone().create_static_body();
                    let store = &**self.entity_store();

                    broadphase.notify_body_created(store, &**store.find_body(id).unwrap());

                    return id;
                }

                fn create_static_body_and_notify_then_count<B: Broadphase<EntityStore=Self::EntityStore>>(self, broadphase: &mut B) -> usize {
                    let _id = self.clone().create_static_body_and_notify(broadphase);
                    let store = &**self.entity_store();

                    return broadphase.contact_candidate_pairs_iter(store).count();
                }
            }
        }
    };
}
