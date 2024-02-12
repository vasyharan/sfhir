use super::*;
/// Information about a medication that is used to support knowledge.
#[derive(Debug,Clone,PartialEq)]
pub struct MedicationKnowledgeIngredient {
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A reference to the resource that provides information about the ingredient.
pub item: CodeableReference,
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
/// Specifies how many (or how much) of the items there are in this Medication.  For
/// example, 250 mg per tablet.  This is expressed as a ratio where the numerator is
/// 250mg and the denominator is 1 tablet but can also be expressed a quantity when
/// the denominator is assumed to be 1 tablet.
pub strength_codeable_concept: CodeableConcept,
/// Specifies how many (or how much) of the items there are in this Medication.  For
/// example, 250 mg per tablet.  This is expressed as a ratio where the numerator is
/// 250mg and the denominator is 1 tablet but can also be expressed a quantity when
/// the denominator is assumed to be 1 tablet.
pub strength_quantity: Quantity,
/// Specifies how many (or how much) of the items there are in this Medication.  For
/// example, 250 mg per tablet.  This is expressed as a ratio where the numerator is
/// 250mg and the denominator is 1 tablet but can also be expressed a quantity when
/// the denominator is assumed to be 1 tablet.
pub strength_ratio: Ratio,
/// Indication of whether this ingredient affects the therapeutic action of the
/// drug.
pub r#type: CodeableConcept,
}
