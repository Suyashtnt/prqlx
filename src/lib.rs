use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, ExprLit, Lit, Path, Token,
};

struct QueryInput {
    /// The SQL query to execute
    query: Expr,
    /// Any extra arguments to pass to [`sqlx::query`]
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for QueryInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let query: Expr = input.parse()?;
        let args = if input.is_empty() {
            Punctuated::new()
        } else {
            let _: Token![,] = input.parse()?;
            input.parse_terminated(Expr::parse, Token![,])?
        };

        Ok(Self { query, args })
    }
}

struct QueryAsInput {
    /// The type to parse the query as
    ty: Path,
    /// The SQL query to execute
    query: Expr,
    /// Any extra arguments to pass to [`sqlx::query`]
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for QueryAsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Path = input.parse()?;
        let _: Token![,] = input.parse()?;
        let query: Expr = input.parse()?;
        let args = if input.is_empty() {
            Punctuated::new()
        } else {
            let _: Token![,] = input.parse()?;
            input.parse_terminated(Expr::parse, Token![,])?
        };

        Ok(Self { ty, query, args })
    }
}

fn to_sql(query: Expr) -> String {
    let prql_string = match query {
        Expr::Lit(ExprLit {
            lit: Lit::Str(lit_str),
            ..
        }) => lit_str.value(),
        _ => panic!("query! requires a string for the query"),
    };

    let opts = prql_compiler::Options::default().no_format().no_signature();
    match prql_compiler::compile(&prql_string, &opts) {
        Ok(r) => r,
        Err(err) => {
            panic!("{}", err)
        }
    }
}

#[proc_macro]
/// Combines a PRQL query with a call to [`sqlx::query`].
pub fn query(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as QueryInput);

    let sql_string = to_sql(parsed.query);

    if parsed.args.is_empty() {
        let tokens = quote! {
            sqlx::query!(#sql_string)
        };

        tokens.into()
    } else {
        let args = parsed.args.iter();
        let tokens = quote! {
            sqlx::query!(#sql_string, #(#args),*)
        };

        tokens.into()
    }
}

#[proc_macro]
/// Combines a PRQL query with a call to [`sqlx::query_as`].
pub fn query_as(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as QueryAsInput);

    let sql_string = to_sql(parsed.query);
    let ty = parsed.ty.require_ident().unwrap();

    if parsed.args.is_empty() {
        let tokens = quote! {
            sqlx::query_as!(#ty, #sql_string)
        };

        tokens.into()
    } else {
        let args = parsed.args.iter();
        let tokens = quote! {
            sqlx::query_as!(#ty, #sql_string, #(#args),*)
        };

        tokens.into()
    }
}
