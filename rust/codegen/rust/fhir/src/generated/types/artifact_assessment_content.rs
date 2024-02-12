use super::*;
/// This Resource provides one or more comments, classifiers or ratings about a
/// Resource and supports attribution and rights management metadata for the added
/// content.
#[derive(Debug,Clone,PartialEq)]
pub struct ArtifactAssessmentContent {
/// Indicates who or what authored the content.
pub author: Reference,
/// Represents a rating, classifier, or assessment of the artifact.
pub classifier: Vec<CodeableConcept>,
/// If the informationType is container, the components of the content.
pub component: Vec<ArtifactAssessmentContent>,
/// Acceptable to publicly share the comment, classifier or rating.
pub free_to_share: Boolean,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The type of information this component of the content represents.
pub information_type: Code,
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
/// A URI that points to what the comment is about, such as a line of text in the
/// CQL, or a specific element in a resource.
pub path: Vec<Uri>,
/// A quantitative rating of the artifact.
pub quantity: Quantity,
/// Additional related artifacts that provide supporting documentation, additional
/// evidence, or further information related to the content.
pub related_artifact: Vec<RelatedArtifact>,
/// A brief summary of the content of this component.
pub summary: Markdown,
/// Indicates what type of content this component represents.
pub r#type: CodeableConcept,
}
