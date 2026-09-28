use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Token,
    ItemFn, LitStr, ReturnType, Type,
};

// Custom parser for the attribute arguments
struct NativeFunctionArgs {
    name: String,
}

impl Parse for NativeFunctionArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit: LitStr = input.parse()?;
        Ok(NativeFunctionArgs { name: lit.value() })
    }
}

/// Proc macro that converts a native function to a Callable
/// Usage: #[native_function("function_name")]
#[proc_macro_attribute]
pub fn native_function(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as NativeFunctionArgs);
    let input_fn = parse_macro_input!(input as ItemFn);

    let function_name = args.name;
    let fn_ident = &input_fn.sig.ident;
    let fn_name = fn_ident.to_string();
    let wrapper_name = syn::Ident::new(&format!("{}_wrapper", fn_name), Span::call_site());

    // Get function parameters
    let inputs = &input_fn.sig.inputs;
    let input_names = inputs.iter().filter_map(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                Some(pat_ident.ident.clone())
            } else {
                None
            }
        } else {
            None
        }
    });
    let param_count = inputs.len();

    // Generate parameter conversion code
    let mut param_conversions = vec![];
    let mut param_names = vec![];

    for (i, param) in inputs.iter().enumerate() {
        let param_name = syn::Ident::new(&format!("param_{}", i), Span::call_site());
        param_names.push(param_name.clone());

        if let syn::FnArg::Typed(pat_type) = param {
            let param_type = pat_type.ty.as_ref();

            let conversion = match param_type {
                Type::Path(type_path) if type_path.path.is_ident("u32") => {
                    quote! {
                        let #param_name = match &args[#i] {
                            crate::evaluator::value::Value::Number(n) => *n as u32,
                            crate::evaluator::value::Value::Int(n) => *n as u32,
                            _ => return crate::evaluator::value::Value::Nothing,
                        };
                    }
                }
                Type::Path(type_path) if type_path.path.is_ident("i32") => {
                    quote! {
                        let #param_name = match &args[#i] {
                            crate::evaluator::value::Value::Number(n) => *n as i32,
                            crate::evaluator::value::Value::Int(n) => *n as i32,
                            _ => return crate::evaluator::value::Value::Nothing,
                        };
                    }
                }
                Type::Path(type_path) if type_path.path.is_ident("f64") => {
                    quote! {
                        let #param_name = match &args[#i] {
                            crate::evaluator::value::Value::Number(n) => *n,
                            crate::evaluator::value::Value::Int(n) => *n as f64,
                            _ => return crate::evaluator::value::Value::Nothing,
                        };
                    }
                }
                Type::Path(type_path) if type_path.path.is_ident("String") => {
                    quote! {
                        let #param_name = match &args[#i] {
                            crate::evaluator::value::Value::String(s) => s.clone(),
                            _ => return crate::evaluator::value::Value::Nothing,
                        };
                    }
                }
                Type::Path(type_path) if type_path.path.is_ident("bool") => {
                    quote! {
                        let #param_name = match &args[#i] {
                            crate::evaluator::value::Value::Boolean(b) => *b,
                            _ => return crate::evaluator::value::Value::Nothing,
                        };
                    }
                }
                _ => {
                    panic!("Unsupported parameter type in native function");
                }
            };
            param_conversions.push(conversion);
        }
    }

    // Generate return value conversion
    let return_conversion = match &input_fn.sig.output {
        ReturnType::Default => {
            quote! {
                #fn_ident(#(#param_names),*);
                crate::evaluator::value::Value::Nil
            }
        }
        ReturnType::Type(_, return_type) => match return_type.as_ref() {
            Type::Path(type_path) if type_path.path.is_ident("u32") => {
                quote! {
                    let result = #fn_ident(#(#param_names),*);
                    crate::evaluator::value::Value::Int(result as i64)
                }
            }
            Type::Path(type_path) if type_path.path.is_ident("i32") => {
                quote! {
                    let result = #fn_ident(#(#param_names),*);
                    crate::evaluator::value::Value::Int(result as i64)
                }
            }
            Type::Path(type_path) if type_path.path.is_ident("f64") => {
                quote! {
                    let result = #fn_ident(#(#param_names),*);
                    crate::evaluator::value::Value::Number(result)
                }
            }
            Type::Path(type_path) if type_path.path.is_ident("String") => {
                quote! {
                    let result = #fn_ident(#(#param_names),*);
                    crate::evaluator::value::Value::String(result)
                }
            }
            Type::Path(type_path) if type_path.path.is_ident("bool") => {
                quote! {
                    let result = #fn_ident(#(#param_names),*);
                    crate::evaluator::value::Value::Boolean(result)
                }
            }
            _ => {
                panic!("Unsupported return type in native function");
            }
        },
    };

    let expanded = quote! {
        // Keep the original function
        #input_fn

        // Create a wrapper struct that implements Callable
        #[derive(Debug, Clone)]
        pub struct #wrapper_name;

        impl crate::evaluator::callable::Callable for #wrapper_name {
            fn call(&self, _vm: &mut crate::evaluator::virtual_machine::VirtualMachine, args: Vec<crate::evaluator::value::Value>) -> crate::evaluator::value::Value {
                // Check arity
                if args.len() != #param_count {
                    return crate::evaluator::value::Value::Nothing;
                }

                // Convert arguments
                #(#param_conversions)*

                // Call the function and convert result
                #return_conversion
            }

            fn arity(&self) -> usize {
                #param_count
            }

            fn name(&self) -> String {
                #function_name.to_string()
            }

            fn callable_type(&self) -> crate::evaluator::callable::CallableType {
                crate::evaluator::callable::CallableType::NativeFunction
            }

            fn arguments(&self) -> Vec<String> {
                vec![#(stringify!(#input_names).to_string()),*]
            }
        }

        // Register the function for compile-time discovery
        inventory::submit! {
            crate::native_functions::NativeFunctionRegistration {
                name: #function_name,
                factory: || std::rc::Rc::new(#wrapper_name),
            }
        }
    };

    TokenStream::from(expanded)
}
