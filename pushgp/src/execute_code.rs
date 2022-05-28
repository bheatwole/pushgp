use crate::code::Extraction;
use crate::*;
use pushgp_macros::*;

pub trait ContextHasCodeStack<L: LiteralEnum<L>> {
    fn code(&self) -> &Stack<Code<L>>;
    fn random_code(&mut self, points: Option<usize>) -> Code<L>;
}

instruction! {
    /// Pushes the result of appending the top two pieces of code. If one of the pieces of code is a single instruction
    /// or literal (that is, something not surrounded by parentheses) then it is surrounded by parentheses first.
    #[stack(Code)]
    fn append(context: &mut Context, src: Code, dst: Code) {
        let src = src.to_list();
        let mut dst = dst.to_list();
        dst.extend_from_slice(&src[..]);
        context.code().push(Code::List(dst));
    }
}

instruction! {
    /// Pushes TRUE onto the BOOLEAN stack if the top piece of code is a single instruction or a literal, and FALSE
    /// otherwise (that is, if it is something surrounded by parentheses). Does not pop the CODE stack
    #[stack(Code)]
    fn atom(context: &mut Context) {
        let c = context.code().peek().unwrap();
        context.bool().push(c.is_atom());
    }
}

instruction! {
    /// Pushes the first item of the list on top of the CODE stack. For example, if the top piece of code is "( A B )"
    /// then this pushes "A" (after popping the argument). If the code on top of the stack is not a list then this has
    /// no effect. The name derives from the similar Lisp function; a more generic name would be "FIRST".
    #[stack(Code)]
    fn car(context: &mut Context, code: Code) {
        context.code().push(match code {
            Code::List(list) => {
                if list.len() > 0 {
                    list[0].clone()
                } else {
                    Code::List(vec![])
                }
            }
            x => x.clone(),
        });
    }
}

instruction! {
    /// Pushes a version of the list from the top of the CODE stack without its first element. For example, if the top
    /// piece of code is "( A B )" then this pushes "( B )" (after popping the argument). If the code on top of the
    /// stack is not a list then this pushes the empty list ("( )"). The name derives from the similar Lisp function; a
    /// more generic name would be "REST".
    #[stack(Code)]
    fn cdr(context: &mut Context, code: Code) {
        context.code().push(match code {
            Code::List(mut list) => {
                if list.len() > 0 {
                    list.remove(0);
                }
                Code::List(list)
            }
            _ => Code::List(vec![]),
        })
    }
}

instruction! {
    /// Pushes the result of "consing" (in the Lisp sense) the second stack item onto the first stack item (which is
    /// coerced to a list if necessary). For example, if the top piece of code is "( A B )" and the second piece of code
    /// is "X" then this pushes "( X A B )" (after popping the argument).
    #[stack(Code)]
    fn cons(context: &mut Context, top: Code, code: Code) {
        context.code().push(match top {
            Code::List(mut list) => {
                list.insert(0, code);
                Code::List(list)
            }
            x => Code::List(vec![code, x]),
        })
    }
}

instruction! {
    /// Pushes the "container" of the second CODE stack item within the first CODE stack item onto the CODE stack. If
    /// second item contains the first anywhere (i.e. in any nested list) then the container is the smallest sub-list
    /// that contains but is not equal to the first instance. For example, if the top piece of code is
    /// "( B ( C ( A ) ) ( D ( A ) ) )" and the second piece of code is "( A )" then this pushes ( C ( A ) ). Pushes an
    /// empty list if there is no such container.
    #[stack(Code)]
    fn container(context: &mut Context, look_for: Code, look_in: Code) {
        if let Some(code) = look_in.container(&look_for) {
            context.code().push(code);
        }
    }
}

instruction! {
    /// Pushes TRUE on the BOOLEAN stack if the second CODE stack item contains the first CODE stack item anywhere
    /// (e.g. in a sub-list).
    #[stack(Code)]
    fn contains(context: &mut Context, look_for: Code, look_in: Code) {
        context.bool().push(look_in.contains(&look_for));
    }
}

instruction! {
    /// Defines the name on top of the NAME stack as an instruction that will push the top item of the CODE stack onto
    /// the EXEC stack.
    #[stack(Code)]
    fn define(context: &mut Context, code: Code, name: Name) {
        context.name().define_name(name, code);
    }
}

