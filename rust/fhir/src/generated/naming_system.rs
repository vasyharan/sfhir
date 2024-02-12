/// A curated namespace that issues unique symbols within that namespace for the
/// identification of concepts, people, devices, etc.  Represents a "System" used
/// within the Identifier and Coding data types.
#[derive(Debug, Clone, PartialEq)]
pub struct NamingSystem {
    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// An individiual or organization primarily involved in the creation and
    /// maintenance of the NamingSystem.
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
    /// A copyright statement relating to the naming system and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the naming system.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the naming system was last significantly
    /// changed. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the naming system changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the naming system from a consumer's
    /// perspective. Details about what the namespace identifies including scope,
    /// granularity, version labeling, etc.
    pub description: super::markdown::Markdown,
    /// An individual or organization primarily responsible for internal coherence of
    /// the NamingSystem.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the NamingSystem content was or is planned to be in
    /// active use.
    pub effective_period: super::period::Period,
    /// An individual or organization asserted by the publisher to be responsible for
    /// officially endorsing the NamingSystem for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A Boolean value to indicate that this naming system is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this naming system when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the naming system is intended to be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// Indicates the purpose for the naming system - what kinds of things does it make
    /// unique?
    pub kind: super::code::Code,
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
    /// A natural language name identifying the naming system. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub name: super::string::String,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the naming system.
    pub publisher: super::string::String,
    /// Explanation of why this naming system is needed and why it has been designed as
    /// it has.
    pub purpose: super::markdown::Markdown,
    /// Related artifacts such as additional documentation, justification, dependencies,
    /// bibliographic references, and predecessor and successor artifacts.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a NamingSystem resource
    pub resource_type: String,
    /// The name of the organization that is responsible for issuing identifiers or
    /// codes for this namespace and ensuring their non-collision.
    pub responsible: super::string::String,
    /// An individual or organization asserted by the publisher to be primarily
    /// responsible for review of some aspect of the NamingSystem.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// The status of this naming system. Enables tracking the life-cycle of the
    /// content.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the naming system.
    pub title: super::string::String,
    /// Descriptions related to the content of the NamingSystem. Topics provide a high-
    /// level categorization as well as keywords for the NamingSystem that can be useful
    /// for filtering and searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// Categorizes a naming system for easier search by grouping related naming
    /// systems.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// Indicates how the system may be identified when referenced in electronic
    /// exchange.
    pub unique_id: Vec<super::naming_system::NamingSystemUniqueId>,
    /// An absolute URI that is used to identify this naming system when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this naming system is (or will be)
    /// published. This URL can be the target of a canonical reference. It SHALL remain
    /// the same when the naming system is stored on different servers.
    pub url: super::uri::Uri,
    /// Provides guidance on the use of the namespace, including the handling of
    /// formatting characters, use of upper vs. lower case, etc.
    pub usage: super::string::String,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...)
    /// or may be references to specific programs (insurance plans, studies, ...) and
    /// may be used to assist with indexing and searching for appropriate naming system
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the naming system
    /// when it is referenced in a specification, model, design or instance. This is
    /// an arbitrary value managed by the naming system author and is not expected to
    /// be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which NamingSystem
    /// is more current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which NamingSystem
    /// is more current.
    pub version_algorithm_string: String,
}

/// A curated namespace that issues unique symbols within that namespace for the
/// identification of concepts, people, devices, etc.  Represents a "System" used
/// within the Identifier and Coding data types.
#[derive(Debug, Clone, PartialEq)]
pub struct NamingSystemUniqueId {
    /// Indicates whether this identifier ie endorsed by the official owner of the
    /// associated naming system.
    pub authoritative: super::boolean::Boolean,
    /// Notes about the past or intended usage of this identifier.
    pub comment: super::string::String,
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
    /// Identifies the period of time over which this identifier is considered
    /// appropriate to refer to the naming system.  Outside of this window, the
    /// identifier might be non-deterministic.
    pub period: super::period::Period,
    /// Indicates whether this identifier is the "preferred" identifier of this type.
    pub preferred: super::boolean::Boolean,
    /// Identifies the unique identifier scheme used for this particular identifier.
    pub r#type: super::code::Code,
    /// The string that should be sent over the wire to identify the code system or
    /// identifier system.
    pub value: super::string::String,
}
