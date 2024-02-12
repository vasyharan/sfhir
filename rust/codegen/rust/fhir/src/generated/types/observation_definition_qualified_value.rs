use super::*;
/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.
#[derive(Debug,Clone,PartialEq)]
pub struct ObservationDefinitionQualifiedValue {
/// The set of abnormal coded results for qualitative observations  that match the
/// criteria of this set of qualified values.
pub abnormal_coded_value_set: Canonical,
/// The age range this  set of qualified values applies to.
pub age: Range,
/// The target population this  set of qualified values applies to.
pub applies_to: Vec<CodeableConcept>,
/// Text based condition for which the the set of qualified values is valid.
pub condition: String,
/// A concept defining the context for this set of qualified values.
pub context: CodeableConcept,
/// The set of critical coded results for qualitative observations  that match the
/// criteria of this set of qualified values.
pub critical_coded_value_set: Canonical,
/// The gender this  set of qualified values applies to.
pub gender: Code,
/// The gestational age this  set of qualified values applies to.
pub gestational_age: Range,
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
/// The set of normal coded results for qualitative observations  that match the
/// criteria of this set of qualified values.
pub normal_coded_value_set: Canonical,
/// The range of values defined for continuous or ordinal observations that match
/// the criteria of this set of qualified values.
pub range: Range,
/// The category of range of values for continuous or ordinal observations that
/// match the criteria of this set of qualified values.
pub range_category: Code,
/// The set of valid coded results for qualitative observations  that match the
/// criteria of this set of qualified values.
pub valid_coded_value_set: Canonical,
}
