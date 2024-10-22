use iterative_ml_training::evaluate;
use rand::prelude::*;

#[test]
fn generate_addition_equation() {
    let left: i8 = random();
    let right: i8 = random();
    let expected = left + right;

    let candidate = format!("{} + {}", left, right);

    println!("Evaluating {}", candidate);
    let result = evaluate(&candidate);
    assert_eq!(expected.to_string(), result)
}
