/// A record of an event relevant for purposes such as operations, privacy,
/// security, maintenance, and performance analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEvent {
    /// Indicator for type of action performed during the event that generated the
    /// audit.
    pub action: super::code::Code,
    /// An actor taking an active role in the event or activity that is logged.
    pub agent: Vec<super::audit_event::AuditEventAgent>,
    /// The authorization (e.g., PurposeOfUse) that was used during the event being
    /// recorded.
    pub authorization: Vec<super::codeable_concept::CodeableConcept>,
    /// Allows tracing of authorizatino for the events and tracking whether proposals/
    /// recommendations were acted upon.
    pub based_on: Vec<super::reference::Reference>,
    /// Classification of the type of event.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// Describes what happened. The most specific code for the event.
    pub code: super::codeable_concept::CodeableConcept,
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
    /// Specific instances of data or objects that have been accessed.
    pub entity: Vec<super::audit_event::AuditEventEntity>,
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
    /// The time or period during which the activity occurred.
    pub occurred_date_time: String,
    /// The time or period during which the activity occurred.
    pub occurred_period: super::period::Period,
    /// Indicates whether the event succeeded or failed. A free text descripiton can be
    /// given in outcome.text.
    pub outcome: super::audit_event::AuditEventOutcome,
    /// The patient element is available to enable deterministic tracking of activities
    /// that involve the patient as the subject of the data used in an activity.
    pub patient: super::reference::Reference,
    /// The time when the event was recorded.
    pub recorded: super::instant::Instant,
    /// This is a AuditEvent resource
    pub resource_type: String,
    /// Indicates and enables segmentation of various severity including debugging from
    /// critical.
    pub severity: super::code::Code,
    /// The actor that is reporting the event.
    pub source: super::audit_event::AuditEventSource,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A record of an event relevant for purposes such as operations, privacy,
/// security, maintenance, and performance analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventAgent {
    /// The authorization (e.g., PurposeOfUse) that was used during the event being
    /// recorded.
    pub authorization: Vec<super::codeable_concept::CodeableConcept>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Where the agent location is known, the agent location when the event occurred.
    pub location: super::reference::Reference,
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
    /// When the event utilizes a network there should be an agent describing the
    /// local system, and an agent describing remote system, with the network interface
    /// details.
    pub network_reference: super::reference::Reference,
    /// When the event utilizes a network there should be an agent describing the
    /// local system, and an agent describing remote system, with the network interface
    /// details.
    pub network_string: String,
    /// When the event utilizes a network there should be an agent describing the
    /// local system, and an agent describing remote system, with the network interface
    /// details.
    pub network_uri: String,
    /// Where the policy(ies) are known that authorized the agent participation in the
    /// event. Typically, a single activity may have multiple applicable policies, such
    /// as patient consent, guarantor funding, etc. The policy would also indicate the
    /// security token used.
    pub policy: Vec<super::uri::Uri>,
    /// Indicator that the user is or is not the requestor, or initiator, for the event
    /// being audited.
    pub requestor: super::boolean::Boolean,
    /// The structural roles of the agent indicating the agent's competency. The
    /// security role enabling the agent with respect to the activity.
    pub role: Vec<super::codeable_concept::CodeableConcept>,
    /// The Functional Role of the user when performing the event.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// Reference to who this agent is that was involved in the event.
    pub who: super::reference::Reference,
}

/// A record of an event relevant for purposes such as operations, privacy,
/// security, maintenance, and performance analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventDetail {
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
    /// The type of extra detail provided in the value.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The  value of the extra detail.
    pub value_base_64_binary: String,
    /// The  value of the extra detail.
    pub value_boolean: bool,
    /// The  value of the extra detail.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The  value of the extra detail.
    pub value_date_time: String,
    /// The  value of the extra detail.
    pub value_integer: i64,
    /// The  value of the extra detail.
    pub value_period: super::period::Period,
    /// The  value of the extra detail.
    pub value_quantity: super::quantity::Quantity,
    /// The  value of the extra detail.
    pub value_range: super::range::Range,
    /// The  value of the extra detail.
    pub value_ratio: super::ratio::Ratio,
    /// The  value of the extra detail.
    pub value_string: String,
    /// The  value of the extra detail.
    pub value_time: String,
}

/// A record of an event relevant for purposes such as operations, privacy,
/// security, maintenance, and performance analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventEntity {
    /// The entity is attributed to an agent to express the agent's responsibility for
    /// that entity in the activity. This is most used to indicate when persistence
    /// media (the entity) are used by an agent. For example when importing data from a
    /// device, the device would be described in an entity, and the user importing data
    /// from that media would be indicated as the entity.agent.
    pub agent: Vec<super::audit_event::AuditEventAgent>,
    /// Tagged value pairs for conveying additional information about the entity.
    pub detail: Vec<super::audit_event::AuditEventDetail>,
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
    /// The query parameters for a query-type entities.
    pub query: super::base_64_binary::Base64Binary,
    /// Code representing the role the entity played in the event being audited.
    pub role: super::codeable_concept::CodeableConcept,
    /// Security labels for the identified entity.
    pub security_label: Vec<super::codeable_concept::CodeableConcept>,
    /// Identifies a specific instance of the entity. The reference should be version
    /// specific. This is allowed to be a Parameters resource.
    pub what: super::reference::Reference,
}

/// A record of an event relevant for purposes such as operations, privacy,
/// security, maintenance, and performance analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventOutcome {
    /// Indicates whether the event succeeded or failed.
    pub code: super::coding::Coding,
    /// Additional details about the error. This may be a text description of the error
    /// or a system code that identifies the error.
    pub detail: Vec<super::codeable_concept::CodeableConcept>,
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

/// A record of an event relevant for purposes such as operations, privacy,
/// security, maintenance, and performance analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEventSource {
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
    /// Identifier of the source where the event was detected.
    pub observer: super::reference::Reference,
    /// Logical source location within the healthcare enterprise network.  For example,
    /// a hospital or other provider location within a multi-entity provider group.
    pub site: super::reference::Reference,
    /// Code specifying the type of source where event originated.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
}
