use super::*;
/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug,Clone,PartialEq)]
pub struct ImplementationGuideResource {
/// A description of the reason that a resource has been included in the
/// implementation guide.
pub description: Markdown,
/// Indicates the FHIR Version(s) this artifact is intended to apply to. If no
/// versions are specified, the resource is assumed to apply to all the versions
/// stated in ImplementationGuide.fhirVersion.
pub fhir_version: Vec<Code>,
/// Reference to the id of the grouping this resource appears in.
pub grouping_id: Id,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// If true, indicates the resource is an example instance.
pub is_example: Boolean,
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
/// A human assigned name for the resource. All resources SHOULD have a name, but
/// the name may be extracted from the resource (e.g. ValueSet.name).
pub name: String,
/// If present, indicates profile(s) the instance is valid against.
pub profile: Vec<Canonical>,
/// Where this resource is found.
pub reference: Reference,
}