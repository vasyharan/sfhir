use super::*;
/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug,Clone,PartialEq)]
pub struct ExplanationOfBenefitCareTeam {
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
/// Member of the team who provided the product or service.
pub provider: Reference,
/// The party who is billing and/or responsible for the claimed products or
/// services.
pub responsible: Boolean,
/// The lead, assisting or supervising practitioner and their discipline if a
/// multidisciplinary team.
pub role: CodeableConcept,
/// A number to uniquely identify care team entries.
pub sequence: PositiveInt,
/// The specialization of the practitioner or provider which is applicable for this
/// service.
pub specialty: CodeableConcept,
}
