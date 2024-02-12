/// Information about a person that is involved in a patient's health or the
/// care for a patient, but who is not the target of healthcare, nor has a formal
/// responsibility in the care process.
#[derive(Debug, Clone, PartialEq)]
pub struct RelatedPerson {
    /// Whether this related person record is in active use.
    pub active: super::boolean::Boolean,
    /// Address where the related person can be contacted or visited.
    pub address: Vec<super::address::Address>,
    /// The date on which the related person was born.
    pub birth_date: super::date::Date,
    /// A language which may be used to communicate with the related person about the
    /// patient's health.
    pub communication: Vec<super::related_person::RelatedPersonCommunication>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Administrative Gender - the gender that the person is considered to have for
    /// administration and record keeping purposes.
    pub gender: super::code::Code,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifier for a person within a particular scope.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
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
    /// A name associated with the person.
    pub name: Vec<super::human_name::HumanName>,
    /// The patient this person is related to.
    pub patient: super::reference::Reference,
    /// The period of time during which this relationship is or was active. If there are
    /// no dates defined, then the interval is unknown.
    pub period: super::period::Period,
    /// Image of the person.
    pub photo: Vec<super::attachment::Attachment>,
    /// The nature of the relationship between the related person and the patient.
    pub relationship: Vec<super::codeable_concept::CodeableConcept>,
    /// This is a RelatedPerson resource
    pub resource_type: String,
    /// A contact detail for the person, e.g. a telephone number or an email address.
    pub telecom: Vec<super::contact_point::ContactPoint>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// Information about a person that is involved in a patient's health or the
/// care for a patient, but who is not the target of healthcare, nor has a formal
/// responsibility in the care process.
#[derive(Debug, Clone, PartialEq)]
pub struct RelatedPersonCommunication {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The ISO-639-1 alpha 2 code in lower case for the language, optionally followed
    /// by a hyphen and the ISO-3166-1 alpha 2 code for the region in upper case; e.g.
    /// "en" for English, or "en-US" for American English versus "en-AU" for Australian
    /// English.
    pub language: super::codeable_concept::CodeableConcept,
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
    /// Indicates whether or not the related person prefers this language (over other
    /// languages he or she masters up a certain level).
    pub preferred: super::boolean::Boolean,
}
