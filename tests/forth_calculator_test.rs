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

#[test]
fn test_word_definitions() {
    assert_eq!(
        eval_forth_calculator(": dup-twice dup dup ; 1 dup-twice", DEFAULT_STACK_SIZE),
        vec![1, 1, 1]
    );
    assert_eq!(
        eval_forth_calculator(": countup 1 2 3 ; countup", DEFAULT_STACK_SIZE),
        vec![1, 2, 3]
    );
    assert_eq!(
        eval_forth_calculator(": foo dup ; : foo dup dup ; 1 foo", DEFAULT_STACK_SIZE),
        vec![1, 1, 1]
    );
    assert_eq!(
        eval_forth_calculator(": swap dup ; 1 swap", DEFAULT_STACK_SIZE),
        vec![1, 1]
    );
    assert_eq!(
        eval_forth_calculator(": + * ; 3 4 +", DEFAULT_STACK_SIZE),
        vec![12]
    );
    assert_eq!(
        eval_forth_calculator(
            ": foo 5 ; : bar foo ; : foo 6 ; bar foo",
            DEFAULT_STACK_SIZE
        ),
        vec![5, 6]
    );
    assert_eq!(
        eval_forth_calculator(": foo 10 ; : foo foo 1 + ; foo", DEFAULT_STACK_SIZE),
        vec![11]
    );
}

#[test]
fn test_case_insensitive() {
        assert_eq!(
        eval_forth_calculator("1 DUP Dup dup", DEFAULT_STACK_SIZE),
        vec![1, 1, 1, 1]
    );
    assert_eq!(
        eval_forth_calculator("1 2 3 4 DROP Drop drop", DEFAULT_STACK_SIZE),
        vec![1]
    );
    assert_eq!(
        eval_forth_calculator("1 2 SWAP 3 Swap 4 swap", DEFAULT_STACK_SIZE),
        vec![2, 3, 4, 1]
    );
    assert_eq!(
        eval_forth_calculator("1 2 OVER Over over", DEFAULT_STACK_SIZE),
        vec![1, 2, 1, 2, 1]
    );
    assert_eq!(
        eval_forth_calculator(": foo dup ; 1 FOO Foo foo", DEFAULT_STACK_SIZE),
        vec![1, 1, 1, 1]
    );
    assert_eq!(
        eval_forth_calculator(": SWAP DUP Dup dup ; 1 swap", DEFAULT_STACK_SIZE),
        vec![1, 1, 1, 1]
    );
}

#[test]
fn test_invalid_words() {
    let mut calc = create_calculator(DEFAULT_STACK_SIZE);
    calc.run(": 1 2 ;".to_string());
    assert_eq!(calc.get_output(), "invalid-word\n");
    assert!(calc.get_stack().is_empty());

    let mut calc = create_calculator(DEFAULT_STACK_SIZE);
    calc.run(": -1 2 ;".to_string());
    assert_eq!(calc.get_output(), "invalid-word\n");
    assert!(calc.get_stack().is_empty());

    let mut calc = create_calculator(DEFAULT_STACK_SIZE);
    calc.run("foo".to_string());
    assert_eq!(calc.get_output(), "?\n");
    assert!(calc.get_stack().is_empty());
}

#[test]
fn test_non_cloning_word_definition() {
    let code = "
        : word1 1 ;
        : word2 word1 word1 ;
        : word4 word2 word2 ;
        : word8 word4 word4 ;
        : word16 word8 word8 ;
        : word32 word16 word16 ;
        : word64 word32 word32 ;
        : word128 word64 word64 ;
        : word256 word128 word128 ;
        : word512 word256 word256 ;
        : word1024 word512 word512 ;
        : word2048 word1024 word1024 ;
        : word4096 word2048 word2048 ;
        : word8192 word4096 word4096 ;
        : word16384 word8192 word8192 ;
        : word32768 word16384 word16384 ;
        : word65536 word32768 word32768 ;
        : word131072 word65536 word65536 ;
        : word262144 word131072 word131072 ;
        : word524288 word262144 word262144 ;
        : word1048576 word524288 word524288 ;
        : word2097152 word1048576 word1048576 ;
        : word4194304 word2097152 word2097152 ;
        : word8388608 word4194304 word4194304 ;
        : word16777216 word8388608 word8388608 ;
        : word33554432 word16777216 word16777216 ;
        : word67108864 word33554432 word33554432 ;
        : word134217728 word67108864 word67108864 ;
    ";
    let result = eval_forth_calculator(code, DEFAULT_STACK_SIZE);
    assert_eq!(result, vec![]);
}


#[test]
fn test_unit_computation_1() {
    let code = ": meter 100 * ; : decimeter 10 * ; : centimeter 1 * ; 1 meter 5 decimeter 2 centimeter + +";
    let result = eval_forth_calculator(code, DEFAULT_STACK_SIZE);
    assert_eq!(result, vec![152]);    
}

#[test]
fn test_unit_computation_2() {
    let code = ": seconds 1 * ; : minutes 60 * seconds ; : hours 60 * minutes ; 2 hours 13 minutes 5 seconds + +";
    let result = eval_forth_calculator(code, DEFAULT_STACK_SIZE);
    assert_eq!(result, vec![7985]);
}

#[test]
fn test_constant_summation() {

    let code = ": one1 1 ; : one2  one1 one1 ; : one4  one2 one2 ; : one8  one4 one4 ; : one16 one8 one8 ; \
         : add1 + ; : add2  add1 add1 ; : add4  add2 add2 ; : add8  add4 add4 ; : add16 add8 add8 ; \
         0 one16 add16";
    let result = eval_forth_calculator(code, DEFAULT_STACK_SIZE);
    assert_eq!(result, vec![16]);    
}

#[test]
fn test_linear_summation() {
    let code = ": next1 dup 1 + ; : next2  next1 next1 ; : next4  next2 next2 ; : next8  next4 next4 ; : next16 next8 next8 ; \
         : add1 + ; : add2  add1 add1 ; : add4  add2 add2 ; : add8  add4 add4 ; : add16 add8 add8 ; \
         0 next16 add16";
    let result = eval_forth_calculator(code, DEFAULT_STACK_SIZE);
    assert_eq!(result, vec![136]);
}

#[test]
fn test_geometric_summation() {

    let code = ": next1 dup 2 * ; : next2  next1 next1 ; : next4  next2 next2 ; : next8  next4 next4 ; \
         : add1 + ; : add2  add1 add1 ; : add4  add2 add2 ; : add8  add4 add4 ; \
         1 next8 add8";
    let result = eval_forth_calculator(code, DEFAULT_STACK_SIZE);
    assert_eq!(result, vec![511]);
}

#[test]
fn test_power_of_2() {

    let code = ": next1 dup 2 * ; : next2  next1 next1 ; : next4  next2 next2 ; \
         : mul1 * ; : mul2  mul1 mul1 ; : mul4  mul2 mul2 ; \
         1 next4 mul4";
    let result = eval_forth_calculator(code, DEFAULT_STACK_SIZE);
    assert_eq!(result, vec![1024]);    
}

