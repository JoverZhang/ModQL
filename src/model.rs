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
    pub structs: Vec<StructDoc>,
    pub enums: Vec<EnumDoc>,
    pub traits: Vec<TraitDoc>,
    pub functions: Vec<FunctionDoc>,
    pub type_aliases: Vec<TypeAliasDoc>,
    pub constants: Vec<ConstantDoc>,
    pub statics: Vec<StaticDoc>,
}

/// A struct with its inherent methods.
#[derive(Debug, Clone)]
pub struct StructDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub methods: Vec<MethodDoc>,
}

/// An enum with its inherent methods.
#[derive(Debug, Clone)]
pub struct EnumDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub methods: Vec<MethodDoc>,
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

/// A type alias.
#[derive(Debug, Clone)]
pub struct TypeAliasDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
}

/// A constant.
#[derive(Debug, Clone)]
pub struct ConstantDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
}

/// A static variable.
#[derive(Debug, Clone)]
pub struct StaticDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
}
