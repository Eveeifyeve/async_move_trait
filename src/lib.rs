use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{ItemFn, ReturnType, parse_macro_input};

/// Attribute macro to transform a function into an async move future.
///
/// This macro allows you to write a function that captures variables by value
/// (using `move`) and returns an `impl Future`, even if the function signature
/// itself is not `async`.
///
/// # Example
///
/// ```
/// #[async_move_trait]
/// fn my_fn(r: &i32) -> i32 {
///     let capture = *r;
///
///     async move {}; // This will be cleaned up by the macro
///
///     capture
/// }
/// ```
///
/// The macro expands this into:
///
/// ```
/// fn my_fn(r: &i32) -> impl std::future::Future<Output = i32> {
///     let capture = *r;
///
///     async move {
///         capture
///     }
/// }
/// ```
///
/// This is especially useful for implementing async trait methods, where you
/// want to capture variables by value and return a future from a non-async
/// function signature.
///
/// - Cleans up any stray `async move {};` statements.
/// - Wraps the function body in an `async move` block.
/// - Changes the return type to `impl Future<Output = T>`, where `T` is the
///   original return type.
///
/// # When to Use
///
/// Use this macro when you need to implement async trait methods or want to
/// return a future from a function while capturing variables by value.
///
/// # Limitations
///
/// - Does not work on inherent or trait methods directly.
/// - Does not perform advanced lifetime or generic handling.
///
#[proc_macro_attribute]
pub fn async_move_trait(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let output_type = match &input.sig.output {
        ReturnType::Type(_, ty) => ty.as_ref(),
        ReturnType::Default => &Box::new(syn::parse_quote! { () }),
    };

    input.sig.output = syn::parse_quote! {
        -> impl std::future::Future<Output = #output_type>
    };

    input.block.stmts.retain(|stmt| {
        let stmt_string = stmt.to_token_stream().to_string();
        stmt_string != "async move {};"
    });

    let orig_block = &input.block;
    input.block = syn::parse_quote!({
        async move #orig_block
    });

    TokenStream::from(quote! { #input })
}
