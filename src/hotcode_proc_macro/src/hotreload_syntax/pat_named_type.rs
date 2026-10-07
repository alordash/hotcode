use syn::*;

pub(crate) struct PatNamedType {
    pub maybe_source_pat: Option<Box<Pat>>,
    pub pat_ident: PatIdent,
    pub ty: Box<Type>,
}
