/// Describes a stream of resource state changes or events and annotated with labels
/// useful to filter projections from this topic.
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionTopic {
    /// The date on which the asset content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// List of properties by which Subscriptions on the SubscriptionTopic can be
    /// filtered. May be defined Search Parameters (e.g., Encounter.patient) or
    /// parameters defined within this SubscriptionTopic context (e.g., hub.event).
    pub can_filter_by: Vec<super::subscription_topic::SubscriptionTopicCanFilterBy>,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the SubscriptionTopic and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the SubscriptionTopic.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date (and optionally time) when the subscription topic was last
    /// significantly changed. The date must change when the business version changes
    /// and it must change if the status code changes. In addition, it should change
    /// when the substantive content of the subscription topic changes.
    pub date: super::date_time::DateTime,
    /// The canonical URL pointing to another FHIR-defined SubscriptionTopic that is
    /// adhered to in whole or in part by this SubscriptionTopic.
    pub derived_from: Vec<super::canonical::Canonical>,
    /// A free text natural language description of the Topic from the consumer's
    /// perspective.
    pub description: super::markdown::Markdown,
    /// The period during which the SubscriptionTopic content was or is planned to be
    /// effective.
    pub effective_period: super::period::Period,
    /// Event definition which can be used to trigger the SubscriptionTopic.
    pub event_trigger: Vec<super::subscription_topic::SubscriptionTopicEventTrigger>,
    /// A flag to indicate that this TopSubscriptionTopicic is authored for testing
    /// purposes (or education/evaluation/marketing), and is not intended to be used for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this subscription topic by the performer and/or
    /// other systems.  These identifiers remain constant as the resource is updated and
    /// propagates from server to server.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A jurisdiction in which the Topic is intended to be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The date on which the asset content was last reviewed. Review happens
    /// periodically after that, but doesn't change the original approval date.
    pub last_review_date: super::date::Date,
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
    /// A natural language name identifying the subscription topic This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub name: super::string::String,
    /// List of properties to describe the shape (e.g., resources) included in
    /// notifications from this Subscription Topic.
    pub notification_shape: Vec<super::subscription_topic::SubscriptionTopicNotificationShape>,
    /// Helps establish the "authority/credibility" of the SubscriptionTopic.  May also
    /// allow for contact.
    pub publisher: super::string::String,
    /// Explains why this Topic is needed and why it has been designed as it has.
    pub purpose: super::markdown::Markdown,
    /// A definition of a resource-based event that triggers a notification based on
    /// the SubscriptionTopic. The criteria may be just a human readable description
    /// and/or a full FHIR search string or FHIRPath expression. Multiple triggers are
    /// considered OR joined (e.g., a resource update matching ANY of the definitions
    /// will trigger a notification).
    pub resource_trigger: Vec<super::subscription_topic::SubscriptionTopicResourceTrigger>,
    /// This is a SubscriptionTopic resource
    pub resource_type: String,
    /// The current state of the SubscriptionTopic.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the subscription topic.  For
    /// example, "admission".
    pub title: super::string::String,
    /// An absolute URI that is used to identify this subscription topic when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this subscription topic is
    /// (or will be) published. This URL can be the target of a canonical reference.
    /// It SHALL remain the same when the subscription topic is stored on different
    /// servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These terms may be used to assist with indexing and searching
    /// of code system definitions.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the subscription topic
    /// when it is referenced in a specification, model, design or instance. This
    /// is an arbitrary value managed by the Topic author and is not expected to be
    /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions
    /// are orderable.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// Describes a stream of resource state changes or events and annotated with labels
/// useful to filter projections from this topic.
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionTopicCanFilterBy {
    /// Comparators allowed for the filter parameter.
    pub comparator: Vec<super::code::Code>,
    /// Description of how this filtering parameter is intended to be used.
    pub description: super::markdown::Markdown,
    /// Either the canonical URL to a search parameter (like "http://hl7.org/fhir/
    /// SearchParameter/encounter-patient") or the officially-defined URI for a shared
    /// filter concept (like "http://example.org/concepts/shared-common-event").
    pub filter_definition: super::uri::Uri,
    /// Either the canonical URL to a search parameter (like "http://hl7.org/
    /// fhir/SearchParameter/encounter-patient") or topic-defined parameter (like
    /// "hub.event") which is a label for the filter.
    pub filter_parameter: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Modifiers allowed for the filter parameter.
    pub modifier: Vec<super::code::Code>,
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
    /// URL of the Resource that is the type used in this filter. This is the "focus"
    /// of the topic (or one of them if there are more than one). It will be the same,
    /// a generality, or a specificity of SubscriptionTopic.resourceTrigger.resource or
    /// SubscriptionTopic.eventTrigger.resource when they are present.
    pub resource: super::uri::Uri,
}

/// Describes a stream of resource state changes or events and annotated with labels
/// useful to filter projections from this topic.
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionTopicEventTrigger {
    /// The human readable description of an event to trigger a notification for the
    /// SubscriptionTopic - for example, "Patient Admission, as defined in HL7v2 via
    /// message ADT^A01". Multiple values are considered OR joined (e.g., matching any
    /// single event listed).
    pub description: super::markdown::Markdown,
    /// A well-defined event which can be used to trigger notifications from the
    /// SubscriptionTopic.
    pub event: super::codeable_concept::CodeableConcept,
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
    /// URL of the Resource that is the focus type used in this event trigger.
    /// Relative URLs are relative to the StructureDefinition root of the
    /// implemented FHIR version (e.g., http://hl7.org/fhir/StructureDefinition).
    /// For example, "Patient" maps to http://hl7.org/fhir/StructureDefinition/
    /// Patient.  For more information, see <a href="elementdefinition-
    /// definitions.html#ElementDefinition.type.code">ElementDefinition.type.code</a>.
    pub resource: super::uri::Uri,
}

/// Describes a stream of resource state changes or events and annotated with labels
/// useful to filter projections from this topic.
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionTopicNotificationShape {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Search-style _include directives, rooted in the resource for this shape. Servers
    /// SHOULD include resources listed here, if they exist and the user is authorized
    /// to receive them.  Clients SHOULD be prepared to receive these additional
    /// resources, but SHALL function properly without them.
    pub include: Vec<super::string::String>,
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
    /// URL of the Resource that is the type used in this shape. This is the
    /// 'focus' resource of the topic (or one of them if there are more than one)
    /// and the root resource for this shape definition. It will be the same, a
    /// generality, or a specificity of SubscriptionTopic.resourceTrigger.resource or
    /// SubscriptionTopic.eventTrigger.resource when they are present.
    pub resource: super::uri::Uri,
    /// Search-style _revinclude directives, rooted in the resource for this shape.
    /// Servers SHOULD include resources listed here, if they exist and the user
    /// is authorized to receive them.  Clients SHOULD be prepared to receive these
    /// additional resources, but SHALL function properly without them.
    pub rev_include: Vec<super::string::String>,
}

/// Describes a stream of resource state changes or events and annotated with labels
/// useful to filter projections from this topic.
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionTopicQueryCriteria {
    /// The FHIR query based rules are applied to the current resource state (e.g.,
    /// state after an update).
    pub current: super::string::String,
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
    /// The FHIR query based rules are applied to the previous resource state (e.g.,
    /// state before an update).
    pub previous: super::string::String,
    /// If set to `true`, both the `current` and `previous` query criteria must evaluate
    /// `true` to trigger a notification for this topic.  If set to `false` or not
    /// present, a notification for this topic will be triggered if either the `current`
    /// or `previous` tests evaluate to `true`.
    pub require_both: super::boolean::Boolean,
    /// For `create` interactions, should the `previous` criteria count as an automatic
    /// pass or an automatic fail. If not present, the testing behavior during `create`
    /// interactions is unspecified (server discretion).
    pub result_for_create: super::code::Code,
    /// For 'delete' interactions, should the 'current' query criteria count as an
    /// automatic pass or an automatic fail. If not present, the testing behavior during
    /// `delete` interactions is unspecified (server discretion).
    pub result_for_delete: super::code::Code,
}

/// Describes a stream of resource state changes or events and annotated with labels
/// useful to filter projections from this topic.
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionTopicResourceTrigger {
    /// The human readable description of this resource trigger for the
    /// SubscriptionTopic -  for example, "An Encounter enters the 'in-progress' state".
    pub description: super::markdown::Markdown,
    /// The FHIRPath based rules that the server should use to determine when to trigger
    /// a notification for this topic.
    pub fhir_path_criteria: super::string::String,
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
    /// The FHIR query based rules that the server should use to determine when to
    /// trigger a notification for this subscription topic.
    pub query_criteria: super::subscription_topic::SubscriptionTopicQueryCriteria,
    /// URL of the Resource that is the type used in this resource trigger.
    /// Relative URLs are relative to the StructureDefinition root of the
    /// implemented FHIR version (e.g., http://hl7.org/fhir/StructureDefinition).
    /// For example, "Patient" maps to http://hl7.org/fhir/StructureDefinition/
    /// Patient.  For more information, see <a href="elementdefinition-
    /// definitions.html#ElementDefinition.type.code">ElementDefinition.type.code</a>.
    pub resource: super::uri::Uri,
    /// The FHIR RESTful interaction which can be used to trigger a notification for
    /// the SubscriptionTopic. Multiple values are considered OR joined (e.g., CREATE or
    /// UPDATE). If not present, all supported interactions are assumed.
    pub supported_interaction: Vec<super::code::Code>,
}
