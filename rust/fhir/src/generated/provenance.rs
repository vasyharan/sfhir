/// Provenance of a resource is a record that describes entities and processes
/// involved in producing and delivering or otherwise influencing that resource.
/// Provenance provides a critical foundation for assessing authenticity, enabling
/// trust, and allowing reproducibility. Provenance assertions are a form of
/// contextual metadata and can themselves become important records with their own
/// provenance. Provenance statement indicates clinical significance in terms of
/// confidence in authenticity, reliability, and trustworthiness, integrity, and
/// stage in lifecycle (e.g. Document Completion - has the artifact been legally
/// authenticated), all of which may impact security, privacy, and trust policies.
#[derive(Debug, Clone, PartialEq)]
pub struct Provenance {
    /// An activity is something that occurs over a period of time and acts upon or
    /// with entities; it may include consuming, processing, transforming, modifying,
    /// relocating, using, or generating entities.
    pub activity: super::codeable_concept::CodeableConcept,
    /// An actor taking a role in an activity  for which it can be assigned some degree
    /// of responsibility for the activity taking place.
    pub agent: Vec<super::provenance::ProvenanceAgent>,
    /// The authorization (e.g., PurposeOfUse) that was used during the event being
    /// recorded.
    pub authorization: Vec<super::codeable_reference::CodeableReference>,
    /// Allows tracing of authorizatino for the events and tracking whether proposals/
    /// recommendations were acted upon.
    pub based_on: Vec<super::reference::Reference>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// This will typically be the encounter the event occurred, but some events may be
    /// initiated prior to or after the official completion of an encounter but still be
    /// tied to the context of the encounter (e.g. pre-admission lab tests).
    pub encounter: super::reference::Reference,
    /// An entity used in this activity.
    pub entity: Vec<super::provenance::ProvenanceEntity>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Where the activity occurred, if relevant.
    pub location: super::reference::Reference,
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
    /// The period during which the activity occurred.
    pub occurred_date_time: String,
    /// The period during which the activity occurred.
    pub occurred_period: super::period::Period,
    /// The patient element is available to enable deterministic tracking of activities
    /// that involve the patient as the subject of the data used in an activity.
    pub patient: super::reference::Reference,
    /// Policy or plan the activity was defined by. Typically, a single activity may
    /// have multiple applicable policy documents, such as patient consent, guarantor
    /// funding, etc.
    pub policy: Vec<super::uri::Uri>,
    /// The instant of time at which the activity was recorded.
    pub recorded: super::instant::Instant,
    /// This is a Provenance resource
    pub resource_type: String,
    /// A digital signature on the target Reference(s). The signer should match a
    /// Provenance.agent. The purpose of the signature is indicated.
    pub signature: Vec<super::signature::Signature>,
    /// The Reference(s) that were generated or updated by  the activity described
    /// in this resource. A provenance can point to more than one target if multiple
    /// resources were created/updated by the same activity.
    pub target: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// Provenance of a resource is a record that describes entities and processes
/// involved in producing and delivering or otherwise influencing that resource.
/// Provenance provides a critical foundation for assessing authenticity, enabling
/// trust, and allowing reproducibility. Provenance assertions are a form of
/// contextual metadata and can themselves become important records with their own
/// provenance. Provenance statement indicates clinical significance in terms of
/// confidence in authenticity, reliability, and trustworthiness, integrity, and
/// stage in lifecycle (e.g. Document Completion - has the artifact been legally
/// authenticated), all of which may impact security, privacy, and trust policies.
#[derive(Debug, Clone, PartialEq)]
pub struct ProvenanceAgent {
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
    /// The agent that delegated authority to perform the activity performed by the
    /// agent.who element.
    pub on_behalf_of: super::reference::Reference,
    /// The structural roles of the agent indicating the agent's competency. The
    /// security role enabling the agent with respect to the activity.
    pub role: Vec<super::codeable_concept::CodeableConcept>,
    /// The Functional Role of the agent with respect to the activity.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// Indicates who or what performed in the event.
    pub who: super::reference::Reference,
}

/// Provenance of a resource is a record that describes entities and processes
/// involved in producing and delivering or otherwise influencing that resource.
/// Provenance provides a critical foundation for assessing authenticity, enabling
/// trust, and allowing reproducibility. Provenance assertions are a form of
/// contextual metadata and can themselves become important records with their own
/// provenance. Provenance statement indicates clinical significance in terms of
/// confidence in authenticity, reliability, and trustworthiness, integrity, and
/// stage in lifecycle (e.g. Document Completion - has the artifact been legally
/// authenticated), all of which may impact security, privacy, and trust policies.
#[derive(Debug, Clone, PartialEq)]
pub struct ProvenanceEntity {
    /// The entity is attributed to an agent to express the agent's responsibility
    /// for that entity, possibly along with other agents. This description can be
    /// understood as shorthand for saying that the agent was responsible for the
    /// activity which used the entity.
    pub agent: Vec<super::provenance::ProvenanceAgent>,
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
    /// How the entity was used during the activity.
    pub role: super::code::Code,
    /// Identity of the  Entity used. May be a logical or physical uri and maybe
    /// absolute or relative.
    pub what: super::reference::Reference,
}
