/// Indicates how the medication is/was taken or should be taken by the patient.
#[derive(Debug, Clone, PartialEq)]
pub struct Dosage {
    /// Supplemental instructions to the patient on how to take the medication  (e.g.
    /// "with meals" or"take half to one hour before food") or warnings for the patient
    /// about the medication (e.g. "may cause drowsiness" or "avoid exposure of skin to
    /// direct sunlight or sunlamps").
    pub additional_instruction: Vec<super::codeable_concept::CodeableConcept>,
    /// Indicates whether the Medication is only taken when needed within a specific
    /// dosing schedule (Boolean option).
    pub as_needed: super::boolean::Boolean,
    /// Indicates whether the Medication is only taken based on a precondition for
    /// taking the Medication (CodeableConcept).
    pub as_needed_for: Vec<super::codeable_concept::CodeableConcept>,
    /// Depending on the resource,this is the amount of medication administered, to  be
    /// administered or typical amount to be administered.
    pub dose_and_rate: Vec<super::dosage::DosageDoseAndRate>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Upper limit on medication per administration.
    pub max_dose_per_administration: super::quantity::Quantity,
    /// Upper limit on medication per lifetime of the patient.
    pub max_dose_per_lifetime: super::quantity::Quantity,
    /// Upper limit on medication per unit of time.
    pub max_dose_per_period: Vec<super::ratio::Ratio>,
    /// Technique for administering medication.
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
    /// Instructions in terms that are understood by the patient or consumer.
    pub patient_instruction: super::string::String,
    /// How drug should enter body.
    pub route: super::codeable_concept::CodeableConcept,
    /// Indicates the order in which the dosage instructions should be applied or
    /// interpreted.
    pub sequence: super::integer::Integer,
    /// Body site to administer to.
    pub site: super::codeable_concept::CodeableConcept,
    /// Free text dosage instructions e.g. SIG.
    pub text: super::string::String,
    /// When medication should be administered.
    pub timing: super::timing::Timing,
}

/// Indicates how the medication is/was taken or should be taken by the patient.
#[derive(Debug, Clone, PartialEq)]
pub struct DosageDoseAndRate {
    /// Amount of medication per dose.
    pub dose_quantity: super::quantity::Quantity,
    /// Amount of medication per dose.
    pub dose_range: super::range::Range,
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
    /// Amount of medication per unit of time.
    pub rate_quantity: super::quantity::Quantity,
    /// Amount of medication per unit of time.
    pub rate_range: super::range::Range,
    /// Amount of medication per unit of time.
    pub rate_ratio: super::ratio::Ratio,
    /// The kind of dose or rate specified, for example, ordered or calculated.
    pub r#type: super::codeable_concept::CodeableConcept,
}
