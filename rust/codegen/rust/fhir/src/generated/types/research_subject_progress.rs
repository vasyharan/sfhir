use super::*;
/// A ResearchSubject is a participant or object which is the recipient of
/// investigative activities in a research study.
#[derive(Debug,Clone,PartialEq)]
pub struct ResearchSubjectProgress {
/// The date when the state ended.
pub end_date: DateTime,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The milestones the subject has passed through.
pub milestone: CodeableConcept,
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
/// The reason for the state change.  If coded it should follow the formal subject
/// state model.
pub reason: CodeableConcept,
/// The date when the new status started.
pub start_date: DateTime,
/// The current state of the subject.
pub subject_state: CodeableConcept,
/// Identifies the aspect of the subject's journey that the state refers to.
pub r#type: CodeableConcept,
}
