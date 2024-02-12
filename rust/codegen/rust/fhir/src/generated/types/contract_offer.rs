use super::*;
/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug,Clone,PartialEq)]
pub struct ContractOffer {
/// Response to offer text.
pub answer: Vec<ContractAnswer>,
/// Type of choice made by accepting party with respect to an offer made by an
/// offeror/ grantee.
pub decision: CodeableConcept,
/// How the decision about a Contract was conveyed.
pub decision_mode: Vec<CodeableConcept>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Unique identifier for this particular Contract Provision.
pub identifier: Vec<Identifier>,
/// The id of the clause or question text of the offer in the referenced
/// questionnaire/response.
pub link_id: Vec<String>,
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
/// Offer Recipient.
pub party: Vec<ContractParty>,
/// Security labels that protects the offer.
pub security_label_number: Vec<UnsignedInt>,
/// Human readable form of this Contract Offer.
pub text: String,
/// The owner of an asset has the residual control rights over the asset: the right
/// to decide all usages of the asset in any way not inconsistent with a prior
/// contract, custom, or law (Hart, 1995, p. 30).
pub topic: Reference,
/// Type of Contract Provision such as specific requirements, purposes for actions,
/// obligations, prohibitions, e.g. life time maximum benefit.
pub r#type: CodeableConcept,
}
