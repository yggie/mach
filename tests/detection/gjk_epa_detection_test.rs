assert_detection_behaviour! {
    use detection::GjkEpaDetection;

    pub fn test_subject() -> GjkEpaDetection {
        GjkEpaDetection::new()
    }
}
