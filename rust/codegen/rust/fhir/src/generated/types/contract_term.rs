use super::*;
/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug,Clone,PartialEq)]
pub struct ContractTerm {
/// An actor taking a role in an activity for which it can be assigned some degree
/// of responsibility for the activity taking place.
pub action: Vec<ContractAction>,
/// Relevant time or time-period when this Contract Provision is applicable.
pub applies: Period,
/// Contract Term Asset List.
pub asset: Vec<ContractAsset>,
/// Nested group of Contract Provisions.
pub group: Vec<ContractTerm>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Unique identifier for this particular Contract Provision.
pub identifier: Identifier,
/// When this Contract Provision was issued.
pub issued: DateTime,
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
/// The matter of concern in the context of this provision of the agrement.
pub offer: ContractOffer,
/// Security labels that protect the handling of information about the term and its
/// elements, which may be specifically identified.
pub security_label: Vec<ContractSecurityLabel>,
/// A specialized legal clause or condition based on overarching contract type.
pub sub_type: CodeableConcept,
/// Statement of a provision in a policy or a contract.
pub text: String,
/// The entity that the term applies to.
pub topic_codeable_concept: CodeableConcept,
/// The entity that the term applies to.
pub topic_reference: Reference,
/// A legal clause or condition contained within a contract that requires one or
/// both parties to perform a particular requirement by some specified time or
/// prevents one or both parties from performing a particular requirement by some
/// specified time.
pub r#type: CodeableConcept,
}
