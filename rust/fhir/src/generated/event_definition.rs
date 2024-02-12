/// The EventDefinition resource provides a reusable description of when a
/// particular event can occur.
#[derive(Debug, Clone, PartialEq)]
pub struct EventDefinition {
    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// An individiual or organization primarily involved in the creation and
    /// maintenance of the content.
    pub author: Vec<super::contact_detail::ContactDetail>,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the event definition and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the event definition.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the event definition was last significantly
    /// changed. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the event definition changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the event definition from a
    /// consumer's perspective.
    pub description: super::markdown::Markdown,
    /// An individual or organization primarily responsible for internal coherence of
    /// the content.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the event definition content was or is planned to be in
    /// active use.
    pub effective_period: super::period::Period,
    /// An individual or organization asserted by the publisher to be responsible for
    /// officially endorsing the content for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A Boolean value to indicate that this event definition is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this event definition when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the event definition is intended to be
    /// used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The date on which the resource content was last reviewed. Review happens
    /// periodically after approval but does not change the original approval date.
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
    /// A natural language name identifying the event definition. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub name: super::string::String,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the event definition.
    pub publisher: super::string::String,
    /// Explanation of why this event definition is needed and why it has been designed
    /// as it has.
    pub purpose: super::markdown::Markdown,
    /// Related resources such as additional documentation, justification, or
    /// bibliographic references.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a EventDefinition resource
    pub resource_type: String,
    /// An individual or organization asserted by the publisher to be primarily
    /// responsible for review of some aspect of the content.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// The status of this event definition. Enables tracking the life-cycle of the
    /// content.
    pub status: super::code::Code,
    /// A code or group definition that describes the intended subject of the event
    /// definition.
    pub subject_codeable_concept: super::codeable_concept::CodeableConcept,
    /// A code or group definition that describes the intended subject of the event
    /// definition.
    pub subject_reference: super::reference::Reference,
    /// An explanatory or alternate title for the event definition giving additional
    /// information about its content.
    pub subtitle: super::string::String,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the event definition.
    pub title: super::string::String,
    /// Descriptive topics related to the module. Topics provide a high-level
    /// categorization of the module that can be useful for filtering and searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// The trigger element defines when the event occurs. If more than one trigger
    /// condition is specified, the event fires whenever any one of the trigger
    /// conditions is met.
    pub trigger: Vec<super::trigger_definition::TriggerDefinition>,
    /// An absolute URI that is used to identify this event definition when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this event definition is (or will
    /// be) published. This URL can be the target of a canonical reference. It SHALL
    /// remain the same when the event definition is stored on different servers.
    pub url: super::uri::Uri,
    /// A detailed description of how the event definition is used from a clinical
    /// perspective.
    pub usage: super::markdown::Markdown,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate event definition
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the event definition
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the event definition author and is not expected to
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
