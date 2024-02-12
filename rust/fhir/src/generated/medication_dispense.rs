/// Indicates that a medication product is to be or has been dispensed for a named
/// person/patient.  This includes a description of the medication product (supply)
/// provided and the instructions for administering the medication.  The medication
/// dispense is the result of a pharmacy system responding to a medication order.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationDispense {
    /// Indicates the medication order that is being dispensed against.
    pub authorizing_prescription: Vec<super::reference::Reference>,
    /// A plan that is fulfilled in whole or in part by this MedicationDispense.
    pub based_on: Vec<super::reference::Reference>,
    /// Indicates the type of medication dispense (for example, drug classification like
    /// ATC, where meds would be administered, legal category of the medication.).
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The amount of medication expressed as a timing amount.
    pub days_supply: super::quantity::Quantity,
    /// Identification of the facility/location where the medication was/will be shipped
    /// to, as part of the dispense event.
    pub destination: super::reference::Reference,
    /// Indicates how the medication is to be used by the patient.
    pub dosage_instruction: Vec<super::dosage::Dosage>,
    /// The encounter that establishes the context for this event.
    pub encounter: super::reference::Reference,
    /// A summary of the events of interest that have occurred, such as when the
    /// dispense was verified.
    pub event_history: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifiers associated with this Medication Dispense that are defined by
    /// business processes and/or used to refer to it when a direct URL reference to
    /// the resource itself is not appropriate. They are business identifiers assigned
    /// to this resource by the performer or other systems and remain constant as the
    /// resource is updated and propagates from server to server.
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
    /// Identifies the medication supplied. This is either a link to a resource
    /// representing the details of the medication or a simple attribute carrying a code
    /// that identifies the medication from a known list of medications.
    pub medication: super::codeable_reference::CodeableReference,
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
    /// Indicates the reason why a dispense was not performed.
    pub not_performed_reason: super::codeable_reference::CodeableReference,
    /// Extra information about the dispense that could not be conveyed in the other
    /// attributes.
    pub note: Vec<super::annotation::Annotation>,
    /// The procedure or medication administration that triggered the dispense.
    pub part_of: Vec<super::reference::Reference>,
    /// Indicates who or what performed the event.
    pub performer: Vec<super::medication_dispense::MedicationDispensePerformer>,
    /// The amount of medication that has been dispensed. Includes unit of measure.
    pub quantity: super::quantity::Quantity,
    /// Identifies the person who picked up the medication or the location of where the
    /// medication was delivered.  This will usually be a patient or their caregiver,
    /// but some cases exist where it can be a healthcare professional or a location.
    pub receiver: Vec<super::reference::Reference>,
    /// The date (and maybe time) when the dispense activity started if whenPrepared or
    /// whenHandedOver is not populated.
    pub recorded: super::date_time::DateTime,
    /// The full representation of the dose of the medication included in all dosage
    /// instructions.  To be used when multiple dosage instructions are included to
    /// represent complex dosing such as increasing or tapering doses.
    pub rendered_dosage_instruction: super::markdown::Markdown,
    /// This is a MedicationDispense resource
    pub resource_type: String,
    /// A code specifying the state of the set of dispense events.
    pub status: super::code::Code,
    /// The date (and maybe time) when the status of the dispense record changed.
    pub status_changed: super::date_time::DateTime,
    /// A link to a resource representing the person or the group to whom the medication
    /// will be given.
    pub subject: super::reference::Reference,
    /// Indicates whether or not substitution was made as part of the dispense.  In
    /// some cases, substitution will be expected but does not happen, in other
    /// cases substitution is not expected but does happen.  This block explains
    /// what substitution did or did not happen and why.  If nothing is specified,
    /// substitution was not done.
    pub substitution: super::medication_dispense::MedicationDispenseSubstitution,
    /// Additional information that supports the medication being dispensed.  For
    /// example, there may be requirements that a specific lab test has been completed
    /// prior to dispensing or the patient's weight at the time of dispensing is
    /// documented.
    pub supporting_information: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Indicates the type of dispensing event that is performed. For example, Trial
    /// Fill, Completion of Trial, Partial Fill, Emergency Fill, Samples, etc.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The time the dispensed product was provided to the patient or their
    /// representative.
    pub when_handed_over: super::date_time::DateTime,
    /// The time when the dispensed product was packaged and reviewed.
    pub when_prepared: super::date_time::DateTime,
}

/// Indicates that a medication product is to be or has been dispensed for a named
/// person/patient.  This includes a description of the medication product (supply)
/// provided and the instructions for administering the medication.  The medication
/// dispense is the result of a pharmacy system responding to a medication order.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationDispensePerformer {
    /// The device, practitioner, etc. who performed the action.  It should be assumed
    /// that the actor is the dispenser of the medication.
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

/// Indicates that a medication product is to be or has been dispensed for a named
/// person/patient.  This includes a description of the medication product (supply)
/// provided and the instructions for administering the medication.  The medication
/// dispense is the result of a pharmacy system responding to a medication order.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationDispenseSubstitution {
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
    /// Indicates the reason for the substitution (or lack of substitution) from what
    /// was prescribed.
    pub reason: Vec<super::codeable_concept::CodeableConcept>,
    /// The person or organization that has primary responsibility for the substitution.
    pub responsible_party: super::reference::Reference,
    /// A code signifying whether a different drug was dispensed from what was
    /// prescribed.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// True if the dispenser dispensed a different drug or product from what was
    /// prescribed.
    pub was_substituted: super::boolean::Boolean,
}
