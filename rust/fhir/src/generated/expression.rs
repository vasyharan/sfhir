/// A expression that is evaluated in a specified context and returns a value.
/// The context of use of the expression must specify the context in which the
/// expression is evaluated, and how the result of the expression is used.
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    /// A brief, natural language description of the condition that effectively
    /// communicates the intended semantics.
    pub description: super::string::String,
    /// An expression in the specified language that returns a value.
    pub expression: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The media type of the language for the expression.
    pub language: super::code::Code,
    /// A short name assigned to the expression to allow for multiple reuse of the
    /// expression in the context where it is defined.
    pub name: super::code::Code,
    /// A URI that defines where the expression is found.
    pub reference: super::uri::Uri,
}
