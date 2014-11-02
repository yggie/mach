use properties::Rigid;
use shapes::Sphere;
use core::Database;

#[test]
fn new_test() {
    let db = Database::new();

    assert!(db.size() == 0);
}

#[test]
fn single_body_test() {
    let s = Sphere::new(2.1);
    let p = Rigid::new(3.0);
    let mut database = Database::new();

    let uid = database.create_body_stationary(s, p);

    // should increase the size by one
    assert!(database.size() == 1);

    // should be able to find the created Body by it’s UID
    match database.find(uid) {
        None => panic!("Could not find Body which was just created!"),
        Some(_) => (),
    }
}
