/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinition {
    /// Identifies additional names by which this element might also be known.
    pub alias: Vec<super::string::String>,
    /// Information about the base definition of the element, provided to make it
    /// unnecessary for tools to trace the deviation of the element through the derived
    /// and related profiles. When the element definition is not the original definition
    /// of an element - e.g. either in a constraint on another type, or for elements
    /// from a super type in a snap shot - then the information in provided in the
    /// element definition may be different to the base definition. On the original
    /// definition of the element, it will be same.
    pub base: super::element_definition::ElementDefinitionBase,
    /// Binds to a value set if this element is coded (code, Coding, CodeableConcept,
    /// Quantity), or the data types (string, uri).
    pub binding: super::element_definition::ElementDefinitionBinding,
    /// A code that has the same meaning as the element in a particular terminology.
    pub code: Vec<super::coding::Coding>,
    /// Explanatory notes and implementation guidance about the data element, including
    /// notes about how to use the data properly, exceptions to proper use, etc. (Note:
    /// The text you are reading is specified in ElementDefinition.comment).
    pub comment: super::markdown::Markdown,
    /// A reference to an invariant that may make additional statements about the
    /// cardinality or value in the instance.
    pub condition: Vec<super::id::Id>,
    /// Formal constraints such as co-occurrence and other constraints that can be
    /// computationally evaluated within the context of the instance.
    pub constraint: Vec<super::element_definition::ElementDefinitionConstraint>,
    /// Identifies an element defined elsewhere in the definition whose content rules
    /// should be applied to the current element. ContentReferences bring across all the
    /// rules that are in the ElementDefinition for the element, including definitions,
    /// cardinality constraints, bindings, invariants etc.
    pub content_reference: super::uri::Uri,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_address: super::address::Address,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_age: super::age::Age,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_annotation: super::annotation::Annotation,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_attachment: super::attachment::Attachment,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_availability: super::availability::Availability,
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
    pub default_value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_codeable_reference: super::codeable_reference::CodeableReference,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_coding: super::coding::Coding,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_contact_detail: super::contact_detail::ContactDetail,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_contact_point: super::contact_point::ContactPoint,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_count: super::count::Count,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_data_requirement: super::data_requirement::DataRequirement,
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
    pub default_value_distance: super::distance::Distance,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_dosage: super::dosage::Dosage,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_duration: super::duration::Duration,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_expression: super::expression::Expression,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_extended_contact_detail:
        super::extended_contact_detail::ExtendedContactDetail,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_human_name: super::human_name::HumanName,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_id: String,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_identifier: super::identifier::Identifier,
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
    pub default_value_meta: super::meta::Meta,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_money: super::money::Money,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_oid: String,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_parameter_definition: super::parameter_definition::ParameterDefinition,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_period: super::period::Period,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_positive_int: u64,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_quantity: super::quantity::Quantity,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_range: super::range::Range,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_ratio: super::ratio::Ratio,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_ratio_range: super::ratio_range::RatioRange,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_reference: super::reference::Reference,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_related_artifact: super::related_artifact::RelatedArtifact,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_sampled_data: super::sampled_data::SampledData,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_signature: super::signature::Signature,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_string: String,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_time: String,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_timing: super::timing::Timing,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_trigger_definition: super::trigger_definition::TriggerDefinition,
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
    pub default_value_usage_context: super::usage_context::UsageContext,
    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub default_value_uuid: String,
    /// Provides a complete explanation of the meaning of the data element for
    /// human readability.  For the case of elements derived from existing elements
    /// (e.g. constraints), the definition SHALL be consistent with the base
    /// definition, but convey the meaning of the element in the particular context
    /// of use of the resource. (Note: The text you are reading is specified in
    /// ElementDefinition.definition).
    pub definition: super::markdown::Markdown,
    /// A sample value for this element demonstrating the type of information that would
    /// typically be found in the element.
    pub example: Vec<super::element_definition::ElementDefinitionExample>,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_address: super::address::Address,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_age: super::age::Age,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_annotation: super::annotation::Annotation,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_attachment: super::attachment::Attachment,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_availability: super::availability::Availability,
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
    pub fixed_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_codeable_reference: super::codeable_reference::CodeableReference,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_coding: super::coding::Coding,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_contact_detail: super::contact_detail::ContactDetail,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_contact_point: super::contact_point::ContactPoint,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_count: super::count::Count,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_data_requirement: super::data_requirement::DataRequirement,
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
    pub fixed_distance: super::distance::Distance,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_dosage: super::dosage::Dosage,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_duration: super::duration::Duration,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_expression: super::expression::Expression,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_extended_contact_detail: super::extended_contact_detail::ExtendedContactDetail,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_human_name: super::human_name::HumanName,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_id: String,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_identifier: super::identifier::Identifier,
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
    pub fixed_meta: super::meta::Meta,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_money: super::money::Money,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_oid: String,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_parameter_definition: super::parameter_definition::ParameterDefinition,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_period: super::period::Period,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_positive_int: u64,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_quantity: super::quantity::Quantity,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_range: super::range::Range,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_ratio: super::ratio::Ratio,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_ratio_range: super::ratio_range::RatioRange,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_reference: super::reference::Reference,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_related_artifact: super::related_artifact::RelatedArtifact,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_sampled_data: super::sampled_data::SampledData,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_signature: super::signature::Signature,
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
    pub fixed_timing: super::timing::Timing,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_trigger_definition: super::trigger_definition::TriggerDefinition,
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
    pub fixed_usage_context: super::usage_context::UsageContext,
    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance, if present. For purposes of comparison, non-significant whitespace
    /// is ignored, and all values must be an exact match (case and accent sensitive).
    /// Missing elements/attributes must also be missing.
    pub fixed_uuid: String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// If true, the value of this element affects the interpretation of the element
    /// or resource that contains it, and the value of the element cannot be ignored.
    /// Typically, this is used for status, negation and qualification codes. The
    /// effect of this is that the element cannot be ignored by systems: they SHALL
    /// either recognize the element and process it, and/or a pre-determination has been
    /// made that it is not relevant to their particular system. When used on the root
    /// element in an extension definition, this indicates whether or not the extension
    /// is a modifier extension.
    pub is_modifier: super::boolean::Boolean,
    /// Explains how that element affects the interpretation of the resource or element
    /// that contains it.
    pub is_modifier_reason: super::string::String,
    /// Whether the element should be included if a client requests a search with the
    /// parameter _summary=true.
    pub is_summary: super::boolean::Boolean,
    /// A single preferred label which is the text to display beside the element
    /// indicating its meaning or to use to prompt for the element in a user display
    /// or form.
    pub label: super::string::String,
    /// Identifies a concept from an external specification that roughly corresponds to
    /// this element.
    pub mapping: Vec<super::element_definition::ElementDefinitionMapping>,
    /// The maximum number of times this element is permitted to appear in the instance.
    pub max: super::string::String,
    /// Indicates the maximum length in characters that is permitted to be present
    /// in conformant instances and which is expected to be supported by conformant
    /// consumers that support the element. ```maxLength``` SHOULD only be used on
    /// primitive data types that have a string representation (see [[[http://hl7.org/
    /// fhir/StructureDefinition/structuredefinition-type-characteristics]]]).
    pub max_length: super::integer::Integer,
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
    pub max_value_quantity: super::quantity::Quantity,
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
    pub meaning_when_missing: super::markdown::Markdown,
    /// The minimum number of times this element SHALL appear in the instance.
    pub min: super::unsigned_int::UnsignedInt,
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
    pub min_value_quantity: super::quantity::Quantity,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// Specifies for a primitive data type that the value of the data type cannot be
    /// replaced by an extension.
    pub must_have_value: super::boolean::Boolean,
    /// If true, implementations that produce or consume resources SHALL provide
    /// "support" for the element in some meaningful way. Note that this is being phased
    /// out and replaced by obligations (see below).  If false, the element may be
    /// ignored and not supported. If false, whether to populate or use the data element
    /// in any way is at the discretion of the implementation.
    pub must_support: super::boolean::Boolean,
    /// If present, indicates that the order of the repeating element has meaning and
    /// describes what that meaning is.  If absent, it means that the order of the
    /// element has no meaning.
    pub order_meaning: super::string::String,
    /// The path identifies the element and is expressed as a "."-separated list of
    /// ancestor elements, beginning with the name of the resource or extension.
    pub path: super::string::String,
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
    pub pattern_address: super::address::Address,
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
    pub pattern_age: super::age::Age,
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
    pub pattern_annotation: super::annotation::Annotation,
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
    pub pattern_attachment: super::attachment::Attachment,
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
    pub pattern_availability: super::availability::Availability,
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
    pub pattern_codeable_concept: super::codeable_concept::CodeableConcept,
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
    pub pattern_codeable_reference: super::codeable_reference::CodeableReference,
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
    pub pattern_coding: super::coding::Coding,
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
    pub pattern_contact_detail: super::contact_detail::ContactDetail,
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
    pub pattern_contact_point: super::contact_point::ContactPoint,
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
    pub pattern_count: super::count::Count,
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
    pub pattern_data_requirement: super::data_requirement::DataRequirement,
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
    pub pattern_distance: super::distance::Distance,
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
    pub pattern_dosage: super::dosage::Dosage,
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
    pub pattern_duration: super::duration::Duration,
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
    pub pattern_expression: super::expression::Expression,
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
    pub pattern_extended_contact_detail: super::extended_contact_detail::ExtendedContactDetail,
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
    pub pattern_human_name: super::human_name::HumanName,
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
    pub pattern_identifier: super::identifier::Identifier,
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
    pub pattern_meta: super::meta::Meta,
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
    pub pattern_money: super::money::Money,
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
    pub pattern_parameter_definition: super::parameter_definition::ParameterDefinition,
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
    pub pattern_period: super::period::Period,
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
    pub pattern_quantity: super::quantity::Quantity,
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
    pub pattern_range: super::range::Range,
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
    pub pattern_ratio: super::ratio::Ratio,
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
    pub pattern_ratio_range: super::ratio_range::RatioRange,
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
    pub pattern_reference: super::reference::Reference,
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
    pub pattern_related_artifact: super::related_artifact::RelatedArtifact,
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
    pub pattern_sampled_data: super::sampled_data::SampledData,
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
    pub pattern_signature: super::signature::Signature,
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
    pub pattern_timing: super::timing::Timing,
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
    pub pattern_trigger_definition: super::trigger_definition::TriggerDefinition,
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
    pub pattern_usage_context: super::usage_context::UsageContext,
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
    pub requirements: super::markdown::Markdown,
    /// A concise description of what this element means (e.g. for use in autogenerated
    /// summaries).
    pub short: super::string::String,
    /// If true, indicates that this slice definition is constraining a slice
    /// definition with the same name in an inherited profile. If false, the slice is
    /// not overriding any slice in an inherited profile. If missing, the slice might
    /// or might not be overriding a slice in an inherited profile, depending on the
    /// sliceName.
    pub slice_is_constraining: super::boolean::Boolean,
    /// The name of this element definition slice, when slicing is working. The name
    /// must be a token with no dots or spaces. This is a unique name referring to a
    /// specific set of constraints applied to this element, used to provide a name to
    /// different slices of the same element.
    pub slice_name: super::string::String,
    /// Indicates that the element is sliced into a set of alternative definitions
    /// (i.e. in a structure definition, there are multiple different constraints on
    /// a single element in the base resource). Slicing can be used in any resource
    /// that has cardinality ..* on the base resource, or any resource with a choice
    /// of types. The set of slices is any elements that come after this in the element
    /// sequence that have the same path, until a shorter path occurs (the shorter path
    /// terminates the set).
    pub slicing: super::element_definition::ElementDefinitionSlicing,
    /// The data type or resource that the value of this element is permitted to be.
    pub r#type: Vec<super::element_definition::ElementDefinitionType>,
    /// Specifies a list of extensions that can appear in place of a primitive value.
    pub value_alternatives: Vec<super::canonical::Canonical>,
}

