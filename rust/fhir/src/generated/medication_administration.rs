/// Describes the event of a patient consuming or otherwise being administered
/// a medication.  This may be as simple as swallowing a tablet or it may be a
/// long running infusion.  Related resources tie this event to the authorizing
/// prescription, and the specific encounter between patient and health care
/// practitioner.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationAdministration {
    /// A plan that is fulfilled in whole or in part by this MedicationAdministration.
    pub based_on: Vec<super::reference::Reference>,
    /// The type of medication administration (for example, drug classification like
    /// ATC, where meds would be administered, legal category of the medication).
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The device that is to be used for the administration of the medication (for
    /// example, PCA Pump).
    pub device: Vec<super::codeable_reference::CodeableReference>,
    /// Describes the medication dosage information details e.g. dose, rate, site,
    /// route, etc.
    pub dosage: super::medication_administration::MedicationAdministrationDosage,
    /// The visit, admission, or other contact between patient and health care provider
    /// during which the medication administration was performed.
    pub encounter: super::reference::Reference,
    /// A summary of the events of interest that have occurred, such as when the
    /// administration was verified.
    pub event_history: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifiers associated with this Medication Administration that are defined by
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
    /// An indication that the full dose was not administered.
    pub is_sub_potent: super::boolean::Boolean,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Identifies the medication that was administered. This is either a link to
    /// a resource representing the details of the medication or a simple attribute
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
    /// Extra information about the medication administration that is not conveyed by
    /// the other attributes.
    pub note: Vec<super::annotation::Annotation>,
    /// A specific date/time or interval of time during which the administration took
    /// place (or did not take place). For many administrations, such as swallowing a
    /// tablet the use of dateTime is more appropriate.
    pub occurence_date_time: String,
    /// A specific date/time or interval of time during which the administration took
    /// place (or did not take place). For many administrations, such as swallowing a
    /// tablet the use of dateTime is more appropriate.
    pub occurence_period: super::period::Period,
    /// A specific date/time or interval of time during which the administration took
    /// place (or did not take place). For many administrations, such as swallowing a
    /// tablet the use of dateTime is more appropriate.
    pub occurence_timing: super::timing::Timing,
    /// A larger event of which this particular event is a component or step.
    pub part_of: Vec<super::reference::Reference>,
    /// The performer of the medication treatment.  For devices this is the device that
    /// performed the administration of the medication.  An IV Pump would be an example
    /// of a device that is performing the administration. Both the IV Pump and the
    /// practitioner that set the rate or bolus on the pump can be listed as performers.
    pub performer: Vec<super::medication_administration::MedicationAdministrationPerformer>,
    /// A code, Condition or observation that supports why the medication was
    /// administered.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// The date the occurrence of the  MedicationAdministration was first captured in
    /// the record - potentially significantly after the occurrence of the event.
    pub recorded: super::date_time::DateTime,
    /// The original request, instruction or authority to perform the administration.
    pub request: super::reference::Reference,
    /// This is a MedicationAdministration resource
    pub resource_type: String,
    /// Will generally be set to show that the administration has been completed.  For
    /// some long running administrations such as infusions, it is possible for an
    /// administration to be started but not completed or it may be paused while some
    /// other process is under way.
    pub status: super::code::Code,
    /// A code indicating why the administration was not performed.
    pub status_reason: Vec<super::codeable_concept::CodeableConcept>,
    /// The reason or reasons why the full dose was not administered.
    pub sub_potent_reason: Vec<super::codeable_concept::CodeableConcept>,
    /// The person or animal or group receiving the medication.
    pub subject: super::reference::Reference,
    /// Additional information (for example, patient height and weight) that supports
    /// the administration of the medication.  This attribute can be used to provide
    /// documentation of specific characteristics of the patient present at the time
    /// of administration.  For example, if the dose says "give "x" if the heartrate
    /// exceeds "y"", then the heart rate can be included using this attribute.
    pub supporting_information: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// Describes the event of a patient consuming or otherwise being administered
/// a medication.  This may be as simple as swallowing a tablet or it may be a
/// long running infusion.  Related resources tie this event to the authorizing
/// prescription, and the specific encounter between patient and health care
/// practitioner.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationAdministrationDosage {
    /// The amount of the medication given at one administration event.   Use this
    /// value when the administration is essentially an instantaneous event such as a
    /// swallowing a tablet or giving an injection.
    pub dose: super::quantity::Quantity,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A coded value indicating the method by which the medication is intended to be
    /// or was introduced into or on the body.  This attribute will most often NOT be
    /// populated.  It is most commonly used for injections.  For example, Slow Push,
    /// Deep IV.
    pub method: super::codeable_concept::CodeableConcept,
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
    /// Identifies the speed with which the medication was or will be introduced into
    /// the patient.  Typically, the rate for an infusion e.g. 100 ml per 1 hour or
    /// 100 ml/hr.  May also be expressed as a rate per unit of time, e.g. 500 ml per 2
    /// hours.  Other examples:  200 mcg/min or 200 mcg/1 minute; 1 liter/8 hours.
    pub rate_quantity: super::quantity::Quantity,
    /// Identifies the speed with which the medication was or will be introduced into
    /// the patient.  Typically, the rate for an infusion e.g. 100 ml per 1 hour or
    /// 100 ml/hr.  May also be expressed as a rate per unit of time, e.g. 500 ml per 2
    /// hours.  Other examples:  200 mcg/min or 200 mcg/1 minute; 1 liter/8 hours.
    pub rate_ratio: super::ratio::Ratio,
    /// A code specifying the route or physiological path of administration of a
    /// therapeutic agent into or onto the patient.  For example, topical, intravenous,
    /// etc.
    pub route: super::codeable_concept::CodeableConcept,
    /// A coded specification of the anatomic site where the medication first entered
    /// the body.  For example, "left arm".
    pub site: super::codeable_concept::CodeableConcept,
    /// Free text dosage can be used for cases where the dosage administered is too
    /// complex to code. When coded dosage is present, the free text dosage may still be
    /// present for display to humans.
    ///
    /// The dosage instructions should reflect the dosage of the medication that was
    /// administered.
    pub text: super::string::String,
}

/// Describes the event of a patient consuming or otherwise being administered
/// a medication.  This may be as simple as swallowing a tablet or it may be a
/// long running infusion.  Related resources tie this event to the authorizing
/// prescription, and the specific encounter between patient and health care
/// practitioner.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationAdministrationPerformer {
    /// Indicates who or what performed the medication administration.
    pub actor: super::codeable_reference::CodeableReference,
    /// Distinguishes the type of involvement of the performer in the medication
    /// administration.
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
