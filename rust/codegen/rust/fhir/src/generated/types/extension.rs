use super::*;
/// Optional Extension Element - found in all resources.
#[derive(Debug,Clone,PartialEq)]
pub struct Extension {
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Source of the definition for the extension code - a logical name or a URL.
pub url: Uri,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_address: Address,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_age: Age,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_annotation: Annotation,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_attachment: Attachment,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_availability: Availability,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_base_64_binary: String,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_boolean: bool,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_canonical: String,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_code: String,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_codeable_concept: CodeableConcept,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_codeable_reference: CodeableReference,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_coding: Coding,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_contact_detail: ContactDetail,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_contact_point: ContactPoint,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_count: Count,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_data_requirement: DataRequirement,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_date: String,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_date_time: String,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_decimal: f64,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_distance: Distance,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_dosage: Dosage,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_duration: Duration,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_expression: Expression,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_extended_contact_detail: ExtendedContactDetail,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_human_name: HumanName,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_id: String,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_identifier: Identifier,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_instant: String,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_integer: i64,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_integer_64: String,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_markdown: String,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_meta: Meta,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_money: Money,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_oid: String,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_parameter_definition: ParameterDefinition,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_period: Period,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_positive_int: u64,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_quantity: Quantity,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_range: Range,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_ratio: Ratio,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_ratio_range: RatioRange,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_reference: Reference,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_related_artifact: RelatedArtifact,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_sampled_data: SampledData,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_signature: Signature,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_string: String,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_time: String,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_timing: Timing,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_trigger_definition: TriggerDefinition,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_unsigned_int: u64,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_uri: String,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_url: String,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_usage_context: UsageContext,
/// Value of extension - must be one of a constrained set of the data types (see
/// [Extensibility](extensibility.html) for a list).
pub value_uuid: String,
}
