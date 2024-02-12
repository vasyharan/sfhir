/// A record of a medication that is being consumed by a patient.   A
/// MedicationStatement may indicate that the patient may be taking the medication
/// now or has taken the medication in the past or will be taking the medication
/// in the future.  The source of this information can be the patient, significant
/// other (such as a family member or spouse), or a clinician.  A common scenario
/// where this information is captured is during the history taking process during
/// a patient visit or stay.   The medication information may come from sources
/// such as the patient's memory, from a prescription bottle,  or from a list of
/// medications the patient, clinician or other party maintains.
///
/// The primary difference between a medicationstatement and a
/// medicationadministration is that the medication administration has complete
/// administration information and is based on actual administration information
/// from the person who administered the medication.  A medicationstatement is
/// often, if not always, less specific.  There is no required date/time when the
/// medication was administered, in fact we only know that a source has reported the
/// patient is taking this medication, where details such as time, quantity, or rate
/// or even medication product may be incomplete or missing or less precise.  As
/// stated earlier, the Medication Statement information may come from the patient's
/// memory, from a prescription bottle or from a list of medications the patient,
/// clinician or other party maintains.  Medication administration is more formal
/// and is not missing detailed information.
///
/// The MedicationStatement resource was previously called MedicationStatement.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationStatement {
    /// Indicates whether the medication is or is not being consumed or administered.
    pub adherence: super::medication_statement::MedicationStatementAdherence,
    /// Type of medication statement (for example, drug classification like ATC, where
    /// meds would be administered, legal category of the medication.).
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The date when the Medication Statement was asserted by the information source.
    pub date_asserted: super::date_time::DateTime,
    /// Allows linking the MedicationStatement to the underlying MedicationRequest, or
    /// to other information that supports or is used to derive the MedicationStatement.
    pub derived_from: Vec<super::reference::Reference>,
    /// Indicates how the medication is/was or should be taken by the patient.
    pub dosage: Vec<super::dosage::Dosage>,
    /// The interval of time during which it is being asserted that the patient
    /// is/was/will be taking the medication (or was not taking, when the
    /// MedicationStatement.adherence element is Not Taking).
    pub effective_date_time: String,
    /// The interval of time during which it is being asserted that the patient
    /// is/was/will be taking the medication (or was not taking, when the
    /// MedicationStatement.adherence element is Not Taking).
    pub effective_period: super::period::Period,
    /// The interval of time during which it is being asserted that the patient
    /// is/was/will be taking the medication (or was not taking, when the
    /// MedicationStatement.adherence element is Not Taking).
    pub effective_timing: super::timing::Timing,
    /// The encounter that establishes the context for this MedicationStatement.
    pub encounter: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifiers associated with this Medication Statement that are defined by
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
    /// The person or organization that provided the information about the taking of
    /// this medication. Note: Use derivedFrom when a MedicationStatement is derived
    /// from other resources, e.g. Claim or MedicationRequest.
    pub information_source: Vec<super::reference::Reference>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Identifies the medication being administered. This is either a link to a
    /// resource representing the details of the medication or a simple attribute
    /// carrying a code that identifies the medication from a known list of medications.
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
    /// Provides extra information about the Medication Statement that is not conveyed
    /// by the other attributes.
    pub note: Vec<super::annotation::Annotation>,
    /// A larger event of which this particular MedicationStatement is a component or
    /// step.
    pub part_of: Vec<super::reference::Reference>,
    /// A concept, Condition or observation that supports why the medication is being/
    /// was taken.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// Link to information that is relevant to a medication statement, for example,
    /// illicit drug use, gestational age, etc.
    pub related_clinical_information: Vec<super::reference::Reference>,
    /// The full representation of the dose of the medication included in all dosage
    /// instructions.  To be used when multiple dosage instructions are included to
    /// represent complex dosing such as increasing or tapering doses.
    pub rendered_dosage_instruction: super::markdown::Markdown,
    /// This is a MedicationStatement resource
    pub resource_type: String,
    /// A code representing the status of recording the medication statement.
    pub status: super::code::Code,
    /// The person, animal or group who is/was taking the medication.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A record of a medication that is being consumed by a patient.   A
/// MedicationStatement may indicate that the patient may be taking the medication
/// now or has taken the medication in the past or will be taking the medication
/// in the future.  The source of this information can be the patient, significant
/// other (such as a family member or spouse), or a clinician.  A common scenario
/// where this information is captured is during the history taking process during
/// a patient visit or stay.   The medication information may come from sources
/// such as the patient's memory, from a prescription bottle,  or from a list of
/// medications the patient, clinician or other party maintains.
///
/// The primary difference between a medicationstatement and a
/// medicationadministration is that the medication administration has complete
/// administration information and is based on actual administration information
/// from the person who administered the medication.  A medicationstatement is
/// often, if not always, less specific.  There is no required date/time when the
/// medication was administered, in fact we only know that a source has reported the
/// patient is taking this medication, where details such as time, quantity, or rate
/// or even medication product may be incomplete or missing or less precise.  As
/// stated earlier, the Medication Statement information may come from the patient's
/// memory, from a prescription bottle or from a list of medications the patient,
/// clinician or other party maintains.  Medication administration is more formal
/// and is not missing detailed information.
///
/// The MedicationStatement resource was previously called MedicationStatement.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationStatementAdherence {
    /// Type of the adherence for the medication.
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
    /// Captures the reason for the current use or adherence of a medication.
    pub reason: super::codeable_concept::CodeableConcept,
}
