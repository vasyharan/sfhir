/// The Library resource is a general-purpose container for knowledge asset
/// definitions. It can be used to describe and expose existing knowledge assets
/// such as logic libraries and information model descriptions, as well as to
/// describe a collection of knowledge assets.
#[derive(Debug, Clone, PartialEq)]
pub struct Library {
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
    /// The content of the library as an Attachment. The content may be a reference
    /// to a url, or may be directly embedded as a base-64 string. Either way, the
    /// contentType of the attachment determines how to interpret the content.
    pub content: Vec<super::attachment::Attachment>,
    /// A copyright statement relating to the library and/or its contents. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// library.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// Describes a set of data that must be provided in order to be able to
    /// successfully perform the computations defined by the library.
    pub data_requirement: Vec<super::data_requirement::DataRequirement>,
    /// The date  (and optionally time) when the library was last significantly changed.
    /// The date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content
    /// of the library changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the library from a consumer's
    /// perspective.
    pub description: super::markdown::Markdown,
    /// An individual or organization primarily responsible for internal coherence of
    /// the content.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the library content was or is planned to be in active
    /// use.
    pub effective_period: super::period::Period,
    /// An individual or organization asserted by the publisher to be responsible for
    /// officially endorsing the content for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A Boolean value to indicate that this library is authored for testing purposes
    /// (or education/evaluation/marketing) and is not intended to be used for genuine
    /// usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this library when it is represented
    /// in other formats, or referenced in a specification, model, design or an
    /// instance. e.g. CMS or NQF identifiers for a measure artifact. Note that at least
    /// one identifier is required for non-experimental active artifacts.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the library is intended to be used.
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
    /// A natural language name identifying the library. This name should be usable as
    /// an identifier for the module by machine processing applications such as code
    /// generation.
    pub name: super::string::String,
    /// The parameter element defines parameters used by the library.
    pub parameter: Vec<super::parameter_definition::ParameterDefinition>,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the library.
    pub publisher: super::string::String,
    /// Explanation of why this library is needed and why it has been designed as it
    /// has.
    pub purpose: super::markdown::Markdown,
    /// Related artifacts such as additional documentation, justification, or
    /// bibliographic references.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a Library resource
    pub resource_type: String,
    /// An individual or organization asserted by the publisher to be primarily
    /// responsible for review of some aspect of the content.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// The status of this library. Enables tracking the life-cycle of the content.
    pub status: super::code::Code,
    /// A code or group definition that describes the intended subject of the contents
    /// of the library.
    pub subject_codeable_concept: super::codeable_concept::CodeableConcept,
    /// A code or group definition that describes the intended subject of the contents
    /// of the library.
    pub subject_reference: super::reference::Reference,
    /// An explanatory or alternate title for the library giving additional information
    /// about its content.
    pub subtitle: super::string::String,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the library.
    pub title: super::string::String,
    /// Descriptive topics related to the content of the library. Topics provide a
    /// high-level categorization of the library that can be useful for filtering and
    /// searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// Identifies the type of library such as a Logic Library, Model Definition, Asset
    /// Collection, or Module Definition.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// An absolute URI that is used to identify this library when it is referenced
    /// in a specification, model, design or an instance; also called its canonical
    /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
    /// which an authoritative instance of this library is (or will be) published. This
    /// URL can be the target of a canonical reference. It SHALL remain the same when
    /// the library is stored on different servers.
    pub url: super::uri::Uri,
    /// A detailed description of how the library is used from a clinical perspective.
    pub usage: super::markdown::Markdown,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate library instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the library when it is
    /// referenced in a specification, model, design or instance. This is an arbitrary
    /// value managed by the library author and is not expected to be globally unique.
    /// For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is
    /// not available. There is also no expectation that versions can be placed in a
    /// lexicographical sequence. To provide a version consistent with the Decision
    /// Support Service specification, use the format Major.Minor.Revision (e.g.
    /// 1.0.0). For more information on versioning knowledge assets, refer to the
    /// Decision Support Service specification. Note that a version is required for non-
    /// experimental active artifacts.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}
