use crate::*;

#[derive(Debug)]
pub struct PushList {}

impl StaticName for PushList {
    fn static_name() -> &'static str {
        "__PUSH.LIST"
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveExec<Vm>> Instruction<Vm> for PushList {
    // The PushList cannot be parsed this way because it requires recursive parsing (and thus access to the parser). See
    // parse.rs for the implementation of recursive parsing
    fn parse<'a>(input: &'a str, _opcode: u32) -> nom::IResult<&'a str, Code> {
        Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::Verify)))
    }

    fn fmt(f: &mut std::fmt::Formatter<'_>, code: &Code, vtable: &InstructionTable<Vm>) -> std::fmt::Result {
        write!(f, "(")?;
        if let Some(iter) = code.get_data().code_iter() {
            for c in iter {
                vtable.fmt(f, c)?;
            }
        } else {
            panic!("fmt called for PushList with data that is not a CodeList")
        }
        write!(f, " )")
    }

    // A PushList should typically have its weight set to zero and never called for a random value. The tree of
    // Code values is created in the random code generation.
    fn random_value(_engine: &mut VirtualMachineEngine<Vm>) -> Code {
        Code::new(0, Data::CodeList(vec![]))
    }
    
    fn execute(mut code: Code, vm: &mut Vm) {
        match code.get_data_mut() {
            Data::CodeList(list) => for item in list.drain(..) {
                vm.exec().push(item);
            }
            _ => panic!("execute called for PushList with data that is not a CodeList"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Extraction;
    use crate::*;

    fn new_base_vm() -> BaseVm {
        let mut vm = BaseVm::new(None, Configuration::new_simple());
        add_base_instructions(&mut vm);
        add_base_literals(&mut vm);

        vm
    }

    #[test]
    fn not_parsable() {
        let vm = new_base_vm();
        let result = vm.engine().parse("( DOESNT WORK");
        assert!(result.is_err());
    }

    // #[test]
    // fn code_display() {
    //     let code = Code::new_list(vec![]);
    //     assert_eq!("( )", format!("{}", code));

    //     let vm = new_base_vm();
    //     let (_, code) = vm.engine().parse("( ( TRUE 0.012345 -12784 a_name ) BOOL.AND )").unwrap();
    //     assert_eq!("( ( TRUE 0.012345 -12784 a_name ) BOOL.AND )", format!("{}", code));
    // }

    #[test]
    fn code_points() {
        let vm = new_base_vm();
        let (_, code) = vm.engine().parse("( ( TRUE 0.012345 -12784 a_name ) BOOL.AND )").unwrap();
        assert_eq!(7, code.points());
    }

    #[test]
    fn extract_point() {
        let vm = new_base_vm();
        let (_, code) = vm.engine().parse("( A ( B ) )").unwrap();
        assert_eq!(4, code.points());
        assert_eq!(code.extract_point(0), Extraction::Extracted(code.clone()));
        assert_eq!(code.extract_point(1), Extraction::Extracted(vm.engine().must_parse("A")));
        assert_eq!(code.extract_point(2), Extraction::Extracted(vm.engine().must_parse("( B )")));
        assert_eq!(code.extract_point(3), Extraction::Extracted(vm.engine().must_parse("B")));
    }

    #[test]
    fn replace_point() {
        let vm = new_base_vm();
        let (_, code) = vm.engine().parse("( A ( B ) )").unwrap();
        let replace_with = vm.engine().must_parse("C");
        assert_eq!(&code.replace_point(0, &replace_with).0, &vm.engine().must_parse("C"));
        assert_eq!(&code.replace_point(1, &replace_with).0, &vm.engine().must_parse("( C ( B ) )"));
        assert_eq!(&code.replace_point(2, &replace_with).0, &vm.engine().must_parse("( A C )"));
        assert_eq!(&code.replace_point(3, &replace_with).0, &vm.engine().must_parse("( A ( C ) )"));
        assert_eq!(&code.replace_point(4, &replace_with).0, &vm.engine().must_parse("( A ( B ) )"));
    }

    #[test]
    fn extract_names() {
        let vm = new_base_vm();
        let code = vm.engine().must_parse("( ANAME ( 1 TRUE ANAME ) BNAME ( ( CNAME ANAME ) ) )");
        let names = code.extract_names();
        assert_eq!(5, names.len());
        assert_eq!(Name::from("ANAME"), names[0]);
        assert_eq!(Name::from("ANAME"), names[1]);
        assert_eq!(Name::from("BNAME"), names[2]);
        assert_eq!(Name::from("CNAME"), names[3]);
        assert_eq!(Name::from("ANAME"), names[4]);
    }

    #[test]
    fn extract_atoms() {
        let vm = new_base_vm();
        let (_, code) = vm.engine().parse("( ( TRUE 0.012345 -12784 a_name ) BOOL.AND )").unwrap();
        let atoms = code.extract_atoms();
        assert_eq!(5, atoms.len());
        assert_eq!(&vm.engine().must_parse("TRUE"), &atoms[0]);
        assert_eq!(&vm.engine().must_parse("0.012345"), &atoms[1]);
        assert_eq!(&vm.engine().must_parse("-12784"), &atoms[2]);
        assert_eq!(&vm.engine().must_parse("a_name"), &atoms[3]);
        assert_eq!(&vm.engine().must_parse("BOOL.AND"), &atoms[4]);
    }

    #[test]
    fn code_discrepancy_items() {
        let vm = new_base_vm();
        // The discrepancy output is a hashset of every unique sub-list and atom from the specified code
        let code = vm.engine().must_parse("( ANAME ( 3 ( 1 ) ) 1 ( 1 ) )");
        let items = code.discrepancy_items();
        assert_eq!(1, *items.get(&vm.engine().must_parse("ANAME")).unwrap());
        assert_eq!(1, *items.get(&vm.engine().must_parse("( 3 ( 1 ) )")).unwrap());
        assert_eq!(1, *items.get(&vm.engine().must_parse("3")).unwrap());
        assert_eq!(2, *items.get(&vm.engine().must_parse("( 1 )")).unwrap());
        assert_eq!(3, *items.get(&vm.engine().must_parse("1")).unwrap());
        assert_eq!(5, items.len());
    }

    #[test]
    fn code_len() {
        let vm = new_base_vm();
        // `len` returns the number of elements in the direct list (not sub-lists)
        assert_eq!(0, vm.engine().must_parse("( )").len());
        assert_eq!(1, vm.engine().must_parse("( A )").len());
        assert_eq!(2, vm.engine().must_parse("( A B )").len());
        assert_eq!(2, vm.engine().must_parse("( A ( B C ) )").len());

        // It also returns 1 for atoms
        assert_eq!(1, vm.engine().must_parse("A").len());
    }

    #[test]
    fn replace() {
        let vm = new_base_vm();
        assert_eq!(
            &vm.engine().must_parse("B"),
            &vm.engine()
                .must_parse("A")
                .replace(&vm.engine().must_parse("A"), &vm.engine().must_parse("B"))
        );
        assert_eq!(
            &vm.engine().must_parse("( B )"),
            &vm.engine()
                .must_parse("( A )")
                .replace(&vm.engine().must_parse("A"), &vm.engine().must_parse("B"))
        );
        assert_eq!(
            &vm.engine().must_parse("( B B )"),
            &vm.engine()
                .must_parse("( A A )")
                .replace(&vm.engine().must_parse("A"), &vm.engine().must_parse("B"))
        );
        assert_eq!(
            &vm.engine().must_parse("B"),
            &vm.engine()
                .must_parse("( A )")
                .replace(&vm.engine().must_parse("( A )"), &vm.engine().must_parse("B"))
        );
        assert_eq!(
            &vm.engine().must_parse("( B )"),
            &vm.engine()
                .must_parse("( ( A ) )")
                .replace(&vm.engine().must_parse("( A )"), &vm.engine().must_parse("B"))
        );
        assert_eq!(
            &vm.engine().must_parse("( A A ( A A ) )"),
            &vm.engine()
                .must_parse("( A ( B ) ( A ( B ) ) )")
                .replace(&vm.engine().must_parse("( B )"), &vm.engine().must_parse("A"))
        );
    }
}
