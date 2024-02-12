use super::*;
/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug,Clone,PartialEq)]
pub struct ContractAsset {
/// Response to assets.
pub answer: Vec<ContractAnswer>,
/// Description of the quality and completeness of the asset that may be a factor in
/// its valuation.
pub condition: String,
/// Circumstance of the asset.
pub context: Vec<ContractContext>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Id [identifier??] of the clause or question text about the asset in the
/// referenced form or QuestionnaireResponse.
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
/// Asset relevant contractual time period.
pub period: Vec<Period>,
/// Type of Asset availability for use or ownership.
pub period_type: Vec<CodeableConcept>,
/// Specifies the applicability of the term to an asset resource instance, and
/// instances it refers to or instances that refer to it, and/or are owned by the
/// offeree.
pub relationship: Coding,
/// Differentiates the kind of the asset .
pub scope: CodeableConcept,
/// Security labels that protects the asset.
pub security_label_number: Vec<UnsignedInt>,
/// May be a subtype or part of an offered asset.
pub subtype: Vec<CodeableConcept>,
/// Clause or question text (Prose Object) concerning the asset in a linked form,
/// such as a QuestionnaireResponse used in the formation of the contract.
pub text: String,
/// Target entity type about which the term may be concerned.
pub r#type: Vec<CodeableConcept>,
/// Associated entities.
pub type_reference: Vec<Reference>,
/// Time period of asset use.
pub use_period: Vec<Period>,
/// Contract Valued Item List.
pub valued_item: Vec<ContractValuedItem>,
}
