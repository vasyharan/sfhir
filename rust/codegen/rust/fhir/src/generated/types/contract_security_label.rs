use super::*;
/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug,Clone,PartialEq)]
pub struct ContractSecurityLabel {
/// Security label privacy tag that specifies the applicable privacy and security
/// policies governing this term and/or term elements.
pub category: Vec<Coding>,
/// Security label privacy tag that specifies the level of confidentiality
/// protection required for this term and/or term elements.
pub classification: Coding,
/// Security label privacy tag that specifies the manner in which term and/or term
/// elements are to be protected.
pub control: Vec<Coding>,
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
/// Number used to link this term or term element to the applicable Security Label.
pub number: Vec<UnsignedInt>,
}