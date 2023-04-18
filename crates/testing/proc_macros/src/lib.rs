use proc_macro::TokenStream;
use quote::*;
use syn::parse_macro_input;

#[proc_macro_attribute]
/// Some tests need extra guard to be executed safely.
/// It fixes some flakes that are caused by tests that are executed in parallel and are using the same resources.
///
/// Make sure to add `#[safe_test]` attribute to each test that needs to be executed safely.
///
/// This macro will inject static mutex variable, so that tests are executed safely without impacting each other.
/// Mutex variable name can be provided as a macro param.
/// If no param is provided, default name TEST_MUTEX will be used.
///
///
/// Example:
/// ```rust
/// #[cfg(test)]
/// #[safe_tests]
/// pub(crate) mod test {
///     const TEST_ENV_VAR: &str = "TEST_ENV_VAR";
///
///     #[test]
///     #[safe_test]
///     fn it_works1() {
///         std::env::set_var(TEST_ENV_VAR, "test1");
///         let test_value = std::env::var(TEST_ENV_VAR).unwrap();
///         assert_eq!(test_value, "test1");
///     }
///     #[test]
///     #[safe_test]
///     fn it_works2() {
///         std::env::set_var(TEST_ENV_VAR, "test2");
///         let test_value = std::env::var(TEST_ENV_VAR).unwrap();
///         assert_eq!(test_value, "test2");
///     }
/// }
/// ```
pub fn safe_tests(attr: TokenStream, input: TokenStream) -> TokenStream {
    let module = parse_macro_input!(input as syn::ItemMod);

    let mutex_name = attr.to_string();

    let mutex_name = if mutex_name.is_empty() {
        "TEST_MUTEX".to_string()
    } else {
        mutex_name
    };
    let mutex_var = syn::Ident::new(&mutex_name, proc_macro2::Span::call_site());

    let content = module.content.unwrap().1;
    proc_macro::TokenStream::from(quote! {
        // Inject static mutex variable, so that tests are executed safely without impacting each other
        static #mutex_var: std::sync::Mutex<()> = std::sync::Mutex::new(());
        #(#content)*
    })
}

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}

/// Uses static mutex variable to execute test safely.
/// Mutex variable name can be provided as a macro param.
/// If no param is provided, default name TEST_MUTEX will be used.
#[proc_macro_attribute]
pub fn safe_test(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input: syn::ItemFn = match syn::parse(input.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(input, e),
    };

    let mutex_name = attr.to_string();

    let mutex_name = if mutex_name.is_empty() {
        "TEST_MUTEX".to_string()
    } else {
        mutex_name
    };
    let mutex_var = syn::Ident::new(&mutex_name, proc_macro2::Span::call_site());

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;

    let stmts = &block.stmts;

    let output = quote! {
        #(#attrs)* #vis #sig {
            let _guard = #mutex_var.lock().unwrap();

            #(#stmts)*
        }
    };

    output.into()
}
