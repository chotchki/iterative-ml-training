use crate::content_generators::training::Training;
use rand::prelude::*;

fn generate_addition_equation() -> Training {
    let left: i8 = random();
    let right: i8 = random();
    let expected = left + right;

    let candidate = format!("{} + {}", left, right);

    Training {
        input: candidate,
        output: expected.to_string(),
    }
}

fn generate_subtraction_equation() -> Training {
    let left: i8 = random();
    let right: i8 = random();
    let expected = left - right;

    let candidate = format!("{} - {}", left, right);

    Training {
        input: candidate,
        output: expected.to_string(),
    }
}
