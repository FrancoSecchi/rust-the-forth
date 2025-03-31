use rust_the_forth::core::operation::get_operations;
use rust_the_forth::core::error::OperationError;

#[test]
fn test_complex_sequence() {
    let ops = get_operations();
    let mut stack = vec![10, 5, 3, 4, 2]; 
    
    ops["*"].apply(&mut stack).unwrap(); 
    assert_eq!(stack, vec![10, 5, 3, 8]);
    
    ops["-"].apply(&mut stack).unwrap();  
    assert_eq!(stack, vec![10, 5, -5]);
    
    ops["/"].apply(&mut stack).unwrap();  
    assert_eq!(stack, vec![10, -1]);
    
    ops["-"].apply(&mut stack).unwrap(); 
    assert_eq!(stack, vec![11]);
}

#[test]
fn test_error_handling() {
    let ops = get_operations();
    let mut stack = vec![1, 0];
    
    assert!(matches!(
        ops["/"].apply(&mut stack),
        Err(OperationError::DivisionByZero)
    ));
        
    assert_eq!(stack, vec![]);
}

#[test]
fn test_complex_sequence_underflow() {            
    let ops = get_operations();

    let mut stack = vec![10, 5, 3, 4, 2];
    
    ops["*"].apply(&mut stack).unwrap(); 
    assert_eq!(stack, vec![10, 5, 3, 8]);

    ops["-"].apply(&mut stack).unwrap();  
    assert_eq!(stack, vec![10, 5, -5]);    
    
    ops["/"].apply(&mut stack).unwrap(); 
    assert_eq!(stack, vec![10, -1]);

    ops["-"].apply(&mut stack).unwrap();
    assert_eq!(stack, vec![11]); 

    assert!(matches!(
        ops["/"].apply(&mut stack),
        Err(OperationError::StackUnderflow)
    ));
}

