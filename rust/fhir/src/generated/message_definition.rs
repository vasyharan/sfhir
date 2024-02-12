/// Defines the characteristics of a message that can be shared between systems,
/// including the type of event that initiates the message, the content to be
/// transmitted and what response(s), if any, are permitted.
#[derive(Debug, Clone, PartialEq)]
pub struct MessageDefinition {
    /// Indicates what types of messages may be sent as an application-level response to
    /// this message.
    pub allowed_response: Vec<super::message_definition::MessageDefinitionAllowedResponse>,
    /// The MessageDefinition that is the basis for the contents of this resource.
    pub base: super::canonical::Canonical,
    /// The impact of the content of the message.
    pub category: super::code::Code,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the message definition and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the message definition.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the message definition was last
    /// significantly changed. The date must change when the business version changes
    /// and it must change if the status code changes. In addition, it should change
    /// when the substantive content of the message definition changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the message definition from a
    /// consumer's perspective.
    pub description: super::markdown::Markdown,
    /// Event code or link to the EventDefinition.
    pub event_coding: super::coding::Coding,
    /// Event code or link to the EventDefinition.
    pub event_uri: String,
    /// A Boolean value to indicate that this message definition is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// Identifies the resource (or resources) that are being addressed by the event.
    /// For example, the Encounter for an admit message or two Account records for
    /// a merge.
    pub focus: Vec<super::message_definition::MessageDefinitionFocus>,
    /// Graph is Canonical reference to a GraphDefinition. If a URL is provided, it is
    /// the canonical reference to a GraphDefinition that it controls what additional
    /// resources are to be added to the Bundle when building the message. The
    /// GraphDefinition can also specify profiles that apply to the various resources.
    pub graph: super::canonical::Canonical,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this message definition when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the message definition is intended to
    /// be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
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
    /// A natural language name identifying the message definition. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub name: super::string::String,
    /// Identifies a protocol or workflow that this MessageDefinition represents a step
    /// in.
    pub parent: Vec<super::canonical::Canonical>,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the message definition.
    pub publisher: super::string::String,
    /// Explanation of why this message definition is needed and why it has been
    /// designed as it has.
    pub purpose: super::markdown::Markdown,
    /// A MessageDefinition that is superseded by this definition.
    pub replaces: Vec<super::canonical::Canonical>,
    /// This is a MessageDefinition resource
    pub resource_type: String,
    /// Declare at a message definition level whether a response is required or only
    /// upon error or success, or never.
    pub response_required: super::code::Code,
    /// The status of this message definition. Enables tracking the life-cycle of the
    /// content.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the message definition.
    pub title: super::string::String,
    /// The business identifier that is used to reference the MessageDefinition and *is*
    /// expected to be consistent from server to server.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate message definition
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the message definition
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the message definition author and is not expected to
    /// be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// Defines the characteristics of a message that can be shared between systems,
/// including the type of event that initiates the message, the content to be
/// transmitted and what response(s), if any, are permitted.
#[derive(Debug, Clone, PartialEq)]
pub struct MessageDefinitionAllowedResponse {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A reference to the message definition that must be adhered to by this supported
    /// response.
    pub message: super::canonical::Canonical,
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
    /// Provides a description of the circumstances in which this response should be
    /// used (as opposed to one of the alternative responses).
    pub situation: super::markdown::Markdown,
}

/// Defines the characteristics of a message that can be shared between systems,
/// including the type of event that initiates the message, the content to be
/// transmitted and what response(s), if any, are permitted.
#[derive(Debug, Clone, PartialEq)]
pub struct MessageDefinitionFocus {
    /// The kind of resource that must be the focus for this message.
    pub code: super::code::Code,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Identifies the maximum number of resources of this type that must be pointed to
    /// by a message in order for it to be valid against this MessageDefinition.
    pub max: super::string::String,
    /// Identifies the minimum number of resources of this type that must be pointed to
    /// by a message in order for it to be valid against this MessageDefinition.
    pub min: super::unsigned_int::UnsignedInt,
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
    /// A profile that reflects constraints for the focal resource (and potentially for
    /// related resources).
    pub profile: super::canonical::Canonical,
}