instruction! {
    /// Pushes the definition associated with the top NAME on the NAME stack (if any) onto the CODE stack. This extracts
    /// the definition for inspection/manipulation, rather than for immediate execution (although it may then be
    /// executed with a call to CODE.DO or a similar instruction).
    #[stack(Code)]
    fn definition(context: &mut Context, name: Name) {
        if let Some(code) = context.name().definition_for_name(&name) {
            context.code().push(code.clone());
        }
    }
}

instruction! {
    /// Pushes a measure of the discrepancy between the top two CODE stack items onto the INTEGER stack. This will be
    /// zero if the top two items are equivalent, and will be higher the 'more different' the items are from one
    /// another. The calculation is as follows:
    ///   1. Construct a list of all of the unique items in both of the lists (where uniqueness is determined by
    ///      equalp). Sub-lists and atoms all count as items.
    ///   2. Initialize the result to zero.
    ///   3. For each unique item increment the result by the difference between the number of occurrences of the item
    ///      in the two pieces of code.
    ///   4. Push the result.
    #[stack(Code)]
    fn discrepancy(context: &mut Context, a: Code, b: Code) {
        // Determine all the unique code items along with the count that each appears
        let a_items = a.discrepancy_items();
        let b_items = b.discrepancy_items();

        // Count up all the difference from a to b
        let mut discrepancy = 0;
        for (key, &a_count) in a_items.iter() {
            let b_count = *b_items.get(&key).unwrap_or(&0);
            discrepancy += (a_count - b_count).abs();
        }

        // Count up the difference from b to a for only the keys we didn't use already
        for (key, &b_count) in b_items.iter() {
            if a_items.get(&key).is_none() {
                discrepancy += b_count;
            }
        }

        // Push that value
        context.integer().push(discrepancy);
    }
}

instruction! {
    /// An iteration instruction that performs a loop (the body of which is taken from the CODE stack) the number of
    /// times indicated by the INTEGER argument, pushing an index (which runs from zero to one less than the number of
    /// iterations) onto the INTEGER stack prior to each execution of the loop body. This should be implemented as a
    /// macro that expands into a call to CODE.DO*RANGE. CODE.DO*COUNT takes a single INTEGER argument (the number of
    /// times that the loop will be executed) and a single CODE argument (the body of the loop). If the provided INTEGER
    /// argument is negative or zero then this becomes a NOOP. Otherwise it expands into:
    ///   ( 0 <1 - IntegerArg> CODE.QUOTE <CodeArg> CODE.DO*RANGE )
    #[stack(Code)]
    fn do_n_count(context: &mut Context, code: Code, count: Integer) {
        // NOOP if count <= 0
        if count <= 0 {
            // Put the items we popped back to make a NOOP
            context.code().push(code);
            context.integer().push(count);
        } else {
            // Turn into DoNRange with (Count - 1) as destination
            let next = Code::List(vec![
                C::make_literal_integer(0),
                C::make_literal_integer(count - 1),
                Code::instruction("CODE.QUOTE"),
                code,
                Code::instruction("CODE.DONRANGE"),
            ]);
            context.exec().push(next);
        }

    }
}

instruction! {
    /// An iteration instruction that executes the top item on the CODE stack a number of times that depends on the top
    /// two integers, while also pushing the loop counter onto the INTEGER stack for possible access during the
    /// execution of the body of the loop. The top integer is the "destination index" and the second integer is the
    /// "current index." First the code and the integer arguments are saved locally and popped. Then the integers are
    /// compared. If the integers are equal then the current index is pushed onto the INTEGER stack and the code (which
    /// is the "body" of the loop) is pushed onto the EXEC stack for subsequent execution. If the integers are not equal
    /// then the current index will still be pushed onto the INTEGER stack but two items will be pushed onto the EXEC
    /// stack -- first a recursive call to CODE.DO*RANGE (with the same code and destination index, but with a current
    /// index that has been either incremented or decremented by 1 to be closer to the destination index) and then the
    /// body code. Note that the range is inclusive of both endpoints; a call with integer arguments 3 and 5 will cause
    /// its body to be executed 3 times, with the loop counter having the values 3, 4, and 5. Note also that one can
    /// specify a loop that "counts down" by providing a destination index that is less than the specified current index
    #[stack(Code)]
    fn do_n_range(context: &mut Context, code: Code, dest: Integer, cur: Integer) {
        // If we haven't reached the destination yet, push the next iteration onto the stack first.
        if cur != dest {
            let increment = if cur < dest { 1 } else { -1 };
            let next = Code::List(vec![
                C::make_literal_integer(cur + increment),
                C::make_literal_integer(dest),
                Code::instruction("CODE.QUOTE"),
                code.clone(),
                Code::instruction("CODE.DONRANGE"),
            ]);
            context.exec().push(next);
        }

        // Push the current index onto the int stack so its accessible in the loop
        context.integer().push(cur);

        // Push the code to run onto the exec stack
        context.exec().push(code);
    }
}

