/// Optional Extension Element - found in all resources.
#[derive(Debug, Clone, PartialEq)]
pub struct Extension {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Source of the definition for the extension code - a logical name or a URL.
    pub url: super::uri::Uri,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_address: super::address::Address,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_age: super::age::Age,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_annotation: super::annotation::Annotation,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_attachment: super::attachment::Attachment,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_availability: super::availability::Availability,
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
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_codeable_reference: super::codeable_reference::CodeableReference,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_coding: super::coding::Coding,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_contact_detail: super::contact_detail::ContactDetail,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_contact_point: super::contact_point::ContactPoint,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_count: super::count::Count,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_data_requirement: super::data_requirement::DataRequirement,
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
    pub value_distance: super::distance::Distance,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_dosage: super::dosage::Dosage,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_duration: super::duration::Duration,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_expression: super::expression::Expression,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_extended_contact_detail: super::extended_contact_detail::ExtendedContactDetail,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_human_name: super::human_name::HumanName,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_id: String,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_identifier: super::identifier::Identifier,
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
    pub value_meta: super::meta::Meta,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_money: super::money::Money,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_oid: String,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_parameter_definition: super::parameter_definition::ParameterDefinition,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_period: super::period::Period,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_positive_int: u64,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_quantity: super::quantity::Quantity,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_range: super::range::Range,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_ratio: super::ratio::Ratio,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_ratio_range: super::ratio_range::RatioRange,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_reference: super::reference::Reference,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_related_artifact: super::related_artifact::RelatedArtifact,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_sampled_data: super::sampled_data::SampledData,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_signature: super::signature::Signature,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_string: String,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_time: String,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_timing: super::timing::Timing,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_trigger_definition: super::trigger_definition::TriggerDefinition,
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
    pub value_usage_context: super::usage_context::UsageContext,
    /// Value of extension - must be one of a constrained set of the data types (see
    /// [Extensibility](extensibility.html) for a list).
    pub value_uuid: String,
}
