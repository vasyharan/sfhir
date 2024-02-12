use super::*;
/// A single issue - either an indication, contraindication, interaction or an
/// undesirable effect for a medicinal product, medication, device or procedure.
#[derive(Debug,Clone,PartialEq)]
pub struct ClinicalUseDefinitionInteraction {
/// The effect of the interaction, for example "reduced gastric absorption of
/// primary medication".
pub effect: CodeableReference,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The incidence of the interaction, e.g. theoretical, observed.
pub incidence: CodeableConcept,
/// The specific medication, product, food, substance etc. or laboratory test that
/// interacts.
pub interactant: Vec<ClinicalUseDefinitionInteractant>,
/// Actions for managing the interaction.
pub management: Vec<CodeableConcept>,
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
/// The type of the interaction e.g. drug-drug interaction, drug-food interaction,
/// drug-lab test interaction.
pub r#type: CodeableConcept,
}
