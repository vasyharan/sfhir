use super::*;
/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug,Clone,PartialEq)]
pub struct ImplementationGuideDependsOn {
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
/// The NPM package name for the Implementation Guide that this IG depends on.
pub package_id: Id,
/// A description explaining the nature of the dependency on the listed IG.
pub reason: Markdown,
/// A canonical reference to the Implementation guide for the dependency.
pub uri: Canonical,
/// The version of the IG that is depended on, when the correct version is required
/// to understand the IG correctly.
pub version: String,
}