instruction! {
    /// Like CODE.DO*COUNT but does not push the loop counter. This should be implemented as a macro that expands into
    /// CODE.DO*RANGE, similarly to the implementation of CODE.DO*COUNT, except that a call to INTEGER.POP should be
    /// tacked on to the front of the loop body code in the call to CODE.DO*RANGE. This call to INTEGER.POP will remove
    /// the loop counter, which will have been pushed by CODE.DO*RANGE, prior to the execution of the loop body.
    #[stack(Code)]
    fn do_n_times(context: &mut Context, code: Code, count: Integer) {
        // NOOP if count <= 0
        if count <= 0 {
            context.code().push(code);
            context.integer().push(count);
        } else {
            // The difference between Count and Times is that the 'current index' is not available to
            // the loop body. Pop that value first
            let code = Code::List(vec![Code::instruction("INTEGER.POP"), code]);

            // Turn into DoNRange with (Count - 1) as destination
            let next = Code::List(vec![
                C::make_literal_integer(0),
                C::make_literal_integer(count - 1),
                Code::instruction("CODE.QUOTE"),
                code,
                Code::instruction("CODE.DONRANGE"),
            ]);
            context.exec().push(next);
        }
    }
}

instruction! {
    /// Like CODE.DO but pops the stack before, rather than after, the recursive execution.
    #[stack(Code)]
    fn do_n(context: &mut Context, code: Code) {
        context.exec().push(code)
    }
}

instruction! {
    /// Recursively invokes the interpreter on the program on top of the CODE stack. After evaluation the CODE stack is
    /// popped; normally this pops the program that was just executed, but if the expression itself manipulates the
    /// stack then this final pop may end up popping something else.
    #[stack(Code)]
    fn do(context: &mut Context, code: Code) {
        context.exec().push(Code::instruction("CODE.POP"));
        context.exec().push(code.clone());
        context.code().push(code);
    }
}

instruction! {
    /// Duplicates the top item on the CODE stack. Does not pop its argument (which, if it did, would negate the effect
    /// of the duplication!).
    #[stack(Code)]
    fn dup(context: &mut Context) {
        context.code().duplicate_top_item();
    }
}

instruction! {
    /// Pushes TRUE if the top two pieces of CODE are equal, or FALSE otherwise.
    #[stack(Code)]
    fn equal(context: &mut Context, a: Code, b: Code) {
        context.bool().push(a == b);
    }
}

instruction! {
    /// Pushes the sub-expression of the top item of the CODE stack that is indexed by the top item of the INTEGER
    /// stack. The indexing here counts "points," where each parenthesized expression and each literal/instruction is
    /// considered a point, and it proceeds in depth first order. The entire piece of code is at index 0; if it is a
    /// list then the first item in the list is at index 1, etc. The integer used as the index is taken modulo the
    /// number of points in the overall expression (and its absolute value is taken in case it is negative) to ensure
    /// that it is within the meaningful range.
    #[stack(Code)]
    fn extract(context: &mut Context, code: Code, point: Integer) {
        let total_points = code.points();
        let point = point.abs() % total_points;
        match code.extract_point(point) {
            Extraction::Extracted(code) => context.code().push(code),
            Extraction::Used(_) => {
                panic!("should always be able to extract some code because of abs() and modulo")
            }
        }
    }
}

instruction! {
    /// Empties the CODE stack.
    #[stack(Code)]
    fn flush(context: &mut Context) {
        context.code().clear();
    }
}