/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionAdditional {
    /// Whether the binding applies to all repeats, or just to any one of them. This is
    /// only relevant for elements that can repeat.
    pub any: super::boolean::Boolean,
    /// Documentation of the purpose of use of the bindingproviding additional
    /// information about how it is intended to be used.
    pub documentation: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// The use of this additional binding.
    pub purpose: super::code::Code,
    /// Concise documentation - for summary tables.
    pub short_doco: super::string::String,
    /// Qualifies the usage of the binding. Typically bindings are qualified by
    /// jurisdiction, but they may also be qualified by gender, workflow status,
    /// clinical domain etc. The information to decide whether a usege context applies
    /// is usually outside the resource, determined by context, and this might present
    /// challenges for validation tooling.
    pub usage: Vec<super::usage_context::UsageContext>,
    /// The valueSet that is being bound for the purpose.
    pub value_set: super::canonical::Canonical,
}

/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionBase {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Maximum cardinality of the base element identified by the path.
    pub max: super::string::String,
    /// Minimum cardinality of the base element identified by the path.
    pub min: super::unsigned_int::UnsignedInt,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// The Path that identifies the base element - this matches the
    /// ElementDefinition.path for that element. Across FHIR, there is only one
    /// base definition of any element - that is, an element definition on a
    /// [[[StructureDefinition]]] without a StructureDefinition.base.
    pub path: super::string::String,
}

