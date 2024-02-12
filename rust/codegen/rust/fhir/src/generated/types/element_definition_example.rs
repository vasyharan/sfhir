use super::*;
/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug,Clone,PartialEq)]
pub struct ElementDefinitionExample {
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Describes the purpose of this example among the set of examples.
pub label: String,
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
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_address: Address,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_age: Age,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_annotation: Annotation,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_attachment: Attachment,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_availability: Availability,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_base_64_binary: String,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_boolean: bool,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_canonical: String,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_code: String,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_codeable_concept: CodeableConcept,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_codeable_reference: CodeableReference,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_coding: Coding,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_contact_detail: ContactDetail,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_contact_point: ContactPoint,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_count: Count,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_data_requirement: DataRequirement,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_date: String,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_date_time: String,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_decimal: f64,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_distance: Distance,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_dosage: Dosage,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_duration: Duration,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_expression: Expression,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_extended_contact_detail: ExtendedContactDetail,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_human_name: HumanName,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_id: String,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_identifier: Identifier,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_instant: String,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_integer: i64,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_integer_64: String,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_markdown: String,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_meta: Meta,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_money: Money,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_oid: String,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_parameter_definition: ParameterDefinition,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_period: Period,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_positive_int: u64,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_quantity: Quantity,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_range: Range,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_ratio: Ratio,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_ratio_range: RatioRange,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_reference: Reference,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_related_artifact: RelatedArtifact,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_sampled_data: SampledData,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_signature: Signature,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_string: String,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_time: String,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_timing: Timing,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_trigger_definition: TriggerDefinition,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_unsigned_int: u64,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_uri: String,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_url: String,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_usage_context: UsageContext,
/// The actual value for the element, which must be one of the types allowed for
/// this element.
pub value_uuid: String,
}
