use rust_the_forth::core::forth_calculator::ForthCalculator;
const DEFAULT_STACK_SIZE: i16 = 128;

fn eval_forth_calculator(code: &str, stack_size: i16) -> Vec<i16> {
    let mut calculator = ForthCalculator::new(stack_size);
    calculator.run(code.to_string());
    calculator.get_stack().clone()
}

#[test]
fn test_positive_numbers() {
    let stack = eval_forth_calculator("1 2 3 4 5", DEFAULT_STACK_SIZE);
    assert_eq!(stack, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_negative_numbers() {
    let stack = eval_forth_calculator("-1 -2 -3 -4 -5", DEFAULT_STACK_SIZE);
    assert_eq!(stack, vec![-1, -2, -3, -4, -5]);
}

#[test]
fn test_add_operations() {
    assert_eq!(eval_forth_calculator("1 2 +", DEFAULT_STACK_SIZE), vec![3]);
    assert_eq!(
        eval_forth_calculator("1 2 3 +", DEFAULT_STACK_SIZE),
        vec![1, 5]
    );
}

#[test]
fn test_sub_operations() {
    assert_eq!(eval_forth_calculator("3 4 -", DEFAULT_STACK_SIZE), vec![-1]);
    assert_eq!(
        eval_forth_calculator("1 12 3 -", DEFAULT_STACK_SIZE),
        vec![1, 9]
    );
}

#[test]
fn test_mul_operations() {
    assert_eq!(eval_forth_calculator("2 4 *", DEFAULT_STACK_SIZE), vec![8]);
    assert_eq!(
        eval_forth_calculator("1 2 3 *", DEFAULT_STACK_SIZE),
        vec![1, 6]
    );
}

#[test]
fn test_div_operations() {
    assert_eq!(eval_forth_calculator("12 3 /", DEFAULT_STACK_SIZE), vec![4]);
    assert_eq!(eval_forth_calculator("8 3 /", DEFAULT_STACK_SIZE), vec![2]);
    assert_eq!(
        eval_forth_calculator("1 12 3 /", DEFAULT_STACK_SIZE),
        vec![1, 4]
    );
}

#[test]
fn test_mixed_operations() {
    assert_eq!(
        eval_forth_calculator("1 2 + 4 -", DEFAULT_STACK_SIZE),
        vec![-1]
    );
    assert_eq!(
        eval_forth_calculator("2 4 * 3 /", DEFAULT_STACK_SIZE),
        vec![2]
    );
    assert_eq!(
        eval_forth_calculator("1 3 4 * +", DEFAULT_STACK_SIZE),
        vec![13]
    );
    assert_eq!(
        eval_forth_calculator("1 3 4 + *", DEFAULT_STACK_SIZE),
        vec![7]
    );
}
