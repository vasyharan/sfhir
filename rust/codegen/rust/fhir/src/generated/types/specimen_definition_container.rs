use super::*;
/// A kind of specimen with associated set of requirements.
#[derive(Debug,Clone,PartialEq)]
pub struct SpecimenDefinitionContainer {
/// Substance introduced in the kind of container to preserve, maintain or enhance
/// the specimen. Examples: Formalin, Citrate, EDTA.
pub additive: Vec<SpecimenDefinitionAdditive>,
/// Color of container cap.
pub cap: CodeableConcept,
/// The capacity (volume or other measure) of this kind of container.
pub capacity: Quantity,
/// The textual description of the kind of container.
pub description: Markdown,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The type of material of the container.
pub material: CodeableConcept,
/// The minimum volume to be conditioned in the container.
pub minimum_volume_quantity: Quantity,
/// The minimum volume to be conditioned in the container.
pub minimum_volume_string: String,
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
/// Special processing that should be applied to the container for this kind of
/// specimen.
pub preparation: Markdown,
/// The type of container used to contain this kind of specimen.
pub r#type: CodeableConcept,
}
