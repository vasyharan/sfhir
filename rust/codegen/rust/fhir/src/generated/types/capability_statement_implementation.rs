use super::*;
/// A Capability Statement documents a set of capabilities (behaviors) of a
/// FHIR Server or Client for a particular version of FHIR that may be used as a
/// statement of actual server functionality or a statement of required or desired
/// server implementation.
#[derive(Debug,Clone,PartialEq)]
pub struct CapabilityStatementImplementation {
/// The organization responsible for the management of the instance and oversight of
/// the data on the server at the specified URL.
pub custodian: Reference,
/// Information about the specific installation that this capability statement
/// relates to.
pub description: Markdown,
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
/// An absolute base URL for the implementation.  This forms the base for REST
/// interfaces as well as the mailbox and document interfaces.
pub url: Url,
}
