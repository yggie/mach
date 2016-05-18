use tests::support::{Action, Property, PropertyCheck};

macro_rules! assert_properties {
    (target: $target:expr, actions: $actions:expr, properties: $properties:expr,) => {
        assert_properties_for_actions($target, $actions, $properties);
    };
}

pub fn assert_properties_for_actions<A, T>(mut target: T, actions: Vec<A>, properties: &[Box<Property<T, Action=A>>]) where A: Action<T> {
    let mut action_history = Vec::with_capacity(actions.len());
    for (index, mut action) in actions.into_iter().enumerate() {
        let property_checks: Vec<Box<PropertyCheck<T>>> = properties.iter()
            .filter_map(|factory| factory.property_check_for(&target, &action))
            .collect();

        action.perform(&mut target);

        let errors: Vec<String> = property_checks.into_iter()
            .map(|property_check| property_check.assert(&target))
            .filter_map(|result| result.err())
            .collect();

        if !errors.is_empty() {
            let actions_up_to_this_point: Vec<A> = action_history.iter()
                .take(index + 1)
                .cloned()
                .collect();

            let num_errors = errors.len();
            let mut formatted_errors: Vec<String> = errors.into_iter()
                .enumerate()
                .map(|(index, string)| format!("{}| \"{}\"", index + 1, string))
                .collect();

            formatted_errors.insert(0, format!("Action sequence: {:?}", actions_up_to_this_point));
            formatted_errors.insert(1, format!("Found {} property violations:", num_errors));

            panic!(formatted_errors.join("\n"));
        }

        action_history.push(action);
    }
}
