use rust_the_forth::core::forth_calculator::ForthCalculator;
const DEFAULT_STACK_SIZE: i16 = 128;

fn create_calculator(stack_size: i16) -> ForthCalculator {
    ForthCalculator::new(stack_size)
}

fn eval_forth_calculator(code: &str, stack_size: i16) -> Vec<i16> {
    let mut calculator = create_calculator(stack_size);
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

#[test]
fn test_boolean_operations() {
    assert_eq!(
        eval_forth_calculator("-1 -1 and", DEFAULT_STACK_SIZE),
        vec![-1]
    );
    assert_eq!(
        eval_forth_calculator("-1 0 and", DEFAULT_STACK_SIZE),
        vec![0]
    );
    assert_eq!(
        eval_forth_calculator("0 0 and", DEFAULT_STACK_SIZE),
        vec![0]
    );

    assert_eq!(eval_forth_calculator("3 3 =", DEFAULT_STACK_SIZE), vec![-1]);
    assert_eq!(eval_forth_calculator("3 4 =", DEFAULT_STACK_SIZE), vec![0]);

    assert_eq!(eval_forth_calculator("2 3 <", DEFAULT_STACK_SIZE), vec![-1]);
    assert_eq!(eval_forth_calculator("3 3 <", DEFAULT_STACK_SIZE), vec![0]);
    assert_eq!(eval_forth_calculator("4 3 <", DEFAULT_STACK_SIZE), vec![0]);

    assert_eq!(eval_forth_calculator("3 2 >", DEFAULT_STACK_SIZE), vec![-1]);
    assert_eq!(eval_forth_calculator("3 3 >", DEFAULT_STACK_SIZE), vec![0]);
    assert_eq!(eval_forth_calculator("2 3 >", DEFAULT_STACK_SIZE), vec![0]);

    assert_eq!(
        eval_forth_calculator("-1 -1 or", DEFAULT_STACK_SIZE),
        vec![-1]
    );
    assert_eq!(
        eval_forth_calculator("-1 0 or", DEFAULT_STACK_SIZE),
        vec![-1]
    );
    assert_eq!(eval_forth_calculator("0 0 or", DEFAULT_STACK_SIZE), vec![0]);

    assert_eq!(eval_forth_calculator("-1 not", DEFAULT_STACK_SIZE), vec![0]);
    assert_eq!(eval_forth_calculator("0 not", DEFAULT_STACK_SIZE), vec![-1]);
}

#[test]
fn test_stack_manipulation_operations() {
    let mut calculator: ForthCalculator = create_calculator(DEFAULT_STACK_SIZE);

    calculator.run("1 2 3 drop".to_string());
    assert_eq!(calculator.get_stack().clone(), vec![1, 2]);

    calculator = create_calculator(DEFAULT_STACK_SIZE);
    calculator.run("1 2 dup".to_string());

    assert_eq!(calculator.get_stack().clone(), vec![1, 2, 2]);

    calculator = create_calculator(DEFAULT_STACK_SIZE);
    calculator.run("1 2 swap".to_string());
    assert_eq!(calculator.get_stack().clone(), vec![2, 1]);

    calculator = create_calculator(DEFAULT_STACK_SIZE);
    calculator.run("1 2 over".to_string());
    assert_eq!(calculator.get_stack().clone(), vec![1, 2, 1]);

    calculator = create_calculator(DEFAULT_STACK_SIZE);
    calculator.run("1 2 3 rot".to_string());
    assert_eq!(calculator.get_stack().clone(), vec![2, 3, 1]);
}

#[test]
fn test_output_operations() {
    let mut calculator: ForthCalculator = create_calculator(DEFAULT_STACK_SIZE);

    calculator.run("42 .".to_string());
    assert_eq!(calculator.get_stack().clone(), vec![]);
    assert_eq!(calculator.get_output().clone(), "42 ");
    calculator = create_calculator(DEFAULT_STACK_SIZE);

    calculator.run("1 2 cr".to_string());
    assert_eq!(calculator.get_stack().clone(), vec![1, 2]);
    assert_eq!(calculator.get_output().clone(), "\n");
    calculator = create_calculator(DEFAULT_STACK_SIZE);

    calculator.run("65 emit".to_string());
    assert_eq!(calculator.get_stack().clone(), vec![]);
    assert_eq!(calculator.get_output().clone(), "A ");
    calculator = create_calculator(DEFAULT_STACK_SIZE);

    calculator.run(".\" Hello, world!\"".to_string());
    assert_eq!(calculator.get_stack().clone(), vec![]);
}
