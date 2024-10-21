use iterative_ml_training::evaluate;

#[test]
fn generate_equations() {
    let result = evaluate("1 + 1");
    assert_eq!(result, "2")
}
