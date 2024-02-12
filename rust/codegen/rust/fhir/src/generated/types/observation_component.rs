use super::*;
/// Measurements and simple assertions made about a patient, device or other
/// subject.
#[derive(Debug,Clone,PartialEq)]
pub struct ObservationComponent {
/// Describes what was observed. Sometimes this is called the observation "code".
pub code: CodeableConcept,
/// Provides a reason why the expected value in the element
/// Observation.component.value[x] is missing.
pub data_absent_reason: CodeableConcept,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A categorical assessment of an observation value.  For example, high, low,
/// normal.
pub interpretation: Vec<CodeableConcept>,
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
/// Guidance on how to interpret the value by comparison to a normal or recommended
/// range.
pub reference_range: Vec<ObservationReferenceRange>,
/// The information determined as a result of making the observation, if the
/// information has a simple value.
pub value_attachment: Attachment,
/// The information determined as a result of making the observation, if the
/// information has a simple value.
pub value_boolean: bool,
/// The information determined as a result of making the observation, if the
/// information has a simple value.
pub value_codeable_concept: CodeableConcept,
/// The information determined as a result of making the observation, if the
/// information has a simple value.
pub value_date_time: String,
/// The information determined as a result of making the observation, if the
/// information has a simple value.
pub value_integer: i64,
/// The information determined as a result of making the observation, if the
/// information has a simple value.
pub value_period: Period,
/// The information determined as a result of making the observation, if the
/// information has a simple value.
pub value_quantity: Quantity,
/// The information determined as a result of making the observation, if the
/// information has a simple value.
pub value_range: Range,
/// The information determined as a result of making the observation, if the
/// information has a simple value.
pub value_ratio: Ratio,
/// The information determined as a result of making the observation, if the
/// information has a simple value.
pub value_reference: Reference,
/// The information determined as a result of making the observation, if the
/// information has a simple value.
pub value_sampled_data: SampledData,
/// The information determined as a result of making the observation, if the
/// information has a simple value.
pub value_string: String,
/// The information determined as a result of making the observation, if the
/// information has a simple value.
pub value_time: String,
}
