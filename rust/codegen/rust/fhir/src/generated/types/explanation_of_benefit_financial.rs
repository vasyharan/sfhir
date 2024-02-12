use super::*;
/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug,Clone,PartialEq)]
pub struct ExplanationOfBenefitFinancial {
/// The quantity of the benefit which is permitted under the coverage.
pub allowed_money: Money,
/// The quantity of the benefit which is permitted under the coverage.
pub allowed_string: String,
/// The quantity of the benefit which is permitted under the coverage.
pub allowed_unsigned_int: u64,
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
/// Classification of benefit being provided.
pub r#type: CodeableConcept,
/// The quantity of the benefit which have been consumed to date.
pub used_money: Money,
/// The quantity of the benefit which have been consumed to date.
pub used_unsigned_int: u64,
}
