/// A record of dispensation of a device - i.e., assigning a device to a patient, or
/// to a professional for their use.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDispense {
    /// The order or request that this dispense is fulfilling.
    pub based_on: Vec<super::reference::Reference>,
    /// Indicates the type of device dispense.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Identification of the facility/location where the device was /should be shipped
    /// to, as part of the dispense process.
    pub destination: super::reference::Reference,
    /// Identifies the device being dispensed. This is either a link to a resource
    /// representing the details of the device or a simple attribute carrying a code
    /// that identifies the device from a known list of devices.
    pub device: super::codeable_reference::CodeableReference,
    /// The encounter that establishes the context for this event.
    pub encounter: super::reference::Reference,
    /// A summary of the events of interest that have occurred, such as when the
    /// dispense was verified.
    pub event_history: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifier for this dispensation.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The principal physical location where the dispense was performed.
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
    /// Extra information about the dispense that could not be conveyed in the other
    /// attributes.
    pub note: Vec<super::annotation::Annotation>,
    /// The bigger event that this dispense is a part of.
    pub part_of: Vec<super::reference::Reference>,
    /// Indicates who or what performed the event.
    pub performer: Vec<super::device_dispense::DeviceDispensePerformer>,
    /// The time when the dispensed product was packaged and reviewed.
    pub prepared_date: super::date_time::DateTime,
    /// The number of devices that have been dispensed.
    pub quantity: super::quantity::Quantity,
    /// Identifies the person who picked up the device or the person or location where
    /// the device was delivered.  This may be a patient or their caregiver, but some
    /// cases exist where it can be a healthcare professional or a location.
    pub receiver: super::reference::Reference,
    /// This is a DeviceDispense resource
    pub resource_type: String,
    /// A code specifying the state of the set of dispense events.
    pub status: super::code::Code,
    /// Indicates the reason why a dispense was or was not performed.
    pub status_reason: super::codeable_reference::CodeableReference,
    /// A link to a resource representing the person to whom the device is intended.
    pub subject: super::reference::Reference,
    /// Additional information that supports the device being dispensed.
    pub supporting_information: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Indicates the type of dispensing event that is performed.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The full representation of the instructions.
    pub usage_instruction: super::markdown::Markdown,
    /// The time the dispensed product was made available to the patient or their
    /// representative.
    pub when_handed_over: super::date_time::DateTime,
}

/// A record of dispensation of a device - i.e., assigning a device to a patient, or
/// to a professional for their use.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDispensePerformer {
    /// The device, practitioner, etc. who performed the action.  It should be assumed
    /// that the actor is the dispenser of the device.
    pub actor: super::reference::Reference,
    /// Distinguishes the type of performer in the dispense.  For example, date enterer,
    /// packager, final checker.
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
