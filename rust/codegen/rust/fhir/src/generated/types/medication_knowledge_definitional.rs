use super::*;
/// Information about a medication that is used to support knowledge.
#[derive(Debug,Clone,PartialEq)]
pub struct MedicationKnowledgeDefinitional {
/// Associated definitions for this medication.
pub definition: Vec<Reference>,
/// Describes the form of the item.  Powder; tablets; capsule.
pub dose_form: CodeableConcept,
/// Specifies descriptive properties of the medicine, such as color, shape,
/// imprints, etc.
pub drug_characteristic: Vec<MedicationKnowledgeDrugCharacteristic>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Identifies a particular constituent of interest in the product.
pub ingredient: Vec<MedicationKnowledgeIngredient>,
/// The intended or approved route of administration.
pub intended_route: Vec<CodeableConcept>,
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