/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionBinding {
    /// Additional bindings that help applications implementing this element. Additional
    /// bindings do not replace the main binding but provide more information and/
    /// or context.
    pub additional: Vec<super::element_definition::ElementDefinitionAdditional>,
    /// Describes the intended use of this particular set of codes.
    pub description: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// Indicates the degree of conformance expectations associated with this binding
    /// - that is, the degree to which the provided value set must be adhered to in
    /// the instances.
    pub strength: Strength,
    /// Refers to the value set that identifies the set of codes the binding refers to.
    pub value_set: super::canonical::Canonical,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Strength {
    Required,
    Extensible,
    Preferred,
    Example,
}

/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionConstraint {
    /// A [FHIRPath](fhirpath.html) expression of constraint that can be executed to see
    /// if this constraint is met.
    pub expression: super::string::String,
    /// Text that can be used to describe the constraint in messages identifying that
    /// the constraint has been violated.
    pub human: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Allows identification of which elements have their cardinalities impacted by
    /// the constraint.  Will not be referenced for constraints that do not affect
    /// cardinality.
    pub key: super::id::Id,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// Description of why this constraint is necessary or appropriate.
    pub requirements: super::markdown::Markdown,
    /// Identifies the impact constraint violation has on the conformance of the
    /// instance.
    pub severity: Severity,
    /// A reference to the original source of the constraint, for traceability purposes.
    pub source: super::canonical::Canonical,
    /// If true, indicates that the warning or best practice guideline should be
    /// suppressed.
    pub suppress: super::boolean::Boolean,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Error,
    Warning,
}

/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionDiscriminator {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// A FHIRPath expression, using [the simple subset of FHIRPath]
    /// (fhirpath.html#simple), that is used to identify the element on which
    /// discrimination is based.
    pub path: super::string::String,
    /// How the element value is interpreted when discrimination is evaluated.
    pub r#type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Value,
    Exists,
    Pattern,
    Type,
    Profile,
    Position,
}

/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionExample {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Describes the purpose of this example among the set of examples.
    pub label: super::string::String,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_address: super::address::Address,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_age: super::age::Age,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_annotation: super::annotation::Annotation,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_attachment: super::attachment::Attachment,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_availability: super::availability::Availability,
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
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_codeable_reference: super::codeable_reference::CodeableReference,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_coding: super::coding::Coding,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_contact_detail: super::contact_detail::ContactDetail,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_contact_point: super::contact_point::ContactPoint,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_count: super::count::Count,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_data_requirement: super::data_requirement::DataRequirement,
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
    pub value_distance: super::distance::Distance,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_dosage: super::dosage::Dosage,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_duration: super::duration::Duration,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_expression: super::expression::Expression,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_extended_contact_detail: super::extended_contact_detail::ExtendedContactDetail,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_human_name: super::human_name::HumanName,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_id: String,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_identifier: super::identifier::Identifier,
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
    pub value_meta: super::meta::Meta,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_money: super::money::Money,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_oid: String,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_parameter_definition: super::parameter_definition::ParameterDefinition,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_period: super::period::Period,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_positive_int: u64,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_quantity: super::quantity::Quantity,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_range: super::range::Range,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_ratio: super::ratio::Ratio,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_ratio_range: super::ratio_range::RatioRange,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_reference: super::reference::Reference,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_related_artifact: super::related_artifact::RelatedArtifact,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_sampled_data: super::sampled_data::SampledData,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_signature: super::signature::Signature,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_string: String,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_time: String,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_timing: super::timing::Timing,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_trigger_definition: super::trigger_definition::TriggerDefinition,
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
    pub value_usage_context: super::usage_context::UsageContext,
    /// The actual value for the element, which must be one of the types allowed for
    /// this element.
    pub value_uuid: String,
}

/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionMapping {
    /// Comments that provide information about the mapping or its use.
    pub comment: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// An internal reference to the definition of a mapping.
    pub identity: super::id::Id,
    /// Identifies the computable language in which mapping.map is expressed.
    pub language: super::code::Code,
    /// Expresses what part of the target specification corresponds to this element.
    pub map: super::string::String,
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
    pub modifier_extension: Vec<super::extension::Extension>,
}

/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionSlicing {
    /// A human-readable text description of how the slicing works. If there is no
    /// discriminator, this is required to be present to provide whatever information is
    /// possible about how the slices can be differentiated.
    pub description: super::string::String,
    /// Designates which child elements are used to discriminate between the slices when
    /// processing an instance. If one or more discriminators are provided, the value
    /// of the child elements in the instance data SHALL completely distinguish which
    /// slice the element in the resource matches based on the allowed values for those
    /// elements in each of the slices.
    pub discriminator: Vec<super::element_definition::ElementDefinitionDiscriminator>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// If the matching elements have to occur in the same order as defined in the
    /// profile.
    pub ordered: super::boolean::Boolean,
    /// Whether additional slices are allowed or not. When the slices are ordered,
    /// profile authors can also say that additional slices are only allowed at the end.
    pub rules: Rules,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rules {
    Closed,
    Open,
    OpenAtEnd,
}

/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementDefinitionType {
    /// If the type is a reference to another resource, how the resource is or can be
    /// aggregated - is it a contained resource, or a reference, and if the context is a
    /// bundle, is it included in the bundle.
    pub aggregation: Vec<Aggregation>,
    /// URL of Data type or Resource that is a(or the) type used for this element.
    /// References are URLs that are relative to http://hl7.org/fhir/StructureDefinition
    /// e.g. "string" is a reference to http://hl7.org/fhir/StructureDefinition/string.
    /// Absolute URLs are only allowed in logical models.
    pub code: super::uri::Uri,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// Identifies a profile structure or implementation Guide that applies to the
    /// datatype this element refers to. If any profiles are specified, then the content
    /// must conform to at least one of them. The URL can be a local reference - to a
    /// contained StructureDefinition, or a reference to another StructureDefinition
    /// or Implementation Guide by a canonical URL. When an implementation guide
    /// is specified, the type SHALL conform to at least one profile defined in the
    /// implementation guide.
    pub profile: Vec<super::canonical::Canonical>,
    /// Used when the type is "Reference" or "canonical", and identifies a profile
    /// structure or implementation Guide that applies to the target of the reference
    /// this element refers to. If any profiles are specified, then the content must
    /// conform to at least one of them. The URL can be a local reference - to a
    /// contained StructureDefinition, or a reference to another StructureDefinition
    /// or Implementation Guide by a canonical URL. When an implementation guide is
    /// specified, the target resource SHALL conform to at least one profile defined in
    /// the implementation guide.
    pub target_profile: Vec<super::canonical::Canonical>,
    /// Whether this reference needs to be version specific or version independent, or
    /// whether either can be used.
    pub versioning: Versioning,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Versioning {
    Either,
    Independent,
    Specific,
}