instruction! {
    /// Pops the BOOLEAN stack and pushes the popped item (TRUE or FALSE) onto the CODE stack.
    #[stack(Code)]
    fn from_boolean(context: &mut Context, value: Bool) {
        context.code().push(C::make_literal_bool(value));
    }
}

instruction! {
    /// Pops the FLOAT stack and pushes the popped item onto the CODE stack.
    #[stack(Code)]
    fn from_float(context: &mut Context, value: Float) {
        context.code().push(C::make_literal_float(value));
    }
}

instruction! {
    /// Pops the INTEGER stack and pushes the popped integer onto the CODE stack.
    #[stack(Code)]
    fn from_integer(context: &mut Context, value: Integer) {
        context.code().push(C::make_literal_integer(value));
    }
}

instruction! {
    /// Pops the NAME stack and pushes the popped item onto the CODE stack.
    #[stack(Code)]
    fn from_name(context: &mut Context, value: Name) {
        context.code().push(C::make_literal_name(value));
    }
}

instruction! {
    /// If the top item of the BOOLEAN stack is TRUE this recursively executes the second item of the CODE stack;
    /// otherwise it recursively executes the first item of the CODE stack. Either way both elements of the CODE stack
    /// (and the BOOLEAN value upon which the decision was made) are popped.
    #[stack(Code)]
    fn if(context: &mut Context, false_branch: Code, true_branch: Code, switch_on: Bool) {
        context.exec().push(if switch_on { true_branch } else { false_branch });
    }
}

instruction! {
    /// Pushes the result of inserting the second item of the CODE stack into the first item, at the position indexed by
    /// the top item of the INTEGER stack (and replacing whatever was there formerly). The indexing is computed as in
    /// CODE.EXTRACT.
    #[stack(Code)]
    fn insert(context: &mut Context, search_in: Code, replace_with: Code, point: Integer) {
        let total_points = search_in.points();
        let point = point.abs() % total_points;
        context.code().push(search_in.replace_point(point, &replace_with).0);
    }
}

instruction! {
    /// Pushes a list of all active instructions in the interpreter's current configuration.
    #[stack(Code)]
    fn instructions(context: &mut Context) {
        for inst in context.all_instruction_names() {
            context.code().push(Code::instruction(inst));
        }
    }
}

instruction! {
    /// Pushes the length of the top item on the CODE stack onto the INTEGER stack. If the top item is not a list then
    /// this pushes a 1. If the top item is a list then this pushes the number of items in the top level of the list;
    /// that is, nested lists contribute only 1 to this count, no matter what they contain.
    #[stack(Code)]
    fn length(context: &mut Context, code: Code) {
        context.integer().push(code.len() as i64);
    }
}

instruction! {
    /// Pushes a list of the top two items of the CODE stack onto the CODE stack.
    #[stack(Code)]
    fn list(context: &mut Context, a: Code, b: Code) {
        context.code().push(Code::List(vec![b, a]));
    }
}

instruction! {
    /// Pushes TRUE onto the BOOLEAN stack if the second item of the CODE stack is a member of the first item (which is
    /// coerced to a list if necessary). Pushes FALSE onto the BOOLEAN stack otherwise.
    #[stack(Code)]
    fn member(context: &mut Context, look_in: Code, look_for: Code) {
        context.bool().push(look_in.has_member(&look_for));
    }
}

instruction! {
    /// Does nothing.
    #[stack(Code)]
    fn noop(context: &mut Context) {
    }
}

instruction! {
    /// Pushes the nth "CDR" (in the Lisp sense) of the expression on top of the CODE stack (which is coerced to a list
    /// first if necessary). If the expression is an empty list then the result is an empty list. N is taken from the
    /// INTEGER stack and is taken modulo the length of the expression into which it is indexing. A "CDR" of a list is
    /// the list without its first element.
    #[stack(Code)]
    fn nth_cdr(context: &mut Context, index: Integer, list: Code) {
        let index = index.abs() as usize;
        let mut list = list.to_list();
        if 0 == list.len() {
            context.code().push(Code::List(list));
        } else {
            let index = index % list.len();
            list.remove(index);
            context.code().push(Code::List(list));
        }
    }
}

