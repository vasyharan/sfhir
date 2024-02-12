/// A set of requirements - a list of features or behaviors of designed systems that
/// are necessary to achieve organizational or regulatory goals.
#[derive(Debug, Clone, PartialEq)]
pub struct Requirements {
    /// An actor these requirements are in regard to.
    pub actor: Vec<super::canonical::Canonical>,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the Requirements and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the Requirements.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the Requirements was published. The date
    /// must change when the business version changes and it must change if the status
    /// code changes. In addition, it should change when the substantive content of the
    /// Requirements changes.
    pub date: super::date_time::DateTime,
    /// Another set of Requirements that this set of Requirements builds on and updates.
    pub derived_from: Vec<super::canonical::Canonical>,
    /// A free text natural language description of the requirements.
    pub description: super::markdown::Markdown,
    /// A Boolean value to indicate that this Requirements is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this Requirements when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the Requirements is intended to be used.
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
    /// A natural language name identifying the Requirements. This name should be usable
    /// as an identifier for the module by machine processing applications such as code
    /// generation.
    pub name: super::string::String,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the Requirements.
    pub publisher: super::string::String,
    /// Explanation of why this Requirements is needed and why it has been designed as
    /// it has.
    pub purpose: super::markdown::Markdown,
    /// A reference to another artifact that created this set of requirements. This
    /// could be a Profile, etc., or external regulation, or business requirements
    /// expressed elsewhere.
    pub reference: Vec<super::url::Url>,
    /// This is a Requirements resource
    pub resource_type: String,
    /// The actual statement of requirement, in markdown format.
    pub statement: Vec<super::requirements::RequirementsStatement>,
    /// The status of this Requirements. Enables tracking the life-cycle of the content.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the Requirements.
    pub title: super::string::String,
    /// An absolute URI that is used to identify this Requirements when it is referenced
    /// in a specification, model, design or an instance; also called its canonical
    /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
    /// which an authoritative instance of this Requirements is (or will be) published.
    /// This URL can be the target of a canonical reference. It SHALL remain the same
    /// when the Requirements is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...)
    /// or may be references to specific programs (insurance plans, studies, ...) and
    /// may be used to assist with indexing and searching for appropriate Requirements
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the Requirements when
    /// it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the Requirements author and is not expected to be
    /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
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

/// A set of requirements - a list of features or behaviors of designed systems that
/// are necessary to achieve organizational or regulatory goals.
#[derive(Debug, Clone, PartialEq)]
pub struct RequirementsStatement {
    /// This boolean flag is set to true of the text of the requirement is conditional
    /// on something e.g. it includes lanauage like 'if x then y'. This conditionality
    /// flag is introduced for purposes of filtering and colour highlighting etc.
    pub conditionality: super::boolean::Boolean,
    /// A short human usable label for this statement.
    pub conformance: Vec<super::code::Code>,
    /// Another statement on one of the requirements that this requirement clarifies
    /// or restricts.
    pub derived_from: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Key that identifies this statement (unique within this resource).
    pub key: super::id::Id,
    /// A short human usable label for this statement.
    pub label: super::string::String,
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
    /// A larger requirement that this requirement helps to refine and enable.
    pub parent: super::string::String,
    /// A reference to another artifact that created this requirement. This could be
    /// a Profile, etc., or external regulation, or business requirements expressed
    /// elsewhere.
    pub reference: Vec<super::url::Url>,
    /// The actual requirement for human consumption.
    pub requirement: super::markdown::Markdown,
    /// A reference to another artifact that satisfies this requirement. This could be
    /// a Profile, extension, or an element in one of those, or a CapabilityStatement,
    /// OperationDefinition, SearchParameter, CodeSystem(/code), ValueSet, Libary etc.
    pub satisfied_by: Vec<super::url::Url>,
    /// Who asked for this statement to be a requirement. By default, it's assumed that
    /// the publisher knows who it is if it matters.
    pub source: Vec<super::reference::Reference>,
}
