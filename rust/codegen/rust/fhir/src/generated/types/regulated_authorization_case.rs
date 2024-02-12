use super::*;
/// Regulatory approval, clearance or licencing related to a regulated product,
/// treatment, facility or activity that is cited in a guidance, regulation, rule
/// or legislative act. An example is Market Authorization relating to a Medicinal
/// Product.
#[derive(Debug,Clone,PartialEq)]
pub struct RegulatedAuthorizationCase {
/// A regulatory submission from an organization to a regulator, as part of
/// an assessing case. Multiple applications may occur over time, with more or
/// different information to support or modify the submission or the authorization.
/// The applications can be considered as steps within the longer running case or
/// procedure for this authorization process.
pub application: Vec<RegulatedAuthorizationCase>,
/// Relevant date for this case.
pub date_date_time: String,
/// Relevant date for this case.
pub date_period: Period,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Identifier by which this case can be referenced.
pub identifier: Identifier,
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
/// The status associated with the case.
pub status: CodeableConcept,
/// The defining type of case.
pub r#type: CodeableConcept,
}
