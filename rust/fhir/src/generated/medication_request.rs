/// An order or request for both supply of the medication and the instructions
/// for administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder"
/// to generalize the use across inpatient and outpatient settings, including care
/// plans, etc., and to harmonize with workflow patterns.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationRequest {
    /// The date (and perhaps time) when the prescription was initially written or
    /// authored on.
    pub authored_on: super::date_time::DateTime,
    /// A plan or request that is fulfilled in whole or in part by this medication
    /// request.
    pub based_on: Vec<super::reference::Reference>,
    /// An arbitrary categorization or grouping of the medication request.  It could
    /// be used for indicating where meds are intended to be administered, eg. in an
    /// inpatient setting or in a patient's home, or a legal category of the medication.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The description of the overall pattern of the administration of the medication
    /// to the patient.
    pub course_of_therapy_type: super::codeable_concept::CodeableConcept,
    /// The intended type of device that is to be used for the administration of the
    /// medication (for example, PCA Pump).
    pub device: Vec<super::codeable_reference::CodeableReference>,
    /// Indicates the specific details for the dispense or medication supply part of
    /// a medication request (also known as a Medication Prescription or Medication
    /// Order).  Note that this information is not always sent with the order.  There
    /// may be in some settings (e.g. hospitals) institutional or system support for
    /// completing the dispense details in the pharmacy department.
    pub dispense_request: super::medication_request::MedicationRequestDispenseRequest,
    /// If true, indicates that the provider is asking for the patient to either stop
    /// taking or to not start taking the specified medication. For example, the patient
    /// is taking an existing medication and the provider is changing their medication.
    /// They want to create two seperate requests: one to stop using the current
    /// medication and another to start the new medication.
    pub do_not_perform: super::boolean::Boolean,
    /// Specific instructions for how the medication is to be used by the patient.
    pub dosage_instruction: Vec<super::dosage::Dosage>,
    /// The period over which the medication is to be taken.  Where there are multiple
    /// dosageInstruction lines (for example, tapering doses), this is the earliest date
    /// and the latest end date of the dosageInstructions.
    pub effective_dose_period: super::period::Period,
    /// The Encounter during which this [x] was created or to which the creation of this
    /// record is tightly associated.
    pub encounter: super::reference::Reference,
    /// Links to Provenance records for past versions of this resource or fulfilling
    /// request or event resources that identify key state transitions or updates
    /// that are likely to be relevant to a user looking at the current version of the
    /// resource.
    pub event_history: Vec<super::reference::Reference>,
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
    /// Identifiers associated with this medication request that are defined by business
    /// processes and/or used to refer to it when a direct URL reference to the resource
    /// itself is not appropriate. They are business identifiers assigned to this
    /// resource by the performer or other systems and remain constant as the resource
    /// is updated and propagates from server to server.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The person or organization who provided the information about this request, if
    /// the source is someone other than the requestor.  This is often used when the
    /// MedicationRequest is reported by another person.
    pub information_source: Vec<super::reference::Reference>,
    /// Insurance plans, coverage extensions, pre-authorizations and/or pre-
    /// determinations that may be required for delivering the requested service.
    pub insurance: Vec<super::reference::Reference>,
    /// Whether the request is a proposal, plan, or an original order.
    pub intent: super::code::Code,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Identifies the medication being requested. This is a link to a resource that
    /// represents the medication which may be the details of the medication or simply
    /// an attribute carrying a code that identifies the medication from a known list
    /// of medications.
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
    /// Extra information about the prescription that could not be conveyed by the other
    /// attributes.
    pub note: Vec<super::annotation::Annotation>,
    /// The specified desired performer of the medication treatment (e.g. the performer
    /// of the medication administration).  For devices, this is the device that is
    /// intended to perform the administration of the medication.  An IV Pump would
    /// be an example of a device that is performing the administration.  Both the IV
    /// Pump and the practitioner that set the rate or bolus on the pump can be listed
    /// as performers.
    pub performer: Vec<super::reference::Reference>,
    /// Indicates the type of performer of the administration of the medication.
    pub performer_type: super::codeable_concept::CodeableConcept,
    /// Reference to an order/prescription that is being replaced by this
    /// MedicationRequest.
    pub prior_prescription: super::reference::Reference,
    /// Indicates how quickly the Medication Request should be addressed with respect to
    /// other requests.
    pub priority: super::code::Code,
    /// The reason or the indication for ordering or not ordering the medication.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// The person who entered the order on behalf of another individual for example in
    /// the case of a verbal or a telephone order.
    pub recorder: super::reference::Reference,
    /// The full representation of the dose of the medication included in all dosage
    /// instructions.  To be used when multiple dosage instructions are included to
    /// represent complex dosing such as increasing or tapering doses.
    pub rendered_dosage_instruction: super::markdown::Markdown,
    /// Indicates if this record was captured as a secondary 'reported' record rather
    /// than as an original primary source-of-truth record.  It may also indicate the
    /// source of the report.
    pub reported: super::boolean::Boolean,
    /// The individual, organization, or device that initiated the request and has
    /// responsibility for its activation.
    pub requester: super::reference::Reference,
    /// This is a MedicationRequest resource
    pub resource_type: String,
    /// A code specifying the current state of the order.  Generally, this will be
    /// active or completed state.
    pub status: super::code::Code,
    /// The date (and perhaps time) when the status was changed.
    pub status_changed: super::date_time::DateTime,
    /// Captures the reason for the current state of the MedicationRequest.
    pub status_reason: super::codeable_concept::CodeableConcept,
    /// The individual or group for whom the medication has been requested.
    pub subject: super::reference::Reference,
    /// Indicates whether or not substitution can or should be part of the dispense.
    /// In some cases, substitution must happen, in other cases substitution must not
    /// happen. This block explains the prescriber's intent. If nothing is specified
    /// substitution may be done.
    pub substitution: super::medication_request::MedicationRequestSubstitution,
    /// Information to support fulfilling (i.e. dispensing or administering) of the
    /// medication, for example, patient height and weight, a MedicationStatement for
    /// the patient).
    pub supporting_information: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// An order or request for both supply of the medication and the instructions
