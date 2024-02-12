use super::*;
/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug,Clone,PartialEq)]
pub struct CitationRelatesTo {
/// A bibliographic citation for the related artifact. This text SHOULD be formatted
/// according to an accepted citation format.
pub citation: Markdown,
/// Provides additional classifiers of the related artifact.
pub classifier: Vec<CodeableConcept>,
/// A brief description of the document or knowledge resource being referenced,
/// suitable for display to a consumer.
pub display: String,
/// The document being referenced, represented as an attachment. Do not use this
/// element if using the resource element to provide the canonical to the related
/// artifact.
pub document: Attachment,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A short label that can be used to reference the related artifact from elsewhere
/// in the containing artifact, such as a footnote index.
pub label: String,
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
/// The related artifact, such as a library, value set, profile, or other knowledge
/// resource.
pub resource: Canonical,
/// The related artifact, if the artifact is not a canonical resource, or a resource
/// reference to a canonical resource.
pub resource_reference: Reference,
/// The type of relationship to the related artifact.
pub r#type: Code,
}
