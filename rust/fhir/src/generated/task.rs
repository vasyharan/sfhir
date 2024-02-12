/// A task to be performed.
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    /// The date and time this task was created.
    pub authored_on: super::date_time::DateTime,
    /// BasedOn refers to a higher-level authorization that triggered the creation
    /// of the task.  It references a "request" resource such as a ServiceRequest,
    /// MedicationRequest, CarePlan, etc. which is distinct from the "request" resource
    /// the task is seeking to fulfill.  This latter resource is referenced by focus.
    /// For example, based on a CarePlan (= basedOn), a task is created to fulfill a
    /// ServiceRequest ( = focus ) to collect a specimen from a patient.
    pub based_on: Vec<super::reference::Reference>,
    /// Contains business-specific nuances of the business state.
    pub business_status: super::codeable_concept::CodeableConcept,
    /// A name or code (or both) briefly describing what the task involves.
    pub code: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A free-text description of what is to be performed.
    pub description: super::string::String,
    /// If true indicates that the Task is asking for the specified action to *not*
    /// occur.
    pub do_not_perform: super::boolean::Boolean,
    /// The healthcare event  (e.g. a patient and healthcare provider interaction)
    /// during which this task was created.
    pub encounter: super::reference::Reference,
    /// Identifies the time action was first taken against the task (start) and/or the
    /// time final action was taken against the task prior to marking it as completed
    /// (end).
    pub execution_period: super::period::Period,
    /// The request being fulfilled or the resource being manipulated (changed,
    /// suspended, etc.) by this task.
    pub focus: super::reference::Reference,
    /// The entity who benefits from the performance of the service specified in the
    /// task (e.g., the patient).
    pub r#for: super::reference::Reference,
    /// A shared identifier common to multiple independent Task and Request instances
    /// that were activated/authorized more or less simultaneously by a single author.
    /// The presence of the same identifier on each request ties those requests together
    /// and may have business ramifications in terms of reporting of results, billing,
    /// etc.  E.g. a requisition number shared by a set of lab tests ordered together,
    /// or a prescription number shared by all meds ordered at one time.
    pub group_identifier: super::identifier::Identifier,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// The business identifier for this task.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Additional information that may be needed in the execution of the task.
    pub input: Vec<super::task::TaskInput>,
    /// The URL pointing to a *FHIR*-defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this Task.
    pub instantiates_canonical: super::canonical::Canonical,
    /// The URL pointing to an *externally* maintained  protocol, guideline, orderset or
    /// other definition that is adhered to in whole or in part by this Task.
    pub instantiates_uri: super::uri::Uri,
    /// Insurance plans, coverage extensions, pre-authorizations and/or pre-
    /// determinations that may be relevant to the Task.
    pub insurance: Vec<super::reference::Reference>,
    /// Indicates the "level" of actionability associated with the Task, i.e. i+R[9]Cs
    /// this a proposed task, a planned task, an actionable task, etc.
    pub intent: super::code::Code,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The date and time of last modification to this task.
    pub last_modified: super::date_time::DateTime,
    /// Principal physical location where this task is performed.
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
    /// Free-text information captured about the task as it progresses.
    pub note: Vec<super::annotation::Annotation>,
    /// Outputs produced by the Task.
    pub output: Vec<super::task::TaskOutput>,
    /// Party responsible for managing task execution.
    pub owner: super::reference::Reference,
    /// Task that this particular task is part of.
    pub part_of: Vec<super::reference::Reference>,
    /// The entity who performed the requested task.
    pub performer: Vec<super::task::TaskPerformer>,
    /// Indicates how quickly the Task should be addressed with respect to other
    /// requests.
    pub priority: super::code::Code,
    /// A description, code, or reference indicating why this task needs to be
    /// performed.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// Links to Provenance records for past versions of this Task that identify key
    /// state transitions or updates that are likely to be relevant to a user looking at
    /// the current version of the task.
    pub relevant_history: Vec<super::reference::Reference>,
    /// The kind of participant or specific participant that should perform the task.
    pub requested_performer: Vec<super::codeable_reference::CodeableReference>,
    /// Indicates the start and/or end of the period of time when completion of the task
    /// is desired to take place.
    pub requested_period: super::period::Period,
    /// The creator of the task.
    pub requester: super::reference::Reference,
    /// This is a Task resource
    pub resource_type: String,
    /// If the Task.focus is a request resource and the task is seeking fulfillment
    /// (i.e. is asking for the request to be actioned), this element identifies any
    /// limitations on what parts of the referenced request should be actioned.
    pub restriction: super::task::TaskRestriction,
    /// The current status of the task.
    pub status: super::code::Code,
    /// An explanation as to why this task is held, failed, was refused, etc.
    pub status_reason: super::codeable_reference::CodeableReference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A task to be performed.
#[derive(Debug, Clone, PartialEq)]
pub struct TaskInput {
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
    /// the task execution.
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

/// A task to be performed.
#[derive(Debug, Clone, PartialEq)]
pub struct TaskOutput {
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

/// A task to be performed.
#[derive(Debug, Clone, PartialEq)]
pub struct TaskPerformer {
    /// The actor or entity who performed the task.
    pub actor: super::reference::Reference,
    /// A code or description of the performer of the task.
    pub function: super::codeable_concept::CodeableConcept,
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
}

/// A task to be performed.
#[derive(Debug, Clone, PartialEq)]
pub struct TaskRestriction {
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
    /// The time-period for which fulfillment is sought. This must fall within
    /// the overall time period authorized in the referenced request.  E.g.
    /// ServiceRequest.occurance[x].
    pub period: super::period::Period,
    /// For requests that are targeted to more than one potential recipient/target, to
    /// identify who is fulfillment is sought for.
    pub recipient: Vec<super::reference::Reference>,
    /// Indicates the number of times the requested action should occur.
    pub repetitions: super::positive_int::PositiveInt,
}
