/// Internal documentation model, independent from rustdoc JSON types.
///
/// This model represents the documentation structure that will be rendered
/// to Markdown. It is produced by the conversion layer from rustdoc JSON.

/// Top-level crate documentation.
#[derive(Debug, Clone)]
pub struct CrateDoc {
    pub name: String,
    pub docs: Option<String>,
    pub modules: Vec<ModuleDoc>,
    pub impls: Vec<ImplDoc>,
    pub structs: Vec<StructDoc>,
    pub enums: Vec<EnumDoc>,
    pub traits: Vec<TraitDoc>,
    pub functions: Vec<FunctionDoc>,
    pub type_aliases: Vec<TypeAliasDoc>,
    pub constants: Vec<ConstantDoc>,
    pub statics: Vec<StaticDoc>,
}

/// A module and its contents.
#[derive(Debug, Clone)]
pub struct ModuleDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub modules: Vec<ModuleDoc>,
    pub impls: Vec<ImplDoc>,
    pub structs: Vec<StructDoc>,
    pub enums: Vec<EnumDoc>,
    pub traits: Vec<TraitDoc>,
    pub functions: Vec<FunctionDoc>,
    pub type_aliases: Vec<TypeAliasDoc>,
    pub constants: Vec<ConstantDoc>,
    pub statics: Vec<StaticDoc>,
}

/// A struct field with its type and documentation.
#[derive(Debug, Clone)]
pub struct FieldDoc {
    pub name: String,
    pub type_str: String,
    pub docs: Option<String>,
    pub is_public: bool,
}

/// Kind of enum variant.
#[derive(Debug, Clone)]
pub enum VariantKind {
    /// A plain variant with no data (e.g. `Foo`).
    Plain,
    /// A tuple variant (e.g. `Foo(u32, String)`).
    Tuple(Vec<String>),
    /// A struct variant (e.g. `Foo { bar: u32 }`).
    Struct(Vec<FieldDoc>),
}

/// An enum variant with its documentation.
#[derive(Debug, Clone)]
pub struct VariantDoc {
    pub name: String,
    pub docs: Option<String>,
    pub kind: VariantKind,
}

/// A struct with its fields, inherent methods, and signature.
#[derive(Debug, Clone)]
pub struct StructDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub fields: Vec<FieldDoc>,
}

/// An enum with its variants, inherent methods, and signature.
#[derive(Debug, Clone)]
pub struct EnumDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub variants: Vec<VariantDoc>,
}

/// A trait with its required and provided methods.
#[derive(Debug, Clone)]
pub struct TraitDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub methods: Vec<MethodDoc>,
}

/// A free function (not a method).
#[derive(Debug, Clone)]
pub struct FunctionDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
}

/// A method belonging to a type or trait.
#[derive(Debug, Clone)]
pub struct MethodDoc {
    pub name: String,
    pub docs: Option<String>,
    pub signature: String,
}

/// An impl block defined in a module.
#[derive(Debug, Clone)]
pub struct ImplDoc {
    pub header: String,
    pub docs: Option<String>,
    pub methods: Vec<MethodDoc>,
    pub target_name: String,
}

/// A type alias with its definition.
#[derive(Debug, Clone)]
pub struct TypeAliasDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
}

/// A constant with its type and value.
#[derive(Debug, Clone)]
pub struct ConstantDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
}

/// A static variable with its type.
#[derive(Debug, Clone)]
pub struct StaticDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
}
