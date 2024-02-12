/// Represents a request for a patient to employ a medical device. The device may be
/// an implantable device, or an external assistive device, such as a walker.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceRequest {
    /// This status is to indicate whether the request is a PRN or may be given as
    /// needed.
    pub as_needed: super::boolean::Boolean,
    /// The reason for using the device.
    pub as_needed_for: super::codeable_concept::CodeableConcept,
    /// When the request transitioned to being actionable.
    pub authored_on: super::date_time::DateTime,
    /// Plan/proposal/order fulfilled by this request.
    pub based_on: Vec<super::reference::Reference>,
    /// The details of the device to be used.
    pub code: super::codeable_reference::CodeableReference,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// If true, indicates that the provider is asking for the patient to either stop
    /// using or to not start using the specified device or category of devices. For
    /// example, the patient has undergone surgery and the provider is indicating that
    /// the patient should not wear contact lenses.
    pub do_not_perform: super::boolean::Boolean,
    /// An encounter that provides additional context in which this request is made.
    pub encounter: super::reference::Reference,
    /// A shared identifier common to multiple independent Request instances that
    /// were activated/authorized more or less simultaneously by a single author.  The
    /// presence of the same identifier on each request ties those requests together and
    /// may have business ramifications in terms of reporting of results, billing, etc.
    /// E.g. a requisition number shared by a set of lab tests ordered together, or a
    /// prescription number shared by all meds ordered at one time.
    pub group_identifier: super::identifier::Identifier,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifiers assigned to this order by the orderer or by the receiver.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this DeviceRequest.
    pub instantiates_canonical: Vec<super::canonical::Canonical>,
    /// The URL pointing to an externally maintained protocol, guideline, orderset or
    /// other definition that is adhered to in whole or in part by this DeviceRequest.
    pub instantiates_uri: Vec<super::uri::Uri>,
    /// Insurance plans, coverage extensions, pre-authorizations and/or pre-
    /// determinations that may be required for delivering the requested service.
    pub insurance: Vec<super::reference::Reference>,
    /// Whether the request is a proposal, plan, an original order or a reflex order.
    pub intent: super::code::Code,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
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
    /// Details about this request that were not represented at all or sufficiently
    /// in one of the attributes provided in a class. These may include for example a
    /// comment, an instruction, or a note associated with the statement.
    pub note: Vec<super::annotation::Annotation>,
    /// The timing schedule for the use of the device. The Schedule data type allows
    /// many different expressions, for example. "Every 8 hours"; "Three times a day";
    /// "1/2 an hour before breakfast for 10 days from 23-Dec 2011:"; "15 Oct 2013, 17
    /// Oct 2013 and 1 Nov 2013".
    pub occurrence_date_time: String,
    /// The timing schedule for the use of the device. The Schedule data type allows
    /// many different expressions, for example. "Every 8 hours"; "Three times a day";
    /// "1/2 an hour before breakfast for 10 days from 23-Dec 2011:"; "15 Oct 2013, 17
    /// Oct 2013 and 1 Nov 2013".
    pub occurrence_period: super::period::Period,
    /// The timing schedule for the use of the device. The Schedule data type allows
    /// many different expressions, for example. "Every 8 hours"; "Three times a day";
    /// "1/2 an hour before breakfast for 10 days from 23-Dec 2011:"; "15 Oct 2013, 17
    /// Oct 2013 and 1 Nov 2013".
    pub occurrence_timing: super::timing::Timing,
    /// Specific parameters for the ordered item.  For example, the prism value for
    /// lenses.
    pub parameter: Vec<super::device_request::DeviceRequestParameter>,
    /// The desired individual or entity to provide the device to the subject of the
    /// request (e.g., patient, location).
    pub performer: super::codeable_reference::CodeableReference,
    /// Indicates how quickly the request should be addressed with respect to other
    /// requests.
    pub priority: super::code::Code,
    /// The number of devices to be provided.
    pub quantity: super::integer::Integer,
    /// Reason or justification for the use of this device.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// Key events in the history of the request.
    pub relevant_history: Vec<super::reference::Reference>,
    /// The request takes the place of the referenced completed or terminated
    /// request(s).
    pub replaces: Vec<super::reference::Reference>,
    /// The individual or entity who initiated the request and has responsibility for
    /// its activation.
    pub requester: super::reference::Reference,
    /// This is a DeviceRequest resource
    pub resource_type: String,
    /// The status of the request.
    pub status: super::code::Code,
    /// The patient who will use the device.
    pub subject: super::reference::Reference,
    /// Additional clinical information about the patient that may influence the request
    /// fulfilment.  For example, this may include where on the subject's body the
    /// device will be used (i.e. the target site).
    pub supporting_info: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// Represents a request for a patient to employ a medical device. The device may be
/// an implantable device, or an external assistive device, such as a walker.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceRequestParameter {
    /// A code or string that identifies the device detail being asserted.
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
    /// The value of the device detail.
    pub value_boolean: bool,
    /// The value of the device detail.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The value of the device detail.
    pub value_quantity: super::quantity::Quantity,
    /// The value of the device detail.
    pub value_range: super::range::Range,
}
