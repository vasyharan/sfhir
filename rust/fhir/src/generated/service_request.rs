/// A record of a request for service such as diagnostic investigations, treatments,
/// or operations to be performed.
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceRequest {
    /// If a CodeableConcept is present, it indicates the pre-condition for performing
    /// the service.  For example "pain", "on flare-up", etc.
    pub as_needed_boolean: bool,
    /// If a CodeableConcept is present, it indicates the pre-condition for performing
    /// the service.  For example "pain", "on flare-up", etc.
    pub as_needed_codeable_concept: super::codeable_concept::CodeableConcept,
    /// When the request transitioned to being actionable.
    pub authored_on: super::date_time::DateTime,
    /// Plan/proposal/order fulfilled by this request.
    pub based_on: Vec<super::reference::Reference>,
    /// Anatomic location where the procedure should be performed. This is the target
    /// site.
    pub body_site: Vec<super::codeable_concept::CodeableConcept>,
    /// Anatomic location where the procedure should be performed. This is the target
    /// site.
    pub body_structure: super::reference::Reference,
    /// A code that classifies the service for searching, sorting and display purposes
    /// (e.g. "Surgical Procedure").
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// A code or reference that identifies a particular service (i.e., procedure,
    /// diagnostic investigation, or panel of investigations) that have been requested.
    pub code: super::codeable_reference::CodeableReference,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Set this to true if the record is saying that the service/procedure should NOT
    /// be performed.
    pub do_not_perform: super::boolean::Boolean,
    /// An encounter that provides additional information about the healthcare context
    /// in which this request is made.
    pub encounter: super::reference::Reference,
    /// The actual focus of a service request when it is not the subject of record
    /// representing something or someone associated with the subject such as a
    /// spouse, parent, fetus, or donor. The focus of a service request could also be
    /// an existing condition,  an intervention, the subject's diet,  another service
    /// request on the subject,  or a body structure such as tumor or implanted device.
    pub focus: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifiers assigned to this order instance by the orderer and/or the receiver
    /// and/or order fulfiller.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this ServiceRequest.
    pub instantiates_canonical: Vec<super::canonical::Canonical>,
    /// The URL pointing to an externally maintained protocol, guideline, orderset or
    /// other definition that is adhered to in whole or in part by this ServiceRequest.
    pub instantiates_uri: Vec<super::uri::Uri>,
    /// Insurance plans, coverage extensions, pre-authorizations and/or pre-
    /// determinations that may be needed for delivering the requested service.
    pub insurance: Vec<super::reference::Reference>,
    /// Whether the request is a proposal, plan, an original order or a reflex order.
    pub intent: super::code::Code,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The preferred location(s) where the procedure should actually happen in coded or
    /// free text form. E.g. at home or nursing day care center.
    pub location: Vec<super::codeable_reference::CodeableReference>,
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
    /// Any other notes and comments made about the service request. For example,
    /// internal billing notes.
    pub note: Vec<super::annotation::Annotation>,
    /// The date/time at which the requested service should occur.
    pub occurrence_date_time: String,
    /// The date/time at which the requested service should occur.
    pub occurrence_period: super::period::Period,
    /// The date/time at which the requested service should occur.
    pub occurrence_timing: super::timing::Timing,
    /// Additional details and instructions about the how the services are to be
    /// delivered.   For example, and order for a urinary catheter may have an order
    /// detail for an external or indwelling catheter, or an order for a bandage may
    /// require additional instructions specifying how the bandage should be applied.
    pub order_detail: Vec<super::service_request::ServiceRequestOrderDetail>,
    /// Instructions in terms that are understood by the patient or consumer.
    pub patient_instruction: Vec<super::service_request::ServiceRequestPatientInstruction>,
    /// The desired performer for doing the requested service.  For example, the
    /// surgeon, dermatopathologist, endoscopist, etc.
    pub performer: Vec<super::reference::Reference>,
    /// Desired type of performer for doing the requested service.
    pub performer_type: super::codeable_concept::CodeableConcept,
    /// Indicates how quickly the ServiceRequest should be addressed with respect to
    /// other requests.
    pub priority: super::code::Code,
    /// An amount of service being requested which can be a quantity ( for example
    /// $1,500 home modification), a ratio ( for example, 20 half day visits per month),
    /// or a range (2.0 to 1.8 Gy per fraction).
    pub quantity_quantity: super::quantity::Quantity,
    /// An amount of service being requested which can be a quantity ( for example
    /// $1,500 home modification), a ratio ( for example, 20 half day visits per month),
    /// or a range (2.0 to 1.8 Gy per fraction).
    pub quantity_range: super::range::Range,
    /// An amount of service being requested which can be a quantity ( for example
    /// $1,500 home modification), a ratio ( for example, 20 half day visits per month),
    /// or a range (2.0 to 1.8 Gy per fraction).
    pub quantity_ratio: super::ratio::Ratio,
    /// An explanation or justification for why this service is being requested in
    /// coded or textual form.   This is often for billing purposes.  May relate to the
    /// resources referred to in `supportingInfo`.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// Key events in the history of the request.
    pub relevant_history: Vec<super::reference::Reference>,
    /// The request takes the place of the referenced completed or terminated
    /// request(s).
    pub replaces: Vec<super::reference::Reference>,
    /// The individual who initiated the request and has responsibility for its
    /// activation.
    pub requester: super::reference::Reference,
    /// A shared identifier common to all service requests that were authorized more
    /// or less simultaneously by a single author, representing the composite or group
    /// identifier.
    pub requisition: super::identifier::Identifier,
    /// This is a ServiceRequest resource
    pub resource_type: String,
    /// One or more specimens that the laboratory procedure will use.
    pub specimen: Vec<super::reference::Reference>,
    /// The status of the order.
    pub status: super::code::Code,
    /// On whom or what the service is to be performed. This is usually a human patient,
    /// but can also be requested on animals, groups of humans or animals, devices such
    /// as dialysis machines, or even locations (typically for environmental scans).
    pub subject: super::reference::Reference,
    /// Additional clinical information about the patient or specimen that may influence
    /// the services or their interpretations.     This information includes diagnosis,
    /// clinical findings and other observations.  In laboratory ordering these are
    /// typically referred to as "ask at order entry questions (AOEs)".  This includes
    /// observations explicitly requested by the producer (filler) to provide context or
    /// supporting information needed to complete the order. For example,  reporting the
    /// amount of inspired oxygen for blood gas measurements.
    pub supporting_info: Vec<super::codeable_reference::CodeableReference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A record of a request for service such as diagnostic investigations, treatments,
/// or operations to be performed.
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceRequestOrderDetail {
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
    /// The parameter details for the service being requested.
    pub parameter: Vec<super::service_request::ServiceRequestParameter>,
    /// Indicates the context of the order details by reference.
    pub parameter_focus: super::codeable_reference::CodeableReference,
}

/// A record of a request for service such as diagnostic investigations, treatments,
/// or operations to be performed.
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceRequestParameter {
    /// A value representing the additional detail or instructions for the order (e.g.,
    /// catheter insertion, body elevation, descriptive device configuration and/or
    /// setting instructions).
    pub code: super::codeable_concept::CodeableConcept,
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
    /// Indicates a value for the order detail.
    pub value_boolean: bool,
    /// Indicates a value for the order detail.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Indicates a value for the order detail.
    pub value_period: super::period::Period,
    /// Indicates a value for the order detail.
    pub value_quantity: super::quantity::Quantity,
    /// Indicates a value for the order detail.
    pub value_range: super::range::Range,
    /// Indicates a value for the order detail.
    pub value_ratio: super::ratio::Ratio,
    /// Indicates a value for the order detail.
    pub value_string: String,
}

/// A record of a request for service such as diagnostic investigations, treatments,
/// or operations to be performed.
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceRequestPatientInstruction {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Instructions in terms that are understood by the patient or consumer.
    pub instruction_markdown: String,
    /// Instructions in terms that are understood by the patient or consumer.
    pub instruction_reference: super::reference::Reference,
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
