use super::*;
/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug,Clone,PartialEq)]
pub struct ContractAction {
/// Encounter or Episode with primary association to the specified term activity.
pub context: Reference,
/// Id [identifier??] of the clause or question text related to the requester of
/// this action in the referenced form or QuestionnaireResponse.
pub context_link_id: Vec<String>,
/// True if the term prohibits the  action.
pub do_not_perform: Boolean,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Reason or purpose for the action stipulated by this Contract Provision.
pub intent: CodeableConcept,
/// Id [identifier??] of the clause or question text related to this action in the
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
/// Comments made about the term action made by the requester, performer, subject or
/// other participants.
pub note: Vec<Annotation>,
/// When action happens.
pub occurrence_date_time: String,
/// When action happens.
pub occurrence_period: Period,
/// When action happens.
pub occurrence_timing: Timing,
/// Indicates who or what is being asked to perform (or not perform) the ction.
pub performer: Reference,
/// Id [identifier??] of the clause or question text related to the reason type or
/// reference of this  action in the referenced form or QuestionnaireResponse.
pub performer_link_id: Vec<String>,
/// The type of role or competency of an individual desired or required to perform
/// or not perform the action.
pub performer_role: CodeableConcept,
/// The type of individual that is desired or required to perform or not perform
/// the action.
pub performer_type: Vec<CodeableConcept>,
/// Rationale for the action to be performed or not performed. Describes why the
/// action is permitted or prohibited. Either a coded concept, or another resource
/// whose existence justifies permitting or not permitting this action.
pub reason: Vec<CodeableReference>,
/// Id [identifier??] of the clause or question text related to the reason type or
/// reference of this  action in the referenced form or QuestionnaireResponse.
pub reason_link_id: Vec<String>,
/// Who or what initiated the action and has responsibility for its activation.
pub requester: Vec<Reference>,
/// Id [identifier??] of the clause or question text related to the requester of
/// this action in the referenced form or QuestionnaireResponse.
pub requester_link_id: Vec<String>,
/// Security labels that protects the action.
pub security_label_number: Vec<UnsignedInt>,
/// Current state of the term action.
pub status: CodeableConcept,
/// Entity of the action.
pub subject: Vec<ContractSubject>,
/// Activity or service obligation to be done or not done, performed or not
/// performed, effectuated or not by this Contract term.
pub r#type: CodeableConcept,
}
