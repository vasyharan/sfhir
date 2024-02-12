use super::*;
/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug,Clone,PartialEq)]
pub struct ElementDefinition {
/// Identifies additional names by which this element might also be known.
pub alias: Vec<String>,
/// Information about the base definition of the element, provided to make it
/// unnecessary for tools to trace the deviation of the element through the derived
/// and related profiles. When the element definition is not the original definition
/// of an element - e.g. either in a constraint on another type, or for elements
/// from a super type in a snap shot - then the information in provided in the
/// element definition may be different to the base definition. On the original
/// definition of the element, it will be same.
pub base: ElementDefinitionBase,
/// Binds to a value set if this element is coded (code, Coding, CodeableConcept,
/// Quantity), or the data types (string, uri).
pub binding: ElementDefinitionBinding,
/// A code that has the same meaning as the element in a particular terminology.
pub code: Vec<Coding>,
/// Explanatory notes and implementation guidance about the data element, including
/// notes about how to use the data properly, exceptions to proper use, etc. (Note:
/// The text you are reading is specified in ElementDefinition.comment).
pub comment: Markdown,
/// A reference to an invariant that may make additional statements about the
/// cardinality or value in the instance.
pub condition: Vec<Id>,
/// Formal constraints such as co-occurrence and other constraints that can be
/// computationally evaluated within the context of the instance.
pub constraint: Vec<ElementDefinitionConstraint>,
/// Identifies an element defined elsewhere in the definition whose content rules
/// should be applied to the current element. ContentReferences bring across all the
/// rules that are in the ElementDefinition for the element, including definitions,
/// cardinality constraints, bindings, invariants etc.
pub content_reference: Uri,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_address: Address,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_age: Age,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_annotation: Annotation,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_attachment: Attachment,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_availability: Availability,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_base_64_binary: String,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_boolean: bool,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_canonical: String,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_code: String,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_codeable_concept: CodeableConcept,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_codeable_reference: CodeableReference,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_coding: Coding,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_contact_detail: ContactDetail,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_contact_point: ContactPoint,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_count: Count,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_data_requirement: DataRequirement,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_date: String,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_date_time: String,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_decimal: f64,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_distance: Distance,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_dosage: Dosage,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_duration: Duration,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_expression: Expression,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_extended_contact_detail: ExtendedContactDetail,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_human_name: HumanName,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_id: String,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_identifier: Identifier,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_instant: String,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_integer: i64,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_integer_64: String,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_markdown: String,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_meta: Meta,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_money: Money,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_oid: String,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_parameter_definition: ParameterDefinition,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_period: Period,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_positive_int: u64,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_quantity: Quantity,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_range: Range,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_ratio: Ratio,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_ratio_range: RatioRange,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_reference: Reference,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_related_artifact: RelatedArtifact,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_sampled_data: SampledData,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_signature: Signature,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_string: String,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_time: String,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_timing: Timing,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_trigger_definition: TriggerDefinition,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_unsigned_int: u64,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_uri: String,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_url: String,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_usage_context: UsageContext,
/// The value that should be used if there is no value stated in the instance (e.g.
/// 'if not otherwise specified, the abstract is false').
pub default_value_uuid: String,
/// Provides a complete explanation of the meaning of the data element for
/// human readability.  For the case of elements derived from existing elements
/// (e.g. constraints), the definition SHALL be consistent with the base
/// definition, but convey the meaning of the element in the particular context
/// of use of the resource. (Note: The text you are reading is specified in
/// ElementDefinition.definition).
pub definition: Markdown,
/// A sample value for this element demonstrating the type of information that would
/// typically be found in the element.
pub example: Vec<ElementDefinitionExample>,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_address: Address,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_age: Age,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_annotation: Annotation,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_attachment: Attachment,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_availability: Availability,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_base_64_binary: String,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_boolean: bool,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_canonical: String,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_code: String,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_codeable_concept: CodeableConcept,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_codeable_reference: CodeableReference,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_coding: Coding,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_contact_detail: ContactDetail,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_contact_point: ContactPoint,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_count: Count,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_data_requirement: DataRequirement,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_date: String,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_date_time: String,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_decimal: f64,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_distance: Distance,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_dosage: Dosage,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_duration: Duration,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_expression: Expression,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_extended_contact_detail: ExtendedContactDetail,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_human_name: HumanName,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_id: String,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_identifier: Identifier,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_instant: String,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_integer: i64,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_integer_64: String,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_markdown: String,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_meta: Meta,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_money: Money,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_oid: String,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_parameter_definition: ParameterDefinition,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_period: Period,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_positive_int: u64,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_quantity: Quantity,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_range: Range,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_ratio: Ratio,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_ratio_range: RatioRange,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_reference: Reference,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_related_artifact: RelatedArtifact,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_sampled_data: SampledData,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_signature: Signature,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_string: String,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_time: String,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_timing: Timing,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_trigger_definition: TriggerDefinition,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_unsigned_int: u64,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_uri: String,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_url: String,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_usage_context: UsageContext,
/// Specifies a value that SHALL be exactly the value  for this element in the
/// instance, if present. For purposes of comparison, non-significant whitespace
/// is ignored, and all values must be an exact match (case and accent sensitive).
/// Missing elements/attributes must also be missing.
pub fixed_uuid: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// If true, the value of this element affects the interpretation of the element
/// or resource that contains it, and the value of the element cannot be ignored.
/// Typically, this is used for status, negation and qualification codes. The
/// effect of this is that the element cannot be ignored by systems: they SHALL
/// either recognize the element and process it, and/or a pre-determination has been
/// made that it is not relevant to their particular system. When used on the root
/// element in an extension definition, this indicates whether or not the extension
/// is a modifier extension.
pub is_modifier: Boolean,
/// Explains how that element affects the interpretation of the resource or element
/// that contains it.
pub is_modifier_reason: String,
/// Whether the element should be included if a client requests a search with the
/// parameter _summary=true.
pub is_summary: Boolean,
/// A single preferred label which is the text to display beside the element
/// indicating its meaning or to use to prompt for the element in a user display
/// or form.
pub label: String,
/// Identifies a concept from an external specification that roughly corresponds to
/// this element.
pub mapping: Vec<ElementDefinitionMapping>,
/// The maximum number of times this element is permitted to appear in the instance.
pub max: String,
/// Indicates the maximum length in characters that is permitted to be present
/// in conformant instances and which is expected to be supported by conformant
/// consumers that support the element. ```maxLength``` SHOULD only be used on
/// primitive data types that have a string representation (see [[[http://hl7.org/
/// fhir/StructureDefinition/structuredefinition-type-characteristics]]]).
pub max_length: Integer,
/// The maximum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub max_value_date: String,
/// The maximum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub max_value_date_time: String,
/// The maximum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub max_value_decimal: f64,
/// The maximum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub max_value_instant: String,
/// The maximum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub max_value_integer: i64,
/// The maximum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub max_value_integer_64: String,
/// The maximum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub max_value_positive_int: u64,
/// The maximum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub max_value_quantity: Quantity,
/// The maximum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub max_value_time: String,
/// The maximum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub max_value_unsigned_int: u64,
/// The Implicit meaning that is to be understood when this element is missing (e.g.
/// 'when this element is missing, the period is ongoing').
pub meaning_when_missing: Markdown,
/// The minimum number of times this element SHALL appear in the instance.
pub min: UnsignedInt,
/// The minimum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub min_value_date: String,
/// The minimum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub min_value_date_time: String,
/// The minimum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub min_value_decimal: f64,
/// The minimum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub min_value_instant: String,
/// The minimum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub min_value_integer: i64,
/// The minimum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub min_value_integer_64: String,
/// The minimum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub min_value_positive_int: u64,
/// The minimum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub min_value_quantity: Quantity,
/// The minimum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub min_value_time: String,
/// The minimum allowed value for the element. The value is inclusive. This is
/// allowed for the types date, dateTime, instant, time, decimal, integer, and
/// Quantity.
pub min_value_unsigned_int: u64,
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
/// Specifies for a primitive data type that the value of the data type cannot be
/// replaced by an extension.
pub must_have_value: Boolean,
/// If true, implementations that produce or consume resources SHALL provide
/// "support" for the element in some meaningful way. Note that this is being phased
/// out and replaced by obligations (see below).  If false, the element may be
/// ignored and not supported. If false, whether to populate or use the data element
/// in any way is at the discretion of the implementation.
pub must_support: Boolean,
/// If present, indicates that the order of the repeating element has meaning and
/// describes what that meaning is.  If absent, it means that the order of the
/// element has no meaning.
pub order_meaning: String,
/// The path identifies the element and is expressed as a "."-separated list of
/// ancestor elements, beginning with the name of the resource or extension.
pub path: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_address: Address,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_age: Age,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_annotation: Annotation,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_attachment: Attachment,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_availability: Availability,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_base_64_binary: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_boolean: bool,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_canonical: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_code: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_codeable_concept: CodeableConcept,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_codeable_reference: CodeableReference,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_coding: Coding,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_contact_detail: ContactDetail,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_contact_point: ContactPoint,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_count: Count,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_data_requirement: DataRequirement,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_date: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_date_time: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_decimal: f64,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_distance: Distance,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_dosage: Dosage,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_duration: Duration,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_expression: Expression,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_extended_contact_detail: ExtendedContactDetail,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_human_name: HumanName,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_id: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_identifier: Identifier,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_instant: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_integer: i64,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_integer_64: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_markdown: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_meta: Meta,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_money: Money,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_oid: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_parameter_definition: ParameterDefinition,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_period: Period,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_positive_int: u64,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_quantity: Quantity,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_range: Range,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_ratio: Ratio,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_ratio_range: RatioRange,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_reference: Reference,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_related_artifact: RelatedArtifact,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_sampled_data: SampledData,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_signature: Signature,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_string: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_time: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_timing: Timing,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_trigger_definition: TriggerDefinition,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_unsigned_int: u64,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_uri: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_url: String,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_usage_context: UsageContext,
/// Specifies a value that each occurrence of the element in the instance SHALL
/// follow - that is, any value in the pattern must be found in the instance, if
/// the element has a value. Other additional values may be found too. This is
/// effectively constraint by example.
///
/// When pattern[x] is used to constrain a primitive, it means that the value
/// provided in the pattern[x] must match the instance value exactly.
///
/// When an element within a pattern[x] is used to constrain an array, it means that
/// each element provided in the pattern[x] must (recursively) match at least one
/// element from the instance array.
///
/// When pattern[x] is used to constrain a complex object, it means that each
/// property in the pattern must be present in the complex object, and its value
/// must recursively match -- i.e.,
///
/// 1. If primitive: it must match exactly the pattern value
/// 2. If a complex object: it must match (recursively) the pattern value
/// 3. If an array: it must match (recursively) the pattern value
///
/// If a pattern[x] is declared on a repeating element, the pattern applies to
/// all repetitions.  If the desire is for a pattern to apply to only one element
/// or a subset of elements, slicing must be used. See [Examples of Patterns]
/// (elementdefinition-examples.html#pattern-examples) for examples of pattern usage
/// and the effect it will have.
pub pattern_uuid: String,
/// Codes that define how this element is represented in instances, when the
/// deviation varies from the normal case. No extensions are allowed on elements
/// with a representation of 'xmlAttr', no matter what FHIR serialization format
/// is used.
pub representation: Vec<Representation>,
/// This element is for traceability of why the element was created and why the
/// constraints exist as they do. This may be used to point to source materials or
/// specifications that drove the structure of this element.
pub requirements: Markdown,
/// A concise description of what this element means (e.g. for use in autogenerated
/// summaries).
pub short: String,
/// If true, indicates that this slice definition is constraining a slice
/// definition with the same name in an inherited profile. If false, the slice is
/// not overriding any slice in an inherited profile. If missing, the slice might
/// or might not be overriding a slice in an inherited profile, depending on the
/// sliceName.
pub slice_is_constraining: Boolean,
/// The name of this element definition slice, when slicing is working. The name
/// must be a token with no dots or spaces. This is a unique name referring to a
/// specific set of constraints applied to this element, used to provide a name to
/// different slices of the same element.
pub slice_name: String,
/// Indicates that the element is sliced into a set of alternative definitions
/// (i.e. in a structure definition, there are multiple different constraints on
/// a single element in the base resource). Slicing can be used in any resource
/// that has cardinality ..* on the base resource, or any resource with a choice
/// of types. The set of slices is any elements that come after this in the element
/// sequence that have the same path, until a shorter path occurs (the shorter path
/// terminates the set).
pub slicing: ElementDefinitionSlicing,
/// The data type or resource that the value of this element is permitted to be.
pub r#type: Vec<ElementDefinitionType>,
/// Specifies a list of extensions that can appear in place of a primitive value.
pub value_alternatives: Vec<Canonical>,
}
