assert_detection_behaviour! {
    use entities::EntityStore;
    use detection::GjkEpaDetection;

    pub fn test_subject<ES: EntityStore>(_store: &ES) -> GjkEpaDetection<ES> {
        GjkEpaDetection::new()
    }
}
