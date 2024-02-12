use super::*;
/// Information about a medication that is used to support knowledge.
#[derive(Debug,Clone,PartialEq)]
pub struct MedicationKnowledgeCost {
/// The price or representation of the cost (for example, Band A, Band B or $, $$)
/// of the medication.
pub cost_codeable_concept: CodeableConcept,
/// The price or representation of the cost (for example, Band A, Band B or $, $$)
/// of the medication.
pub cost_money: Money,
/// The date range for which the cost information of the medication is effective.
pub effective_date: Vec<Period>,
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
/// The source or owner that assigns the price to the medication.
pub source: String,
/// The category of the cost information.  For example, manufacturers' cost, patient
/// cost, claim reimbursement cost, actual acquisition cost.
pub r#type: CodeableConcept,
}