/// for administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder"
/// to generalize the use across inpatient and outpatient settings, including care
/// plans, etc., and to harmonize with workflow patterns.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationRequestDispenseRequest {
    /// The minimum period of time that must occur between dispenses of the medication.
    pub dispense_interval: super::duration::Duration,
    /// Indicates the intended performing Organization that will dispense the medication
    /// as specified by the prescriber.
    pub dispenser: super::reference::Reference,
    /// Provides additional information to the dispenser, for example, counselling to be
    /// provided to the patient.
    pub dispenser_instruction: Vec<super::annotation::Annotation>,
    /// Provides information about the type of adherence packaging to be supplied for
    /// the medication dispense.
    pub dose_administration_aid: super::codeable_concept::CodeableConcept,
    /// Identifies the period time over which the supplied product is expected to be
    /// used, or the length of time the dispense is expected to last.
    pub expected_supply_duration: super::duration::Duration,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Indicates the quantity or duration for the first dispense of the medication.
    pub initial_fill: super::medication_request::MedicationRequestInitialFill,
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
    /// An integer indicating the number of times, in addition to the original dispense,
    /// (aka refills or repeats) that the patient can receive the prescribed medication.
    /// Usage Notes: This integer does not include the original order dispense. This
    /// means that if an order indicates dispense 30 tablets plus "3 repeats", then the
    /// order can be dispensed a total of 4 times and the patient can receive a total
    /// of 120 tablets.  A prescriber may explicitly say that zero refills are permitted
    /// after the initial dispense.
    pub number_of_repeats_allowed: super::unsigned_int::UnsignedInt,
    /// The amount that is to be dispensed for one fill.
    pub quantity: super::quantity::Quantity,
    /// This indicates the validity period of a prescription (stale dating the
    /// Prescription).
    pub validity_period: super::period::Period,
}

/// An order or request for both supply of the medication and the instructions
/// for administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder"
/// to generalize the use across inpatient and outpatient settings, including care
/// plans, etc., and to harmonize with workflow patterns.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationRequestInitialFill {
    /// The length of time that the first dispense is expected to last.
    pub duration: super::duration::Duration,
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
    /// The amount or quantity to provide as part of the first dispense.
    pub quantity: super::quantity::Quantity,
}

/// An order or request for both supply of the medication and the instructions
/// for administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder"
/// to generalize the use across inpatient and outpatient settings, including care
/// plans, etc., and to harmonize with workflow patterns.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationRequestSubstitution {
    /// True if the prescriber allows a different drug to be dispensed from what was
    /// prescribed.
    pub allowed_boolean: bool,
    /// True if the prescriber allows a different drug to be dispensed from what was
    /// prescribed.
    pub allowed_codeable_concept: super::codeable_concept::CodeableConcept,
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
    /// Indicates the reason for the substitution, or why substitution must or must not
    /// be performed.
    pub reason: super::codeable_concept::CodeableConcept,
}
