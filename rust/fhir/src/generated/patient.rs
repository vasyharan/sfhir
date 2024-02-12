/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.
#[derive(Debug, Clone, PartialEq)]
pub struct Patient {
    /// Whether this patient record is in active use.
    /// Many systems use this property to mark as non-current patients, such as those
    /// that have not been seen for a period of time based on an organization's business
    /// rules.
    ///
    /// It is often used to filter patient lists to exclude inactive patients
    ///
    /// Deceased patients may also be marked as inactive for the same reasons, but may
    /// be active for some time after death.
    pub active: super::boolean::Boolean,
    /// An address for the individual.
    pub address: Vec<super::address::Address>,
    /// The date of birth for the individual.
    pub birth_date: super::date::Date,
    /// A language which may be used to communicate with the patient about his or her
    /// health.
    pub communication: Vec<super::patient::PatientCommunication>,
    /// A contact party (e.g. guardian, partner, friend) for the patient.
    pub contact: Vec<super::patient::PatientContact>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Indicates if the individual is deceased or not.
    pub deceased_boolean: bool,
    /// Indicates if the individual is deceased or not.
    pub deceased_date_time: String,
    /// Administrative Gender - the gender that the patient is considered to have for
    /// administration and record keeping purposes.
    pub gender: super::code::Code,
    /// Patient's nominated care provider.
    pub general_practitioner: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// An identifier for this patient.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Link to a Patient or RelatedPerson resource that concerns the same actual
    /// individual.
    pub link: Vec<super::patient::PatientLink>,
    /// Organization that is the custodian of the patient record.
    pub managing_organization: super::reference::Reference,
    /// This field contains a patient's most recent marital (civil) status.
    pub marital_status: super::codeable_concept::CodeableConcept,
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
    /// Indicates whether the patient is part of a multiple (boolean) or indicates the
    /// actual birth order (integer).
    pub multiple_birth_boolean: bool,
    /// Indicates whether the patient is part of a multiple (boolean) or indicates the
    /// actual birth order (integer).
    pub multiple_birth_integer: i64,
    /// A name associated with the individual.
    pub name: Vec<super::human_name::HumanName>,
    /// Image of the patient.
    pub photo: Vec<super::attachment::Attachment>,
    /// This is a Patient resource
    pub resource_type: String,
    /// A contact detail (e.g. a telephone number or an email address) by which the
    /// individual may be contacted.
    pub telecom: Vec<super::contact_point::ContactPoint>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.
#[derive(Debug, Clone, PartialEq)]
pub struct PatientCommunication {
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
    /// Indicates whether or not the patient prefers this language (over other languages
    /// he masters up a certain level).
    pub preferred: super::boolean::Boolean,
}

/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.
#[derive(Debug, Clone, PartialEq)]
pub struct PatientContact {
    /// Address for the contact person.
    pub address: super::address::Address,
    /// Administrative Gender - the gender that the contact person is considered to have
    /// for administration and record keeping purposes.
    pub gender: super::code::Code,
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
    /// A name associated with the contact person.
    pub name: super::human_name::HumanName,
    /// Organization on behalf of which the contact is acting or for which the contact
    /// is working.
    pub organization: super::reference::Reference,
    /// The period during which this contact person or organization is valid to be
    /// contacted relating to this patient.
    pub period: super::period::Period,
    /// The nature of the relationship between the patient and the contact person.
    pub relationship: Vec<super::codeable_concept::CodeableConcept>,
    /// A contact detail for the person, e.g. a telephone number or an email address.
    pub telecom: Vec<super::contact_point::ContactPoint>,
}

/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.
#[derive(Debug, Clone, PartialEq)]
pub struct PatientLink {
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
    /// Link to a Patient or RelatedPerson resource that concerns the same actual
    /// individual.
    pub other: super::reference::Reference,
    /// The type of link between this patient resource and another patient resource.
    pub r#type: super::code::Code,
}
