/// The subscription resource describes a particular client's request to be notified
/// about a SubscriptionTopic.
#[derive(Debug, Clone, PartialEq)]
pub struct Subscription {
    /// The type of channel to send notifications on.
    pub channel_type: super::coding::Coding,
    /// Contact details for a human to contact about the subscription. The primary use
    /// of this for system administrator troubleshooting.
    pub contact: Vec<super::contact_point::ContactPoint>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// How much of the resource content to deliver in the notification payload.
    /// The choices are an empty payload, only the resource id, or the full resource
    /// content.
    pub content: super::code::Code,
    /// The MIME type to send the payload in - e.g., `application/fhir+xml` or
    /// `application/fhir+json`. Note that:
    ///
    /// * clients may request notifications in a specific FHIR version by using the
    /// [FHIR Version Parameter](http.html#version-parameter) - e.g., `application/
    /// fhir+json; fhirVersion=4.0`.
    ///
    /// * additional MIME types can be allowed by channels - e.g., `text/plain` and
    /// `text/html` are defined by the Email channel.
    pub content_type: super::code::Code,
    /// The time for the server to turn the subscription off.
    pub end: super::instant::Instant,
    /// The url that describes the actual end-point to send notifications to.
    pub endpoint: super::url::Url,
    /// The filter properties to be applied to narrow the subscription topic stream.
    /// When multiple filters are applied, evaluates to true if all the conditions
    /// applicable to that resource are met; otherwise it returns false (i.e., logical
    /// AND).
    pub filter_by: Vec<super::subscription::SubscriptionFilterBy>,
    /// If present, a 'heartbeat' notification (keep-alive) is sent via this channel
    /// with an interval period equal to this elements integer value in seconds.  If not
    /// present, a heartbeat notification is not sent.
    pub heartbeat_period: super::unsigned_int::UnsignedInt,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this code system when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Entity with authorization to make subsequent revisions to the Subscription and
    /// also determines what data the subscription is authorized to disclose.
    pub managing_entity: super::reference::Reference,
    /// If present, the maximum number of events that will be included in a notification
    /// bundle. Note that this is not a strict limit on the number of entries in a
    /// bundle, as dependent resources can be included.
    pub max_count: super::positive_int::PositiveInt,
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
    /// A natural language name identifying the subscription.
    pub name: super::string::String,
    /// Channel-dependent information to send as part of the notification (e.g., HTTP
    /// Headers).
    pub parameter: Vec<super::subscription::SubscriptionParameter>,
    /// A description of why this subscription is defined.
    pub reason: super::string::String,
    /// This is a Subscription resource
    pub resource_type: String,
    /// The status of the subscription, which marks the server state for managing the
    /// subscription.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// If present, the maximum amount of time a server will allow before failing a
    /// notification attempt.
    pub timeout: super::unsigned_int::UnsignedInt,
    /// The reference to the subscription topic to be notified about.
    pub topic: super::canonical::Canonical,
}

/// The subscription resource describes a particular client's request to be notified
/// about a SubscriptionTopic.
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionFilterBy {
    /// Comparator applied to this filter parameter.
    pub comparator: super::code::Code,
    /// The filter as defined in the `SubscriptionTopic.canFilterBy.filterParameter`
    /// element.
    pub filter_parameter: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Modifier applied to this filter parameter.
    pub modifier: super::code::Code,
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
    /// A resource listed in the `SubscriptionTopic` this `Subscription` references
    /// (`SubscriptionTopic.canFilterBy.resource`). This element can be used to
    /// differentiate filters for topics that include more than one resource type.
    pub resource_type: super::uri::Uri,
    /// The literal value or resource path as is legal in search - for example,
    /// `Patient/123` or `le1950`.
    pub value: super::string::String,
}

/// The subscription resource describes a particular client's request to be notified
/// about a SubscriptionTopic.
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionParameter {
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
    /// Parameter name for information passed to the channel for notifications, for
    /// example in the case of a REST hook wanting to pass through an authorization
    /// header, the name would be Authorization.
    pub name: super::string::String,
    /// Parameter value for information passed to the channel for notifications, for
    /// example in the case of a REST hook wanting to pass through an authorization
    /// header, the value would be `Bearer 0193...`.
    pub value: super::string::String,
}
