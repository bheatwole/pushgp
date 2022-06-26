use crate::item_fn::ItemFn;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{Block, Error, Expr, FnArg, Ident, Pat, Path, Stmt, Type, TypeParamBound};

struct FunctionParseResults {
    // The Pascal case name of every stack we have detected.
    pub stacks: HashSet<String>,

    // The argument names of the values that we should pop of their stacks, organized by stack. The first value in each
    // Vec will be the first value popped.
    pub pop: HashMap<String, Vec<Ident>>,
}

pub fn handle_macro(stack_ident: &Ident, inner_fn: &mut ItemFn) -> Result<TokenStream> {
    let mut parse_results = FunctionParseResults {
        stacks: HashSet::default(),
        pop: HashMap::default(),
    };

    // Determine the full path that we should reference the 'pushgp' library in our code
    let pushgp =
        match crate_name("pushgp").map_err(|e| Error::new(inner_fn.span(), e.to_string()))? {
            FoundCrate::Itself => "crate".to_owned(),
            FoundCrate::Name(path) => path,
        };
    let pushgp: Path = syn::parse_str::<Path>(&pushgp)?;

    // Determine the base stack name
    let stack_name = format!("{}", stack_ident).to_case(Case::Pascal);
    parse_results.stacks.insert(stack_name.clone());

    // Use the base stack name plus the name of the function to generate the name of the struct
    let function_name = inner_fn.sig.ident.to_string();
    let struct_name: Ident = syn::parse_str::<Ident>(&format!(
        "{}{}",
        stack_name.to_case(Case::Pascal),
        function_name.to_case(Case::Pascal)
    ))?;

    // Use the base stack name plus the name of the function to generate the name of the instruction
    let instruction_name_str = format!(
        "{}.{}",
        stack_name.to_case(Case::UpperFlat),
        function_name.to_case(Case::UpperFlat)
    );

    // Only keep the 'doc' attributes from what's supplied for the function
    inner_fn.attrs.retain(|attr| attr.path.is_ident("doc"));
    let docs = inner_fn.attrs.iter();

    // Parse the fn arguments
    parse_arguments(&inner_fn, &mut parse_results)?;

    // Parse the fn body
    let body = parse_body(&inner_fn, &mut parse_results)?;
    let body = wrap_body(body, &parse_results)?;

    // Make the bound types
    let bound_types = make_bound_types(&parse_results, quote!(#pushgp).to_string())?;

    Ok(quote! {
        #[derive(Debug, PartialEq)]
        pub struct #struct_name {}

        impl #pushgp::StaticName for #struct_name {
            fn static_name() -> &'static str {
                #instruction_name_str
            }
        }
        impl<Vm> #pushgp::StaticInstruction<Vm> for #struct_name
        where
            Vm: #(#bound_types)+*,
        {
            fn parse(input: &str) -> nom::IResult<&str, #pushgp::Code<Vm>> {
                let (rest, _) = nom::bytes::complete::tag(#struct_name::static_name())(input)?;
                let (rest, _) = crate::parse::space_or_end(rest)?;

                Ok((rest, Box::new(#struct_name {})))
            }

            fn random_value(_vm: &mut Vm) -> #pushgp::Code<Vm> {
                Box::new(#struct_name {})
            }
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(#struct_name::static_name())
            }
        }

        impl<Vm> #pushgp::Instruction<Vm> for #struct_name
        where
            Vm: #(#bound_types)+*,
        {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn name(&self) -> &'static str {
                #struct_name::static_name()
            }
            fn clone(&self) -> #pushgp::Code<Vm> {
                Box::new(#struct_name{})
            }
            #(#docs)*
            fn execute(&mut self, vm: &mut Vm) #body
        }
    })
}

fn parse_arguments(inner_fn: &ItemFn, parse_results: &mut FunctionParseResults) -> Result<()> {
    let mut has_vm = false;
    for fn_arg in inner_fn.sig.inputs.iter() {
        if !has_vm {
            if !fn_arg_is_vm_mut_vm(fn_arg) {
                return Err(Error::new(
                    fn_arg.span(),
                    "the function's first parameter must be `vm: &mut Vm`",
                ));
            }
            has_vm = true;
        } else {
            match (fn_arg_name(fn_arg), fn_arg_path_type(fn_arg)) {
                (Some(name_ident), Some(stack_ident)) => {
                    let stack_string = stack_ident.to_string();
                    parse_results.stacks.insert(stack_string.clone());
                    parse_results.pop.entry(stack_string).or_insert(vec![]).push(name_ident);
                }
                _ => return Err(Error::new(
                    fn_arg.span(),
                    "the function's other parameters must be in the format '<variable>: <StackType>' as in `left: Integer`",
                ))
            }
        }
    }

    if !has_vm {
        return Err(Error::new(
            inner_fn.span(),
            "the function must have at least one parameter of `vm: &mut Vm`",
        ));
    }
    Ok(())
}

fn fn_arg_is_vm_mut_vm(arg: &FnArg) -> bool {
    match (fn_arg_name(arg), fn_arg_mut_ref_type(arg)) {
        (Some(name_ident), Some(type_ident)) => name_ident == "vm" && type_ident == "Vm",
        _ => false,
    }
}

fn fn_arg_name(arg: &FnArg) -> Option<Ident> {
    match arg {
        FnArg::Typed(pat_type) => {
            if let Pat::Ident(pat_ident) = pat_type.pat.as_ref() {
                Some(pat_ident.ident.clone())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn fn_arg_mut_ref_type(arg: &FnArg) -> Option<Ident> {
    match arg {
        FnArg::Typed(pat_type) => {
            if let Type::Reference(type_ref) = pat_type.ty.as_ref() {
                if type_ref.mutability.is_none() {
                    None
                } else {
                    if let Type::Path(type_path) = type_ref.elem.as_ref() {
                        type_path.path.get_ident().map(|i| i.clone())
                    } else {
                        None
                    }
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn fn_arg_path_type(arg: &FnArg) -> Option<Ident> {
    match arg {
        FnArg::Typed(pat_type) => {
            if let Type::Path(type_path) = pat_type.ty.as_ref() {
                type_path.path.get_ident().map(|i| i.clone())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_body(inner_fn: &ItemFn, parse_results: &mut FunctionParseResults) -> Result<Block> {
    let body_block = inner_fn.block.as_ref();

    // We're looking for every possible place that `vm.<stack>()` is called
    for stmt in body_block.stmts.iter() {
        find_stack_in_stmt(stmt, parse_results);
    }

    Ok(body_block.clone())
}

fn find_stack_in_stmt(stmt: &Stmt, parse_results: &mut FunctionParseResults) {
    match stmt {
        Stmt::Local(local) => {
            if let Some((_, boxed_expr)) = &local.init {
                find_stack_in_expr(boxed_expr, parse_results);
            }
        }
        Stmt::Item(_item) => {}
        Stmt::Expr(expr) => {
            find_stack_in_expr(expr, parse_results);
        }
        Stmt::Semi(expr, _) => {
            find_stack_in_expr(expr, parse_results);
        }
    }
}

fn find_stack_in_expr(expr: &Expr, parse_results: &mut FunctionParseResults) {
    match expr {
        // This is the key on we're looking for; a method call where the receiver is `vm`
        Expr::MethodCall(expr) => {
            if let Some(receiver) = expr_path_ident(expr.receiver.as_ref()) {
                if receiver == "vm" {
                    // Skip the methods from the VirtualMachine trait
                    let method: String = expr.method.to_string();
                    if !(method == "get_rng"
                        || method == "set_rng_seed"
                        || method == "clear"
                        || method == "run"
                        || method == "next"
                        || method == "add_instruction"
                        || method == "get_configuration"
                        || method == "reset_configuration"
                        || method == "get_instruction_weights"
                        || method == "generate_random_instruction"
                        || method == "parse"
                        || method == "must_parse"
                        || method == "parse_and_set_code"
                        || method == "set_code")
                    {
                        parse_results.stacks.insert(method.to_case(Case::Pascal));
                    }
                }
            } else {
                find_stack_in_expr(expr.receiver.as_ref(), parse_results);
            }
            for arg in expr.args.iter() {
                find_stack_in_expr(arg, parse_results);
            }
        }
        Expr::Array(expr) => {
            for expr in expr.elems.iter() {
                find_stack_in_expr(expr, parse_results);
            }
        }
        Expr::Assign(expr) => {
            find_stack_in_expr(expr.left.as_ref(), parse_results);
            find_stack_in_expr(expr.right.as_ref(), parse_results);
        }
        Expr::AssignOp(expr) => {
            find_stack_in_expr(expr.left.as_ref(), parse_results);
            find_stack_in_expr(expr.right.as_ref(), parse_results);
        }
        Expr::Async(expr) => {
            for stmt in expr.block.stmts.iter() {
                find_stack_in_stmt(stmt, parse_results);
            }
        }
        Expr::Await(expr) => {
            find_stack_in_expr(expr.base.as_ref(), parse_results);
        }
        Expr::Binary(expr) => {
            find_stack_in_expr(expr.left.as_ref(), parse_results);
            find_stack_in_expr(expr.right.as_ref(), parse_results);
        }
        Expr::Block(expr) => {
            for stmt in expr.block.stmts.iter() {
                find_stack_in_stmt(stmt, parse_results);
            }
        }
        Expr::Box(expr) => {
            find_stack_in_expr(expr.expr.as_ref(), parse_results);
        }
        Expr::Break(expr) => {
            if let Some(expr) = &expr.expr {
                find_stack_in_expr(expr.as_ref(), parse_results);
            }
        }
        Expr::Call(expr) => {
            find_stack_in_expr(expr.func.as_ref(), parse_results);
            for expr in expr.args.iter() {
                find_stack_in_expr(expr, parse_results);
            }
        }
        Expr::Cast(expr) => {
            find_stack_in_expr(expr.expr.as_ref(), parse_results);
        }
        Expr::Closure(expr) => {
            find_stack_in_expr(expr.body.as_ref(), parse_results);
        }
        Expr::Field(expr) => {
            find_stack_in_expr(expr.base.as_ref(), parse_results);
        }
        Expr::ForLoop(expr) => {
            find_stack_in_expr(expr.expr.as_ref(), parse_results);
            for stmt in expr.body.stmts.iter() {
                find_stack_in_stmt(stmt, parse_results);
            }
        }
        Expr::Group(expr) => {
            find_stack_in_expr(expr.expr.as_ref(), parse_results);
        }
        Expr::If(expr) => {
            find_stack_in_expr(expr.cond.as_ref(), parse_results);
            for stmt in expr.then_branch.stmts.iter() {
                find_stack_in_stmt(stmt, parse_results);
            }
            if let Some((_, expr)) = &expr.else_branch {
                find_stack_in_expr(expr.as_ref(), parse_results);
            }
        }
        Expr::Index(expr) => {
            find_stack_in_expr(expr.expr.as_ref(), parse_results);
            find_stack_in_expr(expr.index.as_ref(), parse_results);
        }
        Expr::Let(expr) => {
            find_stack_in_expr(expr.expr.as_ref(), parse_results);
        }
        Expr::Loop(expr) => {
            for stmt in expr.body.stmts.iter() {
                find_stack_in_stmt(stmt, parse_results);
            }
        }
        Expr::Match(expr) => {
            find_stack_in_expr(expr.expr.as_ref(), parse_results);
            for arm in expr.arms.iter() {
                if let Some((_, expr)) = &arm.guard {
                    find_stack_in_expr(expr.as_ref(), parse_results);
                }
                find_stack_in_expr(arm.body.as_ref(), parse_results);
            }
        }
        Expr::Paren(expr) => {
            find_stack_in_expr(expr.expr.as_ref(), parse_results);
        }
        Expr::Range(expr) => {
            if let Some(expr) = &expr.from {
                find_stack_in_expr(expr.as_ref(), parse_results);
            }
            if let Some(expr) = &expr.to {
                find_stack_in_expr(expr.as_ref(), parse_results);
            }
        }
        Expr::Reference(expr) => {
            find_stack_in_expr(expr.expr.as_ref(), parse_results);
        }
        Expr::Repeat(expr) => {
            find_stack_in_expr(expr.expr.as_ref(), parse_results);
            find_stack_in_expr(expr.len.as_ref(), parse_results);
        }
        Expr::Try(expr) => {
            find_stack_in_expr(expr.expr.as_ref(), parse_results);
        }
        Expr::TryBlock(expr) => {
            for stmt in expr.block.stmts.iter() {
                find_stack_in_stmt(stmt, parse_results);
            }
        }
        Expr::Tuple(expr) => {
            for expr in expr.elems.iter() {
                find_stack_in_expr(expr, parse_results);
            }
        }
        Expr::Type(expr) => {
            find_stack_in_expr(expr.expr.as_ref(), parse_results);
        }
        Expr::Unary(expr) => {
            find_stack_in_expr(expr.expr.as_ref(), parse_results);
        }
        Expr::Unsafe(expr) => {
            for stmt in expr.block.stmts.iter() {
                find_stack_in_stmt(stmt, parse_results);
            }
        }
        Expr::While(expr) => {
            find_stack_in_expr(expr.cond.as_ref(), parse_results);
            for stmt in expr.body.stmts.iter() {
                find_stack_in_stmt(stmt, parse_results);
            }
        }
        Expr::Yield(expr) => {
            if let Some(expr) = &expr.expr {
                find_stack_in_expr(expr.as_ref(), parse_results);
            }
        }
        // The rest of the types don't do anything
        _ => {}
    }
}

// If the expr is Expr::Path(path) where path.is_ident(), returns path.get_ident()
fn expr_path_ident(expr: &Expr) -> Option<Ident> {
    match expr {
        Expr::Path(expr) => expr.path.get_ident().map(|i| i.clone()),
        _ => None,
    }
}

fn wrap_body(original_body: Block, parse_results: &FunctionParseResults) -> Result<Block> {
    use syn::{BinOp, ExprBinary, ExprIf, Local, PatIdent};

    let mut if_conditions: Option<Box<Expr>> = None;
    let mut let_stmts = vec![];
    for (stack_name, variables) in parse_results.pop.iter() {
        // Create a new expression that checks the length of this stack
        let check_len = ExprBinary {
            attrs: vec![],
            left: Box::new(syn::parse_str::<Expr>(&format!(
                "vm.{}().len()",
                stack_name.to_case(Case::Snake)
            ))?),
            op: BinOp::Ge(syn::token::Ge::default()),
            right: Box::new(syn::parse_str::<Expr>(&format!("{}", variables.len()))?),
        };

        // If we check the len of more than one stack, we need to && them together
        if let Some(past_conditions) = if_conditions {
            let and_more = ExprBinary {
                attrs: vec![],
                left: past_conditions,
                op: BinOp::And(syn::token::AndAnd::default()),
                right: Box::new(Expr::Binary(check_len)),
            };
            if_conditions = Some(Box::new(Expr::Binary(and_more)));
        } else {
            if_conditions = Some(Box::new(Expr::Binary(check_len)));
        }

        // Create the statements that pop that stack to a variable with the names that the rest of the function expect.
        for var in variables {
            let pat_name = PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: None,
                ident: var.clone(),
                subpat: None,
            };
            let pop_expr = syn::parse_str::<Expr>(&format!(
                "vm.{}().pop().unwrap()",
                stack_name.to_case(Case::Snake)
            ))?;
            let local = Local {
                attrs: vec![],
                let_token: syn::token::Let::default(),
                pat: Pat::Ident(pat_name),
                init: Some((syn::token::Eq::default(), Box::new(pop_expr))),
                semi_token: syn::token::Semi::default(),
            };
            let_stmts.push(Stmt::Local(local));
        }
    }

    if let Some(conditions) = if_conditions {
        for stmt in original_body.stmts {
            let_stmts.push(stmt.clone());
        }

        let mut stmts = vec![];
        stmts.push(Stmt::Expr(Expr::If(ExprIf {
            attrs: vec![],
            if_token: syn::token::If::default(),
            cond: conditions,
            then_branch: Block {
                brace_token: syn::token::Brace::default(),
                stmts: let_stmts,
            },
            else_branch: None,
        })));

        Ok(Block {
            brace_token: syn::token::Brace::default(),
            stmts,
        })
    } else {
        Ok(original_body)
    }
}

fn make_bound_types(
    parse_results: &FunctionParseResults,
    pushgp: String,
) -> Result<Vec<TypeParamBound>> {
    let mut bound_types: Vec<TypeParamBound> = vec![];

    // Everything must have the VirtualMachine type
    bound_types.push(syn::parse_str::<TypeParamBound>(&format!(
        "{}::VirtualMachine",
        pushgp
    ))?);

    // The VirtualMachine *type* must have a static lifetime
    bound_types.push(syn::parse_str::<TypeParamBound>("'static")?);

    // All the literals also require the 'Exec' stack, and every VirtualMachine MUST implement Exec so add it whether it
    // was explicitly called or not
    bound_types.push(syn::parse_str::<TypeParamBound>(&format!(
        "{}::VirtualMachineMustHaveExec<Vm>",
        pushgp
    ))?);

    // Add the trait constraints for all the traits the user called
    for stack in parse_results.stacks.iter() {
        if let Some(string_bound) = match stack.as_str() {
            // We already added this earlier
            "Exec" => None,

            // These are part the 'pushgp' namespace
            "Bool" | "Code" | "Float" | "Integer" | "Name" => {
                Some(format!("{}::VirtualMachineMustHave{}<Vm>", pushgp, stack))
            }

            // Anything else is part of the user's namespace and they need to have it 'used'
            _ => Some(format!("VirtualMachineMustHave{}<Vm>", stack)),
        } {
            bound_types.push(syn::parse_str::<TypeParamBound>(&string_bound)?);
        }
    }

    Ok(bound_types)
}
