/// Describes the event of a patient being administered a vaccine or a record of an
/// immunization as reported by a patient, a clinician or another party.
#[derive(Debug, Clone, PartialEq)]
pub struct Immunization {
    /// An indication of which product was administered to the patient. This is
    /// typically a more detailed representation of the concept conveyed by the
    /// vaccineCode data element. If a Medication resource is referenced, it may be to a
    /// stand-alone resource or a contained resource within the Immunization resource.
    pub administered_product: super::codeable_reference::CodeableReference,
    /// A plan, order or recommendation fulfilled in whole or in part by this
    /// immunization.
    pub based_on: Vec<super::reference::Reference>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The quantity of vaccine product that was administered.
    pub dose_quantity: super::quantity::Quantity,
    /// The visit or admission or other contact between patient and health care provider
    /// the immunization was performed as part of.
    pub encounter: super::reference::Reference,
    /// Date vaccine batch expires.
    pub expiration_date: super::date::Date,
    /// Indicates the source of the vaccine actually administered. This may be different
    /// than the patient eligibility (e.g. the patient may be eligible for a publically
    /// purchased vaccine but due to inventory issues, vaccine purchased with private
    /// funds was actually administered).
    pub funding_source: super::codeable_concept::CodeableConcept,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A unique identifier assigned to this immunization record.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Typically the source of the data when the report of the immunization event is
    /// not based on information from the person who administered the vaccine.
    pub information_source: super::codeable_reference::CodeableReference,
    /// Indication if a dose is considered to be subpotent. By default, a dose should be
    /// considered to be potent.
    pub is_subpotent: super::boolean::Boolean,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The service delivery location where the vaccine administration occurred.
    pub location: super::reference::Reference,
    /// Lot number of the  vaccine product.
    pub lot_number: super::string::String,
    /// Name of vaccine manufacturer.
    pub manufacturer: super::codeable_reference::CodeableReference,
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
    /// Extra information about the immunization that is not conveyed by the other
    /// attributes.
    pub note: Vec<super::annotation::Annotation>,
    /// Date vaccine administered or was to be administered.
    pub occurrence_date_time: String,
    /// Date vaccine administered or was to be administered.
    pub occurrence_string: String,
    /// The patient who either received or did not receive the immunization.
    pub patient: super::reference::Reference,
    /// Indicates who performed the immunization event.
    pub performer: Vec<super::immunization::ImmunizationPerformer>,
    /// Indicates whether the data contained in the resource was captured by the
    /// individual/organization which was responsible for the administration of the
    /// vaccine rather than as 'secondary reported' data documented by a third party.
    /// A value of 'true' means this data originated with the individual/organization
    /// which was responsible for the administration of the vaccine.
    pub primary_source: super::boolean::Boolean,
    /// Indicates a patient's eligibility for a funding program.
    pub program_eligibility: Vec<super::immunization::ImmunizationProgramEligibility>,
    /// The protocol (set of recommendations) being followed by the provider who
    /// administered the dose.
    pub protocol_applied: Vec<super::immunization::ImmunizationProtocolApplied>,
    /// Categorical data indicating that an adverse event is associated in time to an
    /// immunization.
    pub reaction: Vec<super::immunization::ImmunizationReaction>,
    /// Describes why the immunization occurred in coded or textual form, or Indicates
    /// another resource (Condition, Observation or DiagnosticReport) whose existence
    /// justifies this immunization.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// This is a Immunization resource
    pub resource_type: String,
    /// The path by which the vaccine product is taken into the body.
    pub route: super::codeable_concept::CodeableConcept,
    /// Body site where vaccine was administered.
    pub site: super::codeable_concept::CodeableConcept,
    /// Indicates the current status of the immunization event.
    pub status: super::code::Code,
    /// Indicates the reason the immunization event was not performed.
    pub status_reason: super::codeable_concept::CodeableConcept,
    /// Reason why a dose is considered to be subpotent.
    pub subpotent_reason: Vec<super::codeable_concept::CodeableConcept>,
    /// Additional information that is relevant to the immunization (e.g. for a vaccine
    /// recipient who is pregnant, the gestational age of the fetus). The reason why
    /// a vaccine was given (e.g. occupation, underlying medical condition) should
    /// be conveyed in Immunization.reason, not as supporting information. The reason
    /// why a vaccine was not given (e.g. contraindication) should be conveyed in
    /// Immunization.statusReason, not as supporting information.
    pub supporting_information: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Vaccine that was administered or was to be administered.
    pub vaccine_code: super::codeable_concept::CodeableConcept,
}

/// Describes the event of a patient being administered a vaccine or a record of an
/// immunization as reported by a patient, a clinician or another party.
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationPerformer {
    /// The practitioner or organization who performed the action.
    pub actor: super::reference::Reference,
    /// Describes the type of performance (e.g. ordering provider, administering
    /// provider, etc.).
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

/// Describes the event of a patient being administered a vaccine or a record of an
/// immunization as reported by a patient, a clinician or another party.
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationProgramEligibility {
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
    /// Indicates which program the patient had their eligility evaluated for.
    pub program: super::codeable_concept::CodeableConcept,
    /// Indicates the patient's eligility status for for a specific payment program.
    pub program_status: super::codeable_concept::CodeableConcept,
}

/// Describes the event of a patient being administered a vaccine or a record of an
/// immunization as reported by a patient, a clinician or another party.
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationProtocolApplied {
    /// Indicates the authority who published the protocol (e.g. ACIP) that is being
    /// followed.
    pub authority: super::reference::Reference,
    /// Nominal position in a series as intended by the practitioner administering the
    /// dose.
    pub dose_number: super::string::String,
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
    /// One possible path to achieve presumed immunity against a disease - within the
    /// context of an authority.
    pub series: super::string::String,
    /// The recommended number of doses to achieve immunity as intended by the
    /// practitioner administering the dose.
    pub series_doses: super::string::String,
    /// The vaccine preventable disease the dose is being administered against.
    pub target_disease: Vec<super::codeable_concept::CodeableConcept>,
}

/// Describes the event of a patient being administered a vaccine or a record of an
/// immunization as reported by a patient, a clinician or another party.
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationReaction {
    /// Date of reaction to the immunization.
    pub date: super::date_time::DateTime,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Details of the reaction.
    pub manifestation: super::codeable_reference::CodeableReference,
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
    /// Self-reported indicator.
    pub reported: super::boolean::Boolean,
}