instruction! {
    /// Pushes the nth element of the expression on top of the CODE stack (which is coerced to a list first if
    /// necessary). If the expression is an empty list then the result is an empty list. N is taken from the INTEGER
    /// stack and is taken modulo the length of the expression into which it is indexing.
    #[stack(Code)]
    fn nth(context: &mut Context, index: Integer, list: Code) {
        let index = index.abs() as usize;
        let mut list = list.to_list();
        if 0 == list.len() {
            context.code().push(Code::List(list));
        } else {
            let index = index % list.len();
            list.truncate(index + 1);
            context.code().push(list.pop().unwrap());
        }
    }
}

instruction! {
    /// Pushes TRUE onto the BOOLEAN stack if the top item of the CODE stack is an empty list, or FALSE otherwise.
    #[stack(Code)]
    fn null(context: &mut Context, code: Code) {
        // This relies on the behavior that code.len() returns 1 for atoms
        context.bool().push(0 == code.len());
    }
}

instruction! {
    /// Pops the CODE stack.
    #[stack(Code)]
    fn pop(context: &mut Context, _popped: Code) {
    }
}

instruction! {
    /// Pushes onto the INTEGER stack the position of the second item on the CODE stack within the first item (which is
    /// coerced to a list if necessary). Pushes -1 if no match is found.
    #[stack(Code)]
    fn position(context: &mut Context, look_in: Code, look_for: Code) {
        match look_in.position_of(&look_for) {
            Some(index) => context.integer().push(index as i64),
            None => context.integer().push(-1),
        }
    }
}

instruction! {
    /// Specifies that the next expression submitted for execution will instead be pushed literally onto the CODE stack.
    /// This can be implemented by moving the top item on the EXEC stack onto the CODE stack.
    #[stack(Code)]
    fn quote(context: &mut Context, top_exec: Exec) {
        context.code().push(top_exec);
    }
}

instruction! {
    /// Pushes a newly-generated random program onto the CODE stack. The limit for the size of the expression is taken
    /// from the INTEGER stack; to ensure that it is in the appropriate range this is taken modulo the value of the
    /// MAX-POINTS-IN-RANDOM-EXPRESSIONS parameter and the absolute value of the result is used.
    #[stack(Code)]
    fn rand(context: &mut Context, points: Integer) {
        let code = context.random_code(Some(points as usize));
        context.code().push(code);
    }
}

instruction! {
    /// Rotates the top three items on the CODE stack, pulling the third item out and pushing it on top. This is
    /// equivalent to "2 CODE.YANK".
    #[stack(Code)]
    fn rot(context: &mut Context) {
        context.code().rotate();
    }
}

instruction! {
    /// Inserts the top piece of CODE "deep" in the stack, at the position indexed by the top INTEGER.
    #[stack(Code)]
    fn shove(context: &mut Context, position: Integer) {
        if !context.code().shove(position) {
            context.integer().push(position);
        }
    }
}

instruction! {
    /// Pushes the number of "points" in the top piece of CODE onto the INTEGER stack. Each instruction, literal, and
    /// pair of parentheses counts as a point.
    #[stack(Code)]
    fn size(context: &mut Context, code: Code) {
        context.integer().push(code.points());
    }
}

instruction! {
    /// Pushes the stack depth onto the INTEGER stack.
    #[stack(Code)]
    fn stack_depth(context: &mut Context) {
        context.integer().push(context.code().len() as i64);
    }
}

instruction! {
    /// Pushes the result of substituting the third item on the code stack for the second item in the first item.
    #[stack(Code)]
    fn substitute(context: &mut Context, look_in: Code, look_for: Code, replace_with: Code) {
        context.code().push(look_in.replace(&look_for, &replace_with));
    }
}

instruction! {
    /// Swaps the top two pieces of CODE.
    #[stack(Code)]
    fn swap(context: &mut Context) {
        context.code().swap();
    }
}

instruction! {
    /// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
    /// The index is taken from the INTEGER stack.
    #[stack(Code)]
    fn yank_dup(context: &mut Context, position: Integer) {
        if !context.code().yank_duplicate(position) {
            context.integer().push(position);
        }
    }
}

instruction! {
    /// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the
    /// INTEGER stack.
    #[stack(Code)]
    fn yank(context: &mut Context, position: Integer) {
        if !context.code().yank(position) {
            context.integer().push(position);
        }
    }
}
