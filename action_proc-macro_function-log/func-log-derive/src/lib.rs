use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn func_log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);

    let func_vis = &func.vis;
    let func_block = &func.block;

    let func_decl = &func.sig;

    if func_decl.constness.is_some() || func_decl.asyncness.is_some() {
        panic!("Try to log a function which is const or async.");
    }

    let func_unsafety = &func_decl.unsafety;
    let func_fn = &func_decl.fn_token;
    let func_name = &func_decl.ident;
    let func_generics = &func_decl.generics;
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;

    let func_name_str = func_name.to_string();

    let expanded = quote! {
    #func_vis #func_unsafety #func_fn #func_name #func_generics(#func_inputs) #func_output {
            println!("Enter function: {}.", #func_name_str);
            let ret = #func_block;
            println!("Exit function: {}.", #func_name_str);
            ret
        }
    };

    expanded.into()
}
