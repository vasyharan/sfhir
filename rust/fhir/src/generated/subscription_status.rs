/// The SubscriptionStatus resource describes the state of a Subscription during
/// notifications.
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionStatus {
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A record of errors that occurred when the server processed a notification.
    pub error: Vec<super::codeable_concept::CodeableConcept>,
    /// The total number of actual events which have been generated since the
    /// Subscription was created (inclusive of this notification) - regardless of how
    /// many have been successfully communicated.  This number is NOT incremented for
    /// handshake and heartbeat notifications.
    pub events_since_subscription_start: super::integer_64::Integer64,
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
    /// Detailed information about events relevant to this subscription notification.
    pub notification_event: Vec<super::subscription_status::SubscriptionStatusNotificationEvent>,
    /// This is a SubscriptionStatus resource
    pub resource_type: String,
    /// The status of the subscription, which marks the server state for managing the
    /// subscription.
    pub status: super::code::Code,
    /// The reference to the Subscription which generated this notification.
    pub subscription: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// The reference to the SubscriptionTopic for the Subscription which generated
    /// this notification.
    pub topic: super::canonical::Canonical,
    /// The type of event being conveyed with this notification.
    pub r#type: super::code::Code,
}

/// The SubscriptionStatus resource describes the state of a Subscription during
/// notifications.
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionStatusNotificationEvent {
    /// Additional context information for this event. Generally, this will contain
    /// references to additional resources included with the event (e.g., the Patient
    /// relevant to an Encounter), however it MAY refer to non-FHIR objects.
    pub additional_context: Vec<super::reference::Reference>,
    /// Either the sequential number of this event in this subscription context or a
    /// relative event number for this notification.
    pub event_number: super::integer_64::Integer64,
    /// The focus of this event. While this will usually be a reference to the focus
    /// resource of the event, it MAY contain a reference to a non-FHIR object.
    pub focus: super::reference::Reference,
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
    /// The actual time this event occurred on the server.
    pub timestamp: super::instant::Instant,
}