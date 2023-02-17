use elvis_op::elvis;

#[test]
fn elvis() {
    let some = Some(114);
    let a: i32 = elvis!(None ?: None ?: some ?: None ?: return);
    assert_eq!(a, 114);
}