/// A record of a device being used by a patient where the record is the result of a
/// report from the patient or a clinician.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceUsage {
    /// This indicates how or if the device is being used.
    pub adherence: super::device_usage::DeviceUsageAdherence,
    /// A plan, proposal or order that is fulfilled in whole or in part by this
    /// DeviceUsage.
    pub based_on: Vec<super::reference::Reference>,
    /// Indicates the anotomic location on the subject's body where the device was used
    /// ( i.e. the target).
    pub body_site: super::codeable_reference::CodeableReference,
    /// This attribute indicates a category for the statement - The device statement
    /// may be made in an inpatient or outpatient settting (inpatient | outpatient |
    /// community | patientspecified).
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The encounter or episode of care that establishes the context for this device
    /// use statement.
    pub context: super::reference::Reference,
    /// The time at which the statement was recorded by informationSource.
    pub date_asserted: super::date_time::DateTime,
    /// Allows linking the DeviceUsage to the underlying Request, or to other
    /// information that supports or is used to derive the DeviceUsage.
    pub derived_from: Vec<super::reference::Reference>,
    /// Code or Reference to device used.
    pub device: super::codeable_reference::CodeableReference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// An external identifier for this statement such as an IRI.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Who reported the device was being used by the patient.
    pub information_source: super::reference::Reference,
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
    /// Details about the device statement that were not represented at all or
    /// sufficiently in one of the attributes provided in a class. These may include for
    /// example a comment, an instruction, or a note associated with the statement.
    pub note: Vec<super::annotation::Annotation>,
    /// The patient who used the device.
    pub patient: super::reference::Reference,
    /// Reason or justification for the use of the device. A coded concept, or another
    /// resource whose existence justifies this DeviceUsage.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// This is a DeviceUsage resource
    pub resource_type: String,
    /// A code representing the patient or other source's judgment about the state of
    /// the device used that this statement is about.  Generally this will be active
    /// or completed.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// How often the device was used.
    pub timing_date_time: String,
    /// How often the device was used.
    pub timing_period: super::period::Period,
    /// How often the device was used.
    pub timing_timing: super::timing::Timing,
    /// The reason for asserting the usage status - for example forgot, lost, stolen,
    /// broken.
    pub usage_reason: Vec<super::codeable_concept::CodeableConcept>,
    /// The status of the device usage, for example always, sometimes, never. This is
    /// not the same as the status of the statement.
    pub usage_status: super::codeable_concept::CodeableConcept,
}

/// A record of a device being used by a patient where the record is the result of a
/// report from the patient or a clinician.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceUsageAdherence {
    /// Type of adherence.
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
    /// Reason for adherence type.
    pub reason: Vec<super::codeable_concept::CodeableConcept>,
}
