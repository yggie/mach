assert_entity_store_behaviour! {
    use entities::MachStore;

    pub fn test_subject() -> MachStore {
        MachStore::new()
    }
}
