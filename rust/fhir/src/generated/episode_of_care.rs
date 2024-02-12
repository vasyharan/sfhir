/// An association between a patient and an organization / healthcare provider(s)
/// during which time encounters may occur. The managing organization assumes a
/// level of responsibility for the patient during this time.
#[derive(Debug, Clone, PartialEq)]
pub struct EpisodeOfCare {
    /// The set of accounts that may be used for billing for this EpisodeOfCare.
    pub account: Vec<super::reference::Reference>,
    /// The practitioner that is the care manager/care coordinator for this patient.
    pub care_manager: super::reference::Reference,
    /// The list of practitioners that may be facilitating this episode of care for
    /// specific purposes.
    pub care_team: Vec<super::reference::Reference>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The list of medical conditions that were addressed during the episode of care.
    pub diagnosis: Vec<super::episode_of_care::EpisodeOfCareDiagnosis>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// The EpisodeOfCare may be known by different identifiers for different contexts
    /// of use, such as when an external agency is tracking the Episode for funding
    /// purposes.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The organization that has assumed the specific responsibilities for care
    /// coordination, care delivery, or other services for the specified duration.
    pub managing_organization: super::reference::Reference,
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
    /// The patient who is the focus of this episode of care.
    pub patient: super::reference::Reference,
    /// The interval during which the managing organization assumes the defined
    /// responsibility.
    pub period: super::period::Period,
    /// The list of medical reasons that are expected to be addressed during the episode
    /// of care.
    pub reason: Vec<super::episode_of_care::EpisodeOfCareReason>,
    /// Referral Request(s) that are fulfilled by this EpisodeOfCare, incoming
    /// referrals.
    pub referral_request: Vec<super::reference::Reference>,
    /// This is a EpisodeOfCare resource
    pub resource_type: String,
    /// planned | waitlist | active | onhold | finished | cancelled.
    pub status: super::code::Code,
    /// The history of statuses that the EpisodeOfCare has been through (without
    /// requiring processing the history of the resource).
    pub status_history: Vec<super::episode_of_care::EpisodeOfCareStatusHistory>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A classification of the type of episode of care; e.g. specialist referral,
    /// disease management, type of funded care.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
}

/// An association between a patient and an organization / healthcare provider(s)
/// during which time encounters may occur. The managing organization assumes a
/// level of responsibility for the patient during this time.
#[derive(Debug, Clone, PartialEq)]
pub struct EpisodeOfCareDiagnosis {
    /// The medical condition that was addressed during the episode of care, expressed
    /// as a text, code or a reference to another resource.
    pub condition: Vec<super::codeable_reference::CodeableReference>,
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
    /// Role that this diagnosis has within the episode of care (e.g. admission,
    /// billing, discharge …).
    pub r#use: super::codeable_concept::CodeableConcept,
}

/// An association between a patient and an organization / healthcare provider(s)
/// during which time encounters may occur. The managing organization assumes a
/// level of responsibility for the patient during this time.
#[derive(Debug, Clone, PartialEq)]
pub struct EpisodeOfCareReason {
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
    /// What the reason value should be used as e.g. Chief Complaint, Health Concern,
    /// Health Maintenance (including screening).
    pub r#use: super::codeable_concept::CodeableConcept,
    /// The medical reason that is expected to be addressed during the episode of care,
    /// expressed as a text, code or a reference to another resource.
    pub value: Vec<super::codeable_reference::CodeableReference>,
}

/// An association between a patient and an organization / healthcare provider(s)
/// during which time encounters may occur. The managing organization assumes a
/// level of responsibility for the patient during this time.
#[derive(Debug, Clone, PartialEq)]
pub struct EpisodeOfCareStatusHistory {
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
    /// The period during this EpisodeOfCare that the specific status applied.
    pub period: super::period::Period,
    /// planned | waitlist | active | onhold | finished | cancelled.
    pub status: super::code::Code,
}
