use super::*;
/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.
#[derive(Debug,Clone,PartialEq)]
pub struct ClaimSupportingInfo {
/// The general class of the information supplied: information; exception; accident,
/// employment; onset, etc.
pub category: CodeableConcept,
/// System and code pertaining to the specific information regarding special
/// conditions relating to the setting, treatment or patient  for which care is
/// sought.
pub code: CodeableConcept,
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
/// Provides the reason in the situation where a reason code is required in addition
/// to the content.
pub reason: CodeableConcept,
/// A number to uniquely identify supporting information entries.
pub sequence: PositiveInt,
/// The date when or period to which this information refers.
pub timing_date: String,
/// The date when or period to which this information refers.
pub timing_period: Period,
/// Additional data or information such as resources, documents, images etc.
/// including references to the data or the actual inclusion of the data.
pub value_attachment: Attachment,
/// Additional data or information such as resources, documents, images etc.
/// including references to the data or the actual inclusion of the data.
pub value_boolean: bool,
/// Additional data or information such as resources, documents, images etc.
/// including references to the data or the actual inclusion of the data.
pub value_identifier: Identifier,
/// Additional data or information such as resources, documents, images etc.
/// including references to the data or the actual inclusion of the data.
pub value_quantity: Quantity,
/// Additional data or information such as resources, documents, images etc.
/// including references to the data or the actual inclusion of the data.
pub value_reference: Reference,
/// Additional data or information such as resources, documents, images etc.
/// including references to the data or the actual inclusion of the data.
pub value_string: String,
}
