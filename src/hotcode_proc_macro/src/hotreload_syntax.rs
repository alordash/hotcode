mod named_fn_arg;
mod pat_named_type;

use crate::SourceCodeId;
use crate::caller_crate;
use named_fn_arg::*;
use not_enough_syntax::*;
use pat_named_type::*;
use proc_macro2::Span;
use quote::{ToTokens, format_ident};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

pub struct Parameters<'a> {
    pub attributes: &'a mut Vec<Attribute>,
    pub signature: &'a Signature,
    pub block: &'a mut Block,
    pub add_debug_assertions_check: bool,
}
pub fn apply(
    Parameters {
        attributes,
        signature,
        block,
        add_debug_assertions_check,
    }: Parameters,
) {
    let call_site = proc_macro::Span::call_site();
    let relative_file_path = call_site.local_file().unwrap_or_else(|| "UNKNOWN".into());
    let source_code_id =
        SourceCodeId::new(relative_file_path, call_site.line(), call_site.column());
    let fn_ident_prefix = source_code_id.into_fn_ident_prefix();
    let export_fn_ident_string =
        format_ident!("__hotcode_{}_{}", fn_ident_prefix, signature.ident).to_string();

    let unsafe_export_name_attribute = generate_unsafe_export_name_attribute(
        signature.span(),
        &export_fn_ident_string,
        add_debug_assertions_check,
    );
    attributes.push(unsafe_export_name_attribute);
    let return_stmt = {
        let span = block.span();
        let caller_library_file_name = caller_crate::library_file_name();
        let named_fn_args = name_fn_args(&signature.inputs);
        let args = named_fn_args
            .iter()
            .map(|x| match x {
                NamedFnArg::Receiver(r) => Expr::Path(expr::path::new(r.self_token.span, ["self"])),
                NamedFnArg::NamedTyped(nt) => Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: path::from_ident(nt.pat_ident.ident.clone()),
                }),
            })
            .collect();
        let call_expr = Expr::Call(ExprCall {
            attrs: Vec::new(),
            func: Box::new(Expr::Call(expr::call::new(
                span,
                Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: path::new_generics_global(
                        span,
                        ["hotcode", "provide_fn"],
                        [GenericArgument::Type(Type::FnPtr(TypeFnPtr {
                            attrs: Vec::new(),
                            lifetimes: None,
                            unsafety: None,
                            abi: None,
                            fn_token: Token![fn](span),
                            paren_token: token::Paren(span),
                            inputs: named_fn_args
                                .iter()
                                .map(|x| NamedArg {
                                    attrs: Vec::new(),
                                    name: None,
                                    ty: match x {
                                        NamedFnArg::Receiver(r) => match &r.kind {
                                            ReceiverKind::Value => Type::Path(r#type::path::new(
                                                r.self_token.span,
                                                ["Self"],
                                            )),
                                            ReceiverKind::Reference(and_token, _, mutability) => {
                                                Type::Reference(TypeReference {
                                                    attrs: Vec::new(),
                                                    and_token: *and_token,
                                                    lifetime: None,
                                                    mutability: *mutability,
                                                    elem: Box::new(Type::Path(r#type::path::new(
                                                        r.self_token.span,
                                                        ["Self"],
                                                    )))
                                                })
                                            }
                                            ReceiverKind::Typed(_, ty) => *ty.clone(),
                                            _ => panic!("Receiver type must be either value, reference or typed.")
                                        },
                                        NamedFnArg::NamedTyped(nt) => *nt.ty.clone(),
                                    },
                                })
                                .collect(),
                            variadic: None,
                            output: signature.output.clone(),
                        }))],
                    ),
                }),
                [
                    expr::lit::string(span, &caller_library_file_name),
                    expr::lit::byte_string(span, export_fn_ident_string.as_bytes()),
                ],
            ))),
            paren_token: token::Paren(span),
            args,
        });
        Stmt::Expr(
            Expr::Return(ExprReturn {
                attrs: Vec::new(),
                return_token: Token![return](span),
                expr: Some(Box::new(call_expr)),
            }),
            Some(Token![;](span)),
        )
    };
    let is_outside_dynamic_library_expr = {
        let span = block.span();
        Expr::Call(expr::call::new(
            span,
            Expr::Path(expr::path::new_global(
                span,
                ["hotcode", "is_outside_dynamic_library"],
            )),
            [],
        ))
    };
    let if_stmt = {
        let span = block.span();
        let attrs = if add_debug_assertions_check {
            vec![attributes::cfg(span, ["debug_assertions"])]
        } else {
            Vec::new()
        };
        Stmt::Expr(
            Expr::If(ExprIf {
                attrs,
                if_token: Token![if](span),
                cond: Box::new(is_outside_dynamic_library_expr),
                then_branch: Block {
                    brace_token: token::Brace(span),
                    stmts: vec![return_stmt],
                },
                else_branch: None,
            }),
            None,
        )
    };
    block.stmts.insert(0, if_stmt);
}

fn generate_unsafe_export_name_attribute(
    span: Span,
    export_fn_ident_string: &str,
    add_debug_assertions_check: bool,
) -> Attribute {
    let unsafe_export_name_meta = Meta::List(MetaList {
        path: path::new(span, ["unsafe"]),
        delimiter: MacroDelimiter::Paren(token::Paren(span)),
        tokens: MetaNameValue {
            path: path::new(span, ["export_name"]),
            eq_token: Token![=](span),
            value: expr::lit::string(span, export_fn_ident_string),
        }
        .to_token_stream(),
    });
    let result_meta = if add_debug_assertions_check {
        Meta::List(MetaList {
            path: path::new(span, ["cfg_attr"]),
            delimiter: MacroDelimiter::Paren(token::Paren(span)),
            tokens: punctuated::<_, Token![,], _>([
                Meta::Path(path::new(span, ["debug_assertions"])),
                unsafe_export_name_meta,
            ])
            .to_token_stream(),
        })
    } else {
        unsafe_export_name_meta
    };
    let result = Attribute {
        pound_token: Token![#](span),
        style: AttrStyle::Outer,
        bracket_token: token::Bracket(span),
        meta: result_meta,
    };
    return result;
}

fn name_fn_args<P>(fn_args: &Punctuated<FnArg, P>) -> Vec<NamedFnArg> {
    let result = fn_args
        .iter()
        .enumerate()
        .map(|(i, x)| match x {
            FnArg::Receiver(r) => NamedFnArg::Receiver(r.clone()),
            FnArg::Typed(t) => NamedFnArg::NamedTyped(PatNamedType {
                pat_ident: match t.pat.as_ref() {
                    Pat::Ident(pat_ident) => pat_ident.clone(),
                    _ => PatIdent {
                        attrs: Vec::new(),
                        by_ref: None,
                        mutability: None,
                        ident: format_ident!("__arg_{i}"),
                        subpat: None,
                    },
                },
                ty: t.ty.clone(),
            }),
        })
        .collect();
    return result;
}
