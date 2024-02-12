use super::*;
/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug,Clone,PartialEq)]
pub struct TestScriptVariable {
/// A default, hard-coded, or user-defined value for this variable.
pub default_value: String,
/// A free text natural language description of the variable and its purpose.
pub description: String,
/// The FHIRPath expression for a specific value to evaluate against the fixture
/// body. When variables are defined, only one of either expression, headerField or
/// path must be specified.
pub expression: String,
/// Will be used to grab the HTTP header field value from the headers that sourceId
/// is pointing to.
pub header_field: String,
/// Displayable text string with hint help information to the user when entering a
/// default value.
pub hint: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// May be used to represent additional information that is not part of the basic
/// definition of the element and that modifies the understanding of the element
/// in which it is contained and/or the understanding of the containing element's
/// descendants. Usually modifier elements provide negation or qualification.
/// To make the use of extensions safe and managable, there is a strict set
/// of governance applied to the definition and use of extensions. Though any
/// implementer can define an extension, there is a set of requirements that SHALL
/// be met as part of the definition of the extension. Applications processing a
/// resource are required to check for modifier extensions.
///
/// Modifier extensions SHALL NOT change the meaning of any elements on Resource
/// or DomainResource (including cannot change the meaning of modifierExtension
/// itself).
pub modifier_extension: Vec<Extension>,
/// Descriptive name for this variable.
pub name: String,
/// XPath or JSONPath to evaluate against the fixture body.  When variables are
/// defined, only one of either expression, headerField or path must be specified.
pub path: String,
/// Fixture to evaluate the XPath/JSONPath expression or the headerField  against
/// within this variable.
pub source_id: Id,
}
