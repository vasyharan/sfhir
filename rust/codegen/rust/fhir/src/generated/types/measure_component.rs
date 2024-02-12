use super::*;
/// The Measure resource provides the definition of a quality measure.
#[derive(Debug,Clone,PartialEq)]
pub struct MeasureComponent {
/// Indicates a meaning for the stratifier component. This can be as simple as a
/// unique identifier, or it can establish meaning in a broader context by drawing
/// from a terminology, allowing stratifiers to be correlated across measures.
pub code: CodeableConcept,
/// An expression that specifies the criteria for this component of the stratifier.
/// This is typically the name of an expression defined within a referenced library,
/// but it may also be a path to a stratifier element.
pub criteria: Expression,
/// The human readable description of this stratifier criteria component.
pub description: Markdown,
/// A Group resource that defines this population as a set of characteristics.
pub group_definition: Reference,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// An identifier that is unique within the Measure allowing linkage to the
/// equivalent item in a MeasureReport resource.
pub link_id: String,
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
}
