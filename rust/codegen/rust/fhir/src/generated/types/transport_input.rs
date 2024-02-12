use super::*;
/// Record of transport of item.
#[derive(Debug,Clone,PartialEq)]
pub struct TransportInput {
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
/// A code or description indicating how the input is intended to be used as part of
/// the transport execution.
pub r#type: CodeableConcept,
/// The value of the input parameter as a basic type.
pub value_address: Address,
/// The value of the input parameter as a basic type.
pub value_age: Age,
/// The value of the input parameter as a basic type.
pub value_annotation: Annotation,
/// The value of the input parameter as a basic type.
pub value_attachment: Attachment,
/// The value of the input parameter as a basic type.
pub value_availability: Availability,
/// The value of the input parameter as a basic type.
pub value_base_64_binary: String,
/// The value of the input parameter as a basic type.
pub value_boolean: bool,
/// The value of the input parameter as a basic type.
pub value_canonical: String,
/// The value of the input parameter as a basic type.
pub value_code: String,
/// The value of the input parameter as a basic type.
pub value_codeable_concept: CodeableConcept,
/// The value of the input parameter as a basic type.
pub value_codeable_reference: CodeableReference,
/// The value of the input parameter as a basic type.
pub value_coding: Coding,
/// The value of the input parameter as a basic type.
pub value_contact_detail: ContactDetail,
/// The value of the input parameter as a basic type.
pub value_contact_point: ContactPoint,
/// The value of the input parameter as a basic type.
pub value_count: Count,
/// The value of the input parameter as a basic type.
pub value_data_requirement: DataRequirement,
/// The value of the input parameter as a basic type.
pub value_date: String,
/// The value of the input parameter as a basic type.
pub value_date_time: String,
/// The value of the input parameter as a basic type.
pub value_decimal: f64,
/// The value of the input parameter as a basic type.
pub value_distance: Distance,
/// The value of the input parameter as a basic type.
pub value_dosage: Dosage,
/// The value of the input parameter as a basic type.
pub value_duration: Duration,
/// The value of the input parameter as a basic type.
pub value_expression: Expression,
/// The value of the input parameter as a basic type.
pub value_extended_contact_detail: ExtendedContactDetail,
/// The value of the input parameter as a basic type.
pub value_human_name: HumanName,
/// The value of the input parameter as a basic type.
pub value_id: String,
/// The value of the input parameter as a basic type.
pub value_identifier: Identifier,
/// The value of the input parameter as a basic type.
pub value_instant: String,
/// The value of the input parameter as a basic type.
pub value_integer: i64,
/// The value of the input parameter as a basic type.
pub value_integer_64: String,
/// The value of the input parameter as a basic type.
pub value_markdown: String,
/// The value of the input parameter as a basic type.
pub value_meta: Meta,
/// The value of the input parameter as a basic type.
pub value_money: Money,
/// The value of the input parameter as a basic type.
pub value_oid: String,
/// The value of the input parameter as a basic type.
pub value_parameter_definition: ParameterDefinition,
/// The value of the input parameter as a basic type.
pub value_period: Period,
/// The value of the input parameter as a basic type.
pub value_positive_int: u64,
/// The value of the input parameter as a basic type.
pub value_quantity: Quantity,
/// The value of the input parameter as a basic type.
pub value_range: Range,
/// The value of the input parameter as a basic type.
pub value_ratio: Ratio,
/// The value of the input parameter as a basic type.
pub value_ratio_range: RatioRange,
/// The value of the input parameter as a basic type.
pub value_reference: Reference,
/// The value of the input parameter as a basic type.
pub value_related_artifact: RelatedArtifact,
/// The value of the input parameter as a basic type.
pub value_sampled_data: SampledData,
/// The value of the input parameter as a basic type.
pub value_signature: Signature,
/// The value of the input parameter as a basic type.
pub value_string: String,
/// The value of the input parameter as a basic type.
pub value_time: String,
/// The value of the input parameter as a basic type.
pub value_timing: Timing,
/// The value of the input parameter as a basic type.
pub value_trigger_definition: TriggerDefinition,
/// The value of the input parameter as a basic type.
pub value_unsigned_int: u64,
/// The value of the input parameter as a basic type.
pub value_uri: String,
/// The value of the input parameter as a basic type.
pub value_url: String,
/// The value of the input parameter as a basic type.
pub value_usage_context: UsageContext,
/// The value of the input parameter as a basic type.
pub value_uuid: String,
}
