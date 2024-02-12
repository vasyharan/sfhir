/// The parameters to the module. This collection specifies both the input and
/// output parameters. Input parameters are provided by the caller as part of the
/// $evaluate operation. Output parameters are included in the GuidanceResponse.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterDefinition {
    /// A brief discussion of what the parameter is for and how it is used by the
    /// module.
    pub documentation: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The maximum number of times this element is permitted to appear in the request
    /// or response.
    pub max: super::string::String,
    /// The minimum number of times this parameter SHALL appear in the request or
    /// response.
    pub min: super::integer::Integer,
    /// The name of the parameter used to allow access to the value of the parameter in
    /// evaluation contexts.
    pub name: super::code::Code,
    /// If specified, this indicates a profile that the input data must conform to, or
    /// that the output data will conform to.
    pub profile: super::canonical::Canonical,
    /// The type of the parameter.
    pub r#type: super::code::Code,
    /// Whether the parameter is input or output for the module.
    pub r#use: super::code::Code,
}
