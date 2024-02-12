/// Risk of harmful or undesirable physiological response which is specific to an
/// individual and associated with exposure to a substance.
#[derive(Debug, Clone, PartialEq)]
pub struct AllergyIntolerance {
    /// Category of the identified substance.
    pub category: Vec<super::code::Code>,
    /// The clinical status of the allergy or intolerance.
    pub clinical_status: super::codeable_concept::CodeableConcept,
    /// Code for an allergy or intolerance statement (either a positive or a negated/
    /// excluded statement).  This may be a code for a substance or pharmaceutical
    /// product that is considered to be responsible for the adverse reaction risk
    /// (e.g., "Latex"), an allergy or intolerance condition (e.g., "Latex allergy"),
    /// or a negated/excluded code for a specific substance or class (e.g., "No latex
    /// allergy") or a general or categorical negated statement (e.g.,  "No known
    /// allergy", "No known drug allergies").  Note: the substance for a specific
    /// reaction may be different from the substance identified as the cause of the
    /// risk, but it must be consistent with it. For instance, it may be a more specific
    /// substance (e.g. a brand medication) or a composite product that includes the
    /// identified substance. It must be clinically safe to only process the 'code'
    /// and ignore the 'reaction.substance'.  If a receiving system is unable to
    /// confirm that AllergyIntolerance.reaction.substance falls within the semantic
    /// scope of AllergyIntolerance.code, then the receiving system should ignore
    /// AllergyIntolerance.reaction.substance.
    pub code: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Estimate of the potential clinical harm, or seriousness, of the reaction to the
    /// identified substance.
    pub criticality: super::code::Code,
    /// The encounter when the allergy or intolerance was asserted.
    pub encounter: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this AllergyIntolerance by the performer or
    /// other systems which remain constant as the resource is updated and propagates
    /// from server to server.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Represents the date and/or time of the last known occurrence of a reaction
    /// event.
    pub last_occurrence: super::date_time::DateTime,
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
    /// Additional narrative about the propensity for the Adverse Reaction, not captured
    /// in other fields.
    pub note: Vec<super::annotation::Annotation>,
    /// Estimated or actual date,  date-time, or age when allergy or intolerance was
    /// identified.
    pub onset_age: super::age::Age,
    /// Estimated or actual date,  date-time, or age when allergy or intolerance was
    /// identified.
    pub onset_date_time: String,
    /// Estimated or actual date,  date-time, or age when allergy or intolerance was
    /// identified.
    pub onset_period: super::period::Period,
    /// Estimated or actual date,  date-time, or age when allergy or intolerance was
    /// identified.
    pub onset_range: super::range::Range,
    /// Estimated or actual date,  date-time, or age when allergy or intolerance was
    /// identified.
    pub onset_string: String,
    /// Indicates who or what participated in the activities related to the allergy or
    /// intolerance and how they were involved.
    pub participant: Vec<super::allergy_intolerance::AllergyIntoleranceParticipant>,
    /// The patient who has the allergy or intolerance.
    pub patient: super::reference::Reference,
    /// Details about each adverse reaction event linked to exposure to the identified
    /// substance.
    pub reaction: Vec<super::allergy_intolerance::AllergyIntoleranceReaction>,
    /// The recordedDate represents when this particular AllergyIntolerance record was
    /// created in the system, which is often a system-generated date.
    pub recorded_date: super::date_time::DateTime,
    /// This is a AllergyIntolerance resource
    pub resource_type: String,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Identification of the underlying physiological mechanism for the reaction risk.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// Assertion about certainty associated with the propensity, or potential risk, of
    /// a reaction to the identified substance (including pharmaceutical product).  The
    /// verification status pertains to the allergy or intolerance, itself, not to any
    /// specific AllergyIntolerance attribute.
    pub verification_status: super::codeable_concept::CodeableConcept,
}

/// Risk of harmful or undesirable physiological response which is specific to an
/// individual and associated with exposure to a substance.
#[derive(Debug, Clone, PartialEq)]
pub struct AllergyIntoleranceParticipant {
    /// Indicates who or what participated in the activities related to the allergy
    /// or intolerance.
    pub actor: super::reference::Reference,
    /// Distinguishes the type of involvement of the actor in the activities related to
    /// the allergy or intolerance.
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

/// Risk of harmful or undesirable physiological response which is specific to an
/// individual and associated with exposure to a substance.
#[derive(Debug, Clone, PartialEq)]
pub struct AllergyIntoleranceReaction {
    /// Text description about the reaction as a whole, including details of the
    /// manifestation if required.
    pub description: super::string::String,
    /// Identification of the route by which the subject was exposed to the substance.
    pub exposure_route: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Clinical symptoms and/or signs that are observed or associated with the adverse
    /// reaction event.
    pub manifestation: Vec<super::codeable_reference::CodeableReference>,
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
    /// Additional text about the adverse reaction event not captured in other fields.
    pub note: Vec<super::annotation::Annotation>,
    /// Record of the date and/or time of the onset of the Reaction.
    pub onset: super::date_time::DateTime,
    /// Clinical assessment of the severity of the reaction event as a whole,
    /// potentially considering multiple different manifestations.
    pub severity: super::code::Code,
    /// Identification of the specific substance (or pharmaceutical product) considered
    /// to be responsible for the Adverse Reaction event. Note: the substance for a
    /// specific reaction may be different from the substance identified as the cause
    /// of the risk, but it must be consistent with it. For instance, it may be a
    /// more specific substance (e.g. a brand medication) or a composite product that
    /// includes the identified substance. It must be clinically safe to only process
    /// the 'code' and ignore the 'reaction.substance'.  If a receiving system is unable
    /// to confirm that AllergyIntolerance.reaction.substance falls within the semantic
    /// scope of AllergyIntolerance.code, then the receiving system should ignore
    /// AllergyIntolerance.reaction.substance.
    pub substance: super::codeable_concept::CodeableConcept,
}
