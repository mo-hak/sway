use crate::priv_prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Deserialize)]
pub struct LitString {
    pub span: Span,
    pub parsed: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Deserialize)]
pub struct LitChar {
    pub span: Span,
    pub parsed: char,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Deserialize)]
pub struct LitInt {
    pub span: Span,
    pub parsed: BigUint,
    pub ty_opt: Option<(LitIntType, Span)>,
    /// True if this [LitInt] represents a `b256` hex literal
    /// in a manually generated lexed tree.
    ///
    /// `b256` hex literals are not explicitly modeled in the
    /// [Literal]. During parsing, they are parsed as [LitInt]
    /// with [LitInt::ty_opt] set to `None`.
    ///
    /// To properly render `b256` manually created hex literals,
    /// that are not backed by a [Span] in the source code,
    /// we need this additional information, to distinguish
    /// them from `u256` hex literals.
    pub is_generated_b256: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LitIntType {
    U8,
    U16,
    U32,
    U64,
    U256,
    I8,
    I16,
    I32,
    I64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Deserialize)]
pub struct LitBool {
    pub span: Span,
    pub kind: LitBoolType,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LitBoolType {
    True,
    False,
}

impl From<LitBoolType> for bool {
    fn from(item: LitBoolType) -> Self {
        match item {
            LitBoolType::True => true,
            LitBoolType::False => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Literal {
    String(LitString),
    Char(LitChar),
    Int(LitInt),
    Bool(LitBool),
}

impl Spanned for LitString {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Spanned for LitChar {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Spanned for LitInt {
    fn span(&self) -> Span {
        match &self.ty_opt {
            Some((_lit_int_ty, span)) => Span::join(self.span.clone(), span),
            None => self.span.clone(),
        }
    }
}

impl Spanned for LitBool {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Spanned for Literal {
    fn span(&self) -> Span {
        match self {
            Literal::String(lit_string) => lit_string.span(),
            Literal::Char(lit_char) => lit_char.span(),
            Literal::Int(lit_int) => lit_int.span(),
            Literal::Bool(lit_bool) => lit_bool.span(),
        }
    }
}
