/// Takes a stack index which is a zero-based index from the end of a Vec and translates it into a Vec index (zero-based
/// from the front). Additionally, if the stack_index is larger than the vec_len, the modulo is used so that it is
/// always a valid result
///
/// stack index: [ 4, 3, 2, 1, 0 ]
///   vec index: [ 0, 1, 2, 3, 4 ]
pub fn stack_to_vec(mut stack_index: i64, vec_len: usize) -> usize {
    assert!(vec_len > 0);

    // If the stack index is negative, add however many times the vec_len it takes to make it positive again
    if stack_index < 0 {
        let times = (stack_index / vec_len as i64).saturating_abs().saturating_add(1);
        stack_index += (vec_len as i64).saturating_mul(times);
    }

    // Now that we know stack_index is positive, cast it to the same type as the output to make life easier
    let mut stack_index = stack_index as usize;

    // Move the stack_index into the range 0..vec_len
    stack_index = stack_index % vec_len;

    // The actual vec index is the reverse of the stack index
    (vec_len - 1) - stack_index
}

#[cfg(test)]
mod tests {
    use crate::util::stack_to_vec;

    #[test]
    fn test_stack_to_vec() {
        // Zero always for the index == len - 1
        assert_eq!(0, stack_to_vec(0, 1));
        assert_eq!(0, stack_to_vec(4, 5));
        assert_eq!(0, stack_to_vec(1999, 2000));

        // len-1 always for index 0
        assert_eq!(0, stack_to_vec(0, 1));
        assert_eq!(4, stack_to_vec(0, 5));
        assert_eq!(1999, stack_to_vec(0, 2000));

        // When len == index, the algorithm wraps around and you get the top of the stack again
        assert_eq!(0, stack_to_vec(1, 1));
        assert_eq!(4, stack_to_vec(5, 5));
        assert_eq!(1999, stack_to_vec(2000, 2000));

        // Going negative should not break the cycle
        assert_eq!(stack_to_vec(5, 5), 4);
        assert_eq!(stack_to_vec(4, 5), 0);
        assert_eq!(stack_to_vec(3, 5), 1);
        assert_eq!(stack_to_vec(2, 5), 2);
        assert_eq!(stack_to_vec(1, 5), 3);
        assert_eq!(stack_to_vec(0, 5), 4);
        assert_eq!(stack_to_vec(-1, 5), 0);
        assert_eq!(stack_to_vec(-2, 5), 1);
        assert_eq!(stack_to_vec(-3, 5), 2);
        assert_eq!(stack_to_vec(-4, 5), 3);
        assert_eq!(stack_to_vec(-5, 5), 4);
    }

    #[test]
    fn test_overflows() {
        // At one point gave 'attempt to multiply with overflow'
        assert_eq!(stack_to_vec(i64::MIN + 2, 2), 0);

        // At one point gave 'attempt to add with overflow'
        assert_eq!(stack_to_vec(i64::MIN + 1, 1), 0);
    }
}
