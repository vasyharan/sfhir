/// Record of transport of item.
#[derive(Debug, Clone, PartialEq)]
pub struct Transport {
    /// The date and time this transport was created.
    pub authored_on: super::date_time::DateTime,
    /// BasedOn refers to a higher-level authorization that triggered the creation of
    /// the transport.  It references a "request" resource such as a ServiceRequest
    /// or Transport, which is distinct from the "request" resource the Transport
    /// is seeking to fulfill.  This latter resource is referenced by FocusOn.  For
    /// example, based on a ServiceRequest (= BasedOn), a transport is created to
    /// fulfill a procedureRequest ( = FocusOn ) to transport a specimen to the lab.
    pub based_on: Vec<super::reference::Reference>,
    /// A name or code (or both) briefly describing what the transport involves.
    pub code: super::codeable_concept::CodeableConcept,
    /// Identifies the completion time of the event (the occurrence).
    pub completion_time: super::date_time::DateTime,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The current location for the entity to be transported.
    pub current_location: super::reference::Reference,
    /// A free-text description of what is to be performed.
    pub description: super::string::String,
    /// The healthcare event  (e.g. a patient and healthcare provider interaction)
    /// during which this transport was created.
    pub encounter: super::reference::Reference,
    /// The request being actioned or the resource being manipulated by this transport.
    pub focus: super::reference::Reference,
    /// The entity who benefits from the performance of the service specified in the
    /// transport (e.g., the patient).
    pub r#for: super::reference::Reference,
    /// A shared identifier common to multiple independent Request instances that
    /// were activated/authorized more or less simultaneously by a single author.  The
    /// presence of the same identifier on each request ties those requests together and
    /// may have business ramifications in terms of reporting of results, billing, etc.
    /// E.g. a requisition number shared by a set of lab tests ordered together, or a
    /// prescription number shared by all meds ordered at one time.
    pub group_identifier: super::identifier::Identifier,
    /// The transport event prior to this one.
    pub history: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifier for the transport event that is used to identify it across multiple
    /// disparate systems.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Additional information that may be needed in the execution of the transport.
    pub input: Vec<super::transport::TransportInput>,
    /// The URL pointing to a *FHIR*-defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this Transport.
    pub instantiates_canonical: super::canonical::Canonical,
    /// The URL pointing to an *externally* maintained  protocol, guideline, orderset or
    /// other definition that is adhered to in whole or in part by this Transport.
    pub instantiates_uri: super::uri::Uri,
    /// Insurance plans, coverage extensions, pre-authorizations and/or pre-
    /// determinations that may be relevant to the Transport.
    pub insurance: Vec<super::reference::Reference>,
    /// Indicates the "level" of actionability associated with the Transport, i.e.
    /// i+R[9]Cs this a proposed transport, a planned transport, an actionable
    /// transport, etc.
    pub intent: super::code::Code,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The date and time of last modification to this transport.
    pub last_modified: super::date_time::DateTime,
    /// Principal physical location where this transport is performed.
    pub location: super::reference::Reference,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// May be used to represent additional information that is not part of the
    /// basic definition of the resource and that modifies the understanding of the
    /// element that contains it and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification.
    /// To make the use of extensions safe and managable, there is a strict set
    /// of governance applied to the definition and use of extensions. Though any
    /// implementer is allowed to define an extension, there is a set of requirements
    /// that SHALL be met as part of the definition of the extension. Applications
    /// processing a resource are required to check for modifier extensions.
    ///
    /// Modifier extensions SHALL NOT change the meaning of any elements on Resource
    /// or DomainResource (including cannot change the meaning of modifierExtension
    /// itself).
    pub modifier_extension: Vec<super::extension::Extension>,
    /// Free-text information captured about the transport as it progresses.
    pub note: Vec<super::annotation::Annotation>,
    /// Outputs produced by the Transport.
    pub output: Vec<super::transport::TransportOutput>,
    /// Individual organization or Device currently responsible for transport execution.
    pub owner: super::reference::Reference,
    /// A larger event of which this particular event is a component or step.
    pub part_of: Vec<super::reference::Reference>,
    /// The kind of participant that should perform the transport.
    pub performer_type: Vec<super::codeable_concept::CodeableConcept>,
    /// Indicates how quickly the Transport should be addressed with respect to other
    /// requests.
    pub priority: super::code::Code,
    /// A resource reference indicating why this transport needs to be performed.
    pub reason: super::codeable_reference::CodeableReference,
    /// Links to Provenance records for past versions of this Transport that identify
    /// key state transitions or updates that are likely to be relevant to a user
    /// looking at the current version of the transport.
    pub relevant_history: Vec<super::reference::Reference>,
    /// The desired or final location for the transport.
    pub requested_location: super::reference::Reference,
    /// The creator of the transport.
    pub requester: super::reference::Reference,
    /// This is a Transport resource
    pub resource_type: String,
    /// If the Transport.focus is a request resource and the transport is seeking
    /// fulfillment (i.e. is asking for the request to be actioned), this element
    /// identifies any limitations on what parts of the referenced request should be
    /// actioned.
    pub restriction: super::transport::TransportRestriction,
    /// A code specifying the state of the transport event.
    pub status: super::code::Code,
    /// An explanation as to why this transport is held, failed, was refused, etc.
    pub status_reason: super::codeable_concept::CodeableConcept,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// Record of transport of item.
#[derive(Debug, Clone, PartialEq)]
pub struct TransportInput {
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
    /// A code or description indicating how the input is intended to be used as part of
    /// the transport execution.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The value of the input parameter as a basic type.
    pub value_address: super::address::Address,
    /// The value of the input parameter as a basic type.
    pub value_age: super::age::Age,
    /// The value of the input parameter as a basic type.
    pub value_annotation: super::annotation::Annotation,
    /// The value of the input parameter as a basic type.
    pub value_attachment: super::attachment::Attachment,
    /// The value of the input parameter as a basic type.
    pub value_availability: super::availability::Availability,
    /// The value of the input parameter as a basic type.
    pub value_base_64_binary: String,
    /// The value of the input parameter as a basic type.
    pub value_boolean: bool,
    /// The value of the input parameter as a basic type.
    pub value_canonical: String,
    /// The value of the input parameter as a basic type.
    pub value_code: String,
    /// The value of the input parameter as a basic type.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The value of the input parameter as a basic type.
    pub value_codeable_reference: super::codeable_reference::CodeableReference,
    /// The value of the input parameter as a basic type.
    pub value_coding: super::coding::Coding,
    /// The value of the input parameter as a basic type.
    pub value_contact_detail: super::contact_detail::ContactDetail,
    /// The value of the input parameter as a basic type.
    pub value_contact_point: super::contact_point::ContactPoint,
    /// The value of the input parameter as a basic type.
    pub value_count: super::count::Count,
    /// The value of the input parameter as a basic type.
    pub value_data_requirement: super::data_requirement::DataRequirement,
    /// The value of the input parameter as a basic type.
    pub value_date: String,
    /// The value of the input parameter as a basic type.
    pub value_date_time: String,
    /// The value of the input parameter as a basic type.
    pub value_decimal: f64,
    /// The value of the input parameter as a basic type.
    pub value_distance: super::distance::Distance,
    /// The value of the input parameter as a basic type.
    pub value_dosage: super::dosage::Dosage,
    /// The value of the input parameter as a basic type.
    pub value_duration: super::duration::Duration,
    /// The value of the input parameter as a basic type.
    pub value_expression: super::expression::Expression,
    /// The value of the input parameter as a basic type.
    pub value_extended_contact_detail: super::extended_contact_detail::ExtendedContactDetail,
    /// The value of the input parameter as a basic type.
    pub value_human_name: super::human_name::HumanName,
    /// The value of the input parameter as a basic type.
    pub value_id: String,
    /// The value of the input parameter as a basic type.
    pub value_identifier: super::identifier::Identifier,
    /// The value of the input parameter as a basic type.
    pub value_instant: String,
    /// The value of the input parameter as a basic type.
    pub value_integer: i64,
    /// The value of the input parameter as a basic type.
    pub value_integer_64: String,
    /// The value of the input parameter as a basic type.
    pub value_markdown: String,
    /// The value of the input parameter as a basic type.
    pub value_meta: super::meta::Meta,
    /// The value of the input parameter as a basic type.
    pub value_money: super::money::Money,
    /// The value of the input parameter as a basic type.
    pub value_oid: String,
    /// The value of the input parameter as a basic type.
    pub value_parameter_definition: super::parameter_definition::ParameterDefinition,
    /// The value of the input parameter as a basic type.
    pub value_period: super::period::Period,
    /// The value of the input parameter as a basic type.
    pub value_positive_int: u64,
    /// The value of the input parameter as a basic type.
    pub value_quantity: super::quantity::Quantity,
    /// The value of the input parameter as a basic type.
    pub value_range: super::range::Range,
    /// The value of the input parameter as a basic type.
    pub value_ratio: super::ratio::Ratio,
    /// The value of the input parameter as a basic type.
    pub value_ratio_range: super::ratio_range::RatioRange,
    /// The value of the input parameter as a basic type.
    pub value_reference: super::reference::Reference,
    /// The value of the input parameter as a basic type.
    pub value_related_artifact: super::related_artifact::RelatedArtifact,
    /// The value of the input parameter as a basic type.
    pub value_sampled_data: super::sampled_data::SampledData,
    /// The value of the input parameter as a basic type.
    pub value_signature: super::signature::Signature,
    /// The value of the input parameter as a basic type.
    pub value_string: String,
    /// The value of the input parameter as a basic type.
    pub value_time: String,
    /// The value of the input parameter as a basic type.
    pub value_timing: super::timing::Timing,
    /// The value of the input parameter as a basic type.
    pub value_trigger_definition: super::trigger_definition::TriggerDefinition,
    /// The value of the input parameter as a basic type.
    pub value_unsigned_int: u64,
    /// The value of the input parameter as a basic type.
    pub value_uri: String,
    /// The value of the input parameter as a basic type.
    pub value_url: String,
    /// The value of the input parameter as a basic type.
    pub value_usage_context: super::usage_context::UsageContext,
    /// The value of the input parameter as a basic type.
    pub value_uuid: String,
}

/// Record of transport of item.
#[derive(Debug, Clone, PartialEq)]
pub struct TransportOutput {
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
    /// The name of the Output parameter.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The value of the Output parameter as a basic type.
    pub value_address: super::address::Address,
    /// The value of the Output parameter as a basic type.
    pub value_age: super::age::Age,
    /// The value of the Output parameter as a basic type.
    pub value_annotation: super::annotation::Annotation,
    /// The value of the Output parameter as a basic type.
    pub value_attachment: super::attachment::Attachment,
    /// The value of the Output parameter as a basic type.
    pub value_availability: super::availability::Availability,
    /// The value of the Output parameter as a basic type.
    pub value_base_64_binary: String,
    /// The value of the Output parameter as a basic type.
    pub value_boolean: bool,
    /// The value of the Output parameter as a basic type.
    pub value_canonical: String,
    /// The value of the Output parameter as a basic type.
    pub value_code: String,
    /// The value of the Output parameter as a basic type.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The value of the Output parameter as a basic type.
    pub value_codeable_reference: super::codeable_reference::CodeableReference,
    /// The value of the Output parameter as a basic type.
    pub value_coding: super::coding::Coding,
    /// The value of the Output parameter as a basic type.
    pub value_contact_detail: super::contact_detail::ContactDetail,
    /// The value of the Output parameter as a basic type.
    pub value_contact_point: super::contact_point::ContactPoint,
    /// The value of the Output parameter as a basic type.
    pub value_count: super::count::Count,
    /// The value of the Output parameter as a basic type.
    pub value_data_requirement: super::data_requirement::DataRequirement,
    /// The value of the Output parameter as a basic type.
    pub value_date: String,
    /// The value of the Output parameter as a basic type.
    pub value_date_time: String,
    /// The value of the Output parameter as a basic type.
    pub value_decimal: f64,
    /// The value of the Output parameter as a basic type.
    pub value_distance: super::distance::Distance,
    /// The value of the Output parameter as a basic type.
    pub value_dosage: super::dosage::Dosage,
    /// The value of the Output parameter as a basic type.
    pub value_duration: super::duration::Duration,
    /// The value of the Output parameter as a basic type.
    pub value_expression: super::expression::Expression,
    /// The value of the Output parameter as a basic type.
    pub value_extended_contact_detail: super::extended_contact_detail::ExtendedContactDetail,
    /// The value of the Output parameter as a basic type.
    pub value_human_name: super::human_name::HumanName,
    /// The value of the Output parameter as a basic type.
    pub value_id: String,
    /// The value of the Output parameter as a basic type.
    pub value_identifier: super::identifier::Identifier,
    /// The value of the Output parameter as a basic type.
    pub value_instant: String,
    /// The value of the Output parameter as a basic type.
    pub value_integer: i64,
    /// The value of the Output parameter as a basic type.
    pub value_integer_64: String,
    /// The value of the Output parameter as a basic type.
    pub value_markdown: String,
    /// The value of the Output parameter as a basic type.
    pub value_meta: super::meta::Meta,
    /// The value of the Output parameter as a basic type.
    pub value_money: super::money::Money,
    /// The value of the Output parameter as a basic type.
    pub value_oid: String,
    /// The value of the Output parameter as a basic type.
    pub value_parameter_definition: super::parameter_definition::ParameterDefinition,
    /// The value of the Output parameter as a basic type.
    pub value_period: super::period::Period,
    /// The value of the Output parameter as a basic type.
    pub value_positive_int: u64,
    /// The value of the Output parameter as a basic type.
    pub value_quantity: super::quantity::Quantity,
    /// The value of the Output parameter as a basic type.
    pub value_range: super::range::Range,
    /// The value of the Output parameter as a basic type.
    pub value_ratio: super::ratio::Ratio,
    /// The value of the Output parameter as a basic type.
    pub value_ratio_range: super::ratio_range::RatioRange,
    /// The value of the Output parameter as a basic type.
    pub value_reference: super::reference::Reference,
    /// The value of the Output parameter as a basic type.
    pub value_related_artifact: super::related_artifact::RelatedArtifact,
    /// The value of the Output parameter as a basic type.
    pub value_sampled_data: super::sampled_data::SampledData,
    /// The value of the Output parameter as a basic type.
    pub value_signature: super::signature::Signature,
    /// The value of the Output parameter as a basic type.
    pub value_string: String,
    /// The value of the Output parameter as a basic type.
    pub value_time: String,
    /// The value of the Output parameter as a basic type.
    pub value_timing: super::timing::Timing,
    /// The value of the Output parameter as a basic type.
    pub value_trigger_definition: super::trigger_definition::TriggerDefinition,
    /// The value of the Output parameter as a basic type.
    pub value_unsigned_int: u64,
    /// The value of the Output parameter as a basic type.
    pub value_uri: String,
    /// The value of the Output parameter as a basic type.
    pub value_url: String,
    /// The value of the Output parameter as a basic type.
    pub value_usage_context: super::usage_context::UsageContext,
    /// The value of the Output parameter as a basic type.
    pub value_uuid: String,
}

/// Record of transport of item.
#[derive(Debug, Clone, PartialEq)]
pub struct TransportRestriction {
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
    /// Over what time-period is fulfillment sought.
    pub period: super::period::Period,
    /// For requests that are targeted to more than one potential recipient/target, to
    /// identify who is fulfillment is sought for.
    pub recipient: Vec<super::reference::Reference>,
    /// Indicates the number of times the requested action should occur.
    pub repetitions: super::positive_int::PositiveInt,
}
