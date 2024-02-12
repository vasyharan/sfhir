/// This resource is used to pass information into and back from an operation
/// (whether invoked directly from REST or within a messaging environment).  It is
/// not persisted or allowed to be referenced by other resources.
#[derive(Debug, Clone, PartialEq)]
pub struct Parameters {
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// A parameter passed to or received from the operation.
    pub parameter: Vec<super::parameters::ParametersParameter>,
    /// This is a Parameters resource
    pub resource_type: String,
}

/// This resource is used to pass information into and back from an operation
/// (whether invoked directly from REST or within a messaging environment).  It is
/// not persisted or allowed to be referenced by other resources.
#[derive(Debug, Clone, PartialEq)]
pub struct ParametersParameter {
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
    /// The name of the parameter (reference to the operation definition).
    pub name: super::string::String,
    /// A named part of a multi-part parameter.
    pub part: Vec<super::parameters::ParametersParameter>,
    /// Conveys the content if the parameter is a whole resource.
    pub resource: super::resource_list::ResourceList,
    /// Conveys the content if the parameter is a data type.
    pub value_address: super::address::Address,
    /// Conveys the content if the parameter is a data type.
    pub value_age: super::age::Age,
    /// Conveys the content if the parameter is a data type.
    pub value_annotation: super::annotation::Annotation,
    /// Conveys the content if the parameter is a data type.
    pub value_attachment: super::attachment::Attachment,
    /// Conveys the content if the parameter is a data type.
    pub value_availability: super::availability::Availability,
    /// Conveys the content if the parameter is a data type.
    pub value_base_64_binary: String,
    /// Conveys the content if the parameter is a data type.
    pub value_boolean: bool,
    /// Conveys the content if the parameter is a data type.
    pub value_canonical: String,
    /// Conveys the content if the parameter is a data type.
    pub value_code: String,
    /// Conveys the content if the parameter is a data type.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Conveys the content if the parameter is a data type.
    pub value_codeable_reference: super::codeable_reference::CodeableReference,
    /// Conveys the content if the parameter is a data type.
    pub value_coding: super::coding::Coding,
    /// Conveys the content if the parameter is a data type.
    pub value_contact_detail: super::contact_detail::ContactDetail,
    /// Conveys the content if the parameter is a data type.
    pub value_contact_point: super::contact_point::ContactPoint,
    /// Conveys the content if the parameter is a data type.
    pub value_count: super::count::Count,
    /// Conveys the content if the parameter is a data type.
    pub value_data_requirement: super::data_requirement::DataRequirement,
    /// Conveys the content if the parameter is a data type.
    pub value_date: String,
    /// Conveys the content if the parameter is a data type.
    pub value_date_time: String,
    /// Conveys the content if the parameter is a data type.
    pub value_decimal: f64,
    /// Conveys the content if the parameter is a data type.
    pub value_distance: super::distance::Distance,
    /// Conveys the content if the parameter is a data type.
    pub value_dosage: super::dosage::Dosage,
    /// Conveys the content if the parameter is a data type.
    pub value_duration: super::duration::Duration,
    /// Conveys the content if the parameter is a data type.
    pub value_expression: super::expression::Expression,
    /// Conveys the content if the parameter is a data type.
    pub value_extended_contact_detail: super::extended_contact_detail::ExtendedContactDetail,
    /// Conveys the content if the parameter is a data type.
    pub value_human_name: super::human_name::HumanName,
    /// Conveys the content if the parameter is a data type.
    pub value_id: String,
    /// Conveys the content if the parameter is a data type.
    pub value_identifier: super::identifier::Identifier,
    /// Conveys the content if the parameter is a data type.
    pub value_instant: String,
    /// Conveys the content if the parameter is a data type.
    pub value_integer: i64,
    /// Conveys the content if the parameter is a data type.
    pub value_integer_64: String,
    /// Conveys the content if the parameter is a data type.
    pub value_markdown: String,
    /// Conveys the content if the parameter is a data type.
    pub value_meta: super::meta::Meta,
    /// Conveys the content if the parameter is a data type.
    pub value_money: super::money::Money,
    /// Conveys the content if the parameter is a data type.
    pub value_oid: String,
    /// Conveys the content if the parameter is a data type.
    pub value_parameter_definition: super::parameter_definition::ParameterDefinition,
    /// Conveys the content if the parameter is a data type.
    pub value_period: super::period::Period,
    /// Conveys the content if the parameter is a data type.
    pub value_positive_int: u64,
    /// Conveys the content if the parameter is a data type.
    pub value_quantity: super::quantity::Quantity,
    /// Conveys the content if the parameter is a data type.
    pub value_range: super::range::Range,
    /// Conveys the content if the parameter is a data type.
    pub value_ratio: super::ratio::Ratio,
    /// Conveys the content if the parameter is a data type.
    pub value_ratio_range: super::ratio_range::RatioRange,
    /// Conveys the content if the parameter is a data type.
    pub value_reference: super::reference::Reference,
    /// Conveys the content if the parameter is a data type.
    pub value_related_artifact: super::related_artifact::RelatedArtifact,
    /// Conveys the content if the parameter is a data type.
    pub value_sampled_data: super::sampled_data::SampledData,
    /// Conveys the content if the parameter is a data type.
    pub value_signature: super::signature::Signature,
    /// Conveys the content if the parameter is a data type.
    pub value_string: String,
    /// Conveys the content if the parameter is a data type.
    pub value_time: String,
    /// Conveys the content if the parameter is a data type.
    pub value_timing: super::timing::Timing,
    /// Conveys the content if the parameter is a data type.
    pub value_trigger_definition: super::trigger_definition::TriggerDefinition,
    /// Conveys the content if the parameter is a data type.
    pub value_unsigned_int: u64,
    /// Conveys the content if the parameter is a data type.
    pub value_uri: String,
    /// Conveys the content if the parameter is a data type.
    pub value_url: String,
    /// Conveys the content if the parameter is a data type.
    pub value_usage_context: super::usage_context::UsageContext,
    /// Conveys the content if the parameter is a data type.
    pub value_uuid: String,
}
