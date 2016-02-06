use mach::maths::SparseMatrix;

#[test]
fn it_zeroes_all_entries_by_default() {
    let matrix = SparseMatrix::new(2);

    assert_eq!(matrix[(0, 0)], 0.0);
    assert_eq!(matrix[(0, 1)], 0.0);
    assert_eq!(matrix[(1, 0)], 0.0);
    assert_eq!(matrix[(1, 1)], 0.0);
}
