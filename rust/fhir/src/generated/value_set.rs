/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug, Clone, PartialEq)]
pub struct ValueSet {
    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// An individiual or organization primarily involved in the creation and
    /// maintenance of the ValueSet.
    pub author: Vec<super::contact_detail::ContactDetail>,
    /// A set of criteria that define the contents of the value set by including or
    /// excluding codes selected from the specified code system(s) that the value set
    /// draws from. This is also known as the Content Logical Definition (CLD).
    pub compose: super::value_set::ValueSetCompose,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the value set and/or its contents. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// value set.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date (and optionally time) when the value set metadata or content logical
    /// definition (.compose) was created or revised.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the value set from a consumer's
    /// perspective. The textual description specifies the span of meanings for concepts
    /// to be included within the Value Set Expansion, and also may specify the intended
    /// use and limitations of the Value Set.
    pub description: super::markdown::Markdown,
    /// An individual or organization primarily responsible for internal coherence of
    /// the ValueSet.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the ValueSet content was or is planned to be in active
    /// use.
    pub effective_period: super::period::Period,
    /// An individual or organization asserted by the publisher to be responsible for
    /// officially endorsing the ValueSet for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A value set can also be "expanded", where the value set is turned into a simple
    /// collection of enumerated codes. This element holds the expansion, if it has
    /// been performed.
    pub expansion: super::value_set::ValueSetExpansion,
    /// A Boolean value to indicate that this value set is authored for testing purposes
    /// (or education/evaluation/marketing) and is not intended to be used for genuine
    /// usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this value set when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// If this is set to 'true', then no new versions of the content logical definition
    /// can be created.  Note: Other metadata might still change.
    pub immutable: super::boolean::Boolean,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the value set is intended to be used.
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
    /// A natural language name identifying the value set. This name should be usable
    /// as an identifier for the module by machine processing applications such as code
    /// generation.
    pub name: super::string::String,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the value set.
    pub publisher: super::string::String,
    /// Explanation of why this value set is needed and why it has been designed as
    /// it has.
    pub purpose: super::markdown::Markdown,
    /// Related artifacts such as additional documentation, justification, dependencies,
    /// bibliographic references, and predecessor and successor artifacts.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a ValueSet resource
    pub resource_type: String,
    /// An individual or organization asserted by the publisher to be primarily
    /// responsible for review of some aspect of the ValueSet.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// Description of the semantic space the Value Set Expansion is intended to cover
    /// and should further clarify the text in ValueSet.description.
    pub scope: super::value_set::ValueSetScope,
    /// The status of this value set. Enables tracking the life-cycle of the
    /// content. The status of the value set applies to the value set definition
    /// (ValueSet.compose) and the associated ValueSet metadata. Expansions do not have
    /// a state.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the value set.
    pub title: super::string::String,
    /// Descriptions related to the content of the ValueSet. Topics provide a high-
    /// level categorization as well as keywords for the ValueSet that can be useful for
    /// filtering and searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// An absolute URI that is used to identify this value set when it is referenced
    /// in a specification, model, design or an instance; also called its canonical
    /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
    /// which an authoritative instance of this value set is (or will be) published.
    /// This URL can be the target of a canonical reference. It SHALL remain the same
    /// when the value set is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...)
    /// or may be references to specific programs (insurance plans, studies, ...) and
    /// may be used to assist with indexing and searching for appropriate value set
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the value set when it is
    /// referenced in a specification, model, design or instance. This is an arbitrary
    /// value managed by the value set author and is not expected to be globally unique.
    /// For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is
    /// not available. There is also no expectation that versions can be placed in a
    /// lexicographical sequence.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which ValueSet is
    /// more current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which ValueSet is
    /// more current.
    pub version_algorithm_string: String,
}

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug, Clone, PartialEq)]
pub struct ValueSetCompose {
    /// Exclude one or more codes from the value set based on code system filters and/or
    /// other value sets.
    pub exclude: Vec<super::value_set::ValueSetInclude>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Whether inactive codes - codes that are not approved for current use - are in
    /// the value set. If inactive = true, inactive codes are to be included in the
    /// expansion, if inactive = false, the inactive codes will not be included in
    /// the expansion. If absent, the behavior is determined by the implementation, or
    /// by the applicable $expand parameters (but generally, inactive codes would be
    /// expected to be included).
    pub inactive: super::boolean::Boolean,
    /// Include one or more codes from a code system or other value set(s).
    pub include: Vec<super::value_set::ValueSetInclude>,
    /// The Locked Date is  the effective date that is used to determine the version of
    /// all referenced Code Systems and Value Set Definitions included in the compose
    /// that are not already tied to a specific version.
    pub locked_date: super::date::Date,
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
    /// A property to return in the expansion, if the client doesn't ask for any
    /// particular properties. May be either a code from the code system definition
    /// (convenient) or a the formal URI that refers to the property. The special value
    /// '*' means all properties known to the server.
    pub property: Vec<super::string::String>,
}

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug, Clone, PartialEq)]
pub struct ValueSetConcept {
    /// Specifies a code for the concept to be included or excluded.
    pub code: super::code::Code,
    /// Additional representations for this concept when used in this value set - other
    /// languages, aliases, specialized purposes, used for particular purposes, etc.
    pub designation: Vec<super::value_set::ValueSetDesignation>,
    /// The text to display to the user for this concept in the context of this
    /// valueset. If no display is provided, then applications using the value set use
    /// the display specified for the code by the system.
    pub display: super::string::String,
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

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug, Clone, PartialEq)]
pub struct ValueSetContains {
    /// If true, this entry is included in the expansion for navigational purposes, and
    /// the user cannot select the code directly as a proper value.
    pub r#abstract: super::boolean::Boolean,
    /// The code for this item in the expansion hierarchy. If this code is missing the
    /// entry in the hierarchy is a place holder (abstract) and does not represent a
    /// valid code in the value set.
    pub code: super::code::Code,
    /// Other codes and entries contained under this entry in the hierarchy.
    pub contains: Vec<super::value_set::ValueSetContains>,
    /// Additional representations for this item - other languages, aliases, specialized
    /// purposes, used for particular purposes, etc. These are relevant when the
    /// conditions of the expansion do not fix to a single correct representation.
    pub designation: Vec<super::value_set::ValueSetDesignation>,
    /// The recommended display for this item in the expansion.
    pub display: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// If the concept is inactive in the code system that defines it. Inactive codes
    /// are those that are no longer to be used, but are maintained by the code system
    /// for understanding legacy data. It might not be known or specified whether a
    /// concept is inactive (and it may depend on the context of use).
    pub inactive: super::boolean::Boolean,
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
    /// A property value for this concept.
    pub property: Vec<super::value_set::ValueSetProperty1>,
    /// An absolute URI which is the code system in which the code for this item in the
    /// expansion is defined.
    pub system: super::uri::Uri,
    /// The version of the code system from this code was taken. Note that a well-
    /// maintained code system does not need the version reported, because the meaning
    /// of codes is consistent across versions. However this cannot consistently be
    /// assured, and when the meaning is not guaranteed to be consistent, the version
    /// SHOULD be exchanged.
    pub version: super::string::String,
}

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug, Clone, PartialEq)]
pub struct ValueSetDesignation {
    /// Additional codes that detail how this designation would be used, if there is
    /// more than one use.
    pub additional_use: Vec<super::coding::Coding>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The language this designation is defined for.
    pub language: super::code::Code,
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
    /// A code that represents types of uses of designations.
    pub r#use: super::coding::Coding,
    /// The text value for this designation.
    pub value: super::string::String,
}

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug, Clone, PartialEq)]
pub struct ValueSetExpansion {
    /// The codes that are contained in the value set expansion.
    pub contains: Vec<super::value_set::ValueSetContains>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// An identifier that uniquely identifies this expansion of the valueset, based on
    /// a unique combination of the provided parameters, the system default parameters,
    /// and the underlying system code system versions etc. Systems may re-use the same
    /// identifier as long as those factors remain the same, and the expansion is the
    /// same, but are not required to do so. This is a business identifier.
    pub identifier: super::uri::Uri,
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
    /// As per paging Search results, the next URLs are opaque to the client, have no
    /// dictated structure, and only the server understands them.
    pub next: super::uri::Uri,
    /// If paging is being used, the offset at which this resource starts.  I.e. this
    /// resource is a partial view into the expansion. If paging is not being used, this
    /// element SHALL NOT be present.
    pub offset: super::integer::Integer,
    /// A parameter that controlled the expansion process. These parameters may be used
    /// by users of expanded value sets to check whether the expansion is suitable for a
    /// particular purpose, or to pick the correct expansion.
    pub parameter: Vec<super::value_set::ValueSetParameter>,
    /// A property defines an additional slot through which additional information can
    /// be provided about a concept.
    pub property: Vec<super::value_set::ValueSetProperty>,
    /// The time at which the expansion was produced by the expanding system.
    pub timestamp: super::date_time::DateTime,
    /// The total number of concepts in the expansion. If the number of concept nodes
    /// in this resource is less than the stated number, then the server can return more
    /// using the offset parameter.
    pub total: super::integer::Integer,
}

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug, Clone, PartialEq)]
pub struct ValueSetFilter {
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
    /// The kind of operation to perform as a part of the filter criteria.
    pub op: super::code::Code,
    /// A code that identifies a property or a filter defined in the code system.
    pub property: super::code::Code,
    /// The match value may be either a code defined by the system, or a string value,
    /// which is a regex match on the literal string of the property value  (if the
    /// filter represents a property defined in CodeSystem) or of the system filter
    /// value (if the filter represents a filter defined in CodeSystem) when the
    /// operation is 'regex', or one of the values (true and false), when the operation
    /// is 'exists'.
    pub value: super::string::String,
}

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug, Clone, PartialEq)]
pub struct ValueSetInclude {
    /// Specifies a concept to be included or excluded.
    pub concept: Vec<super::value_set::ValueSetConcept>,
    /// A copyright statement for the specific code system asserted by the
    /// containing ValueSet.compose.include element's system value (if the associated
    /// ValueSet.compose.include.version element is not present); or the code system and
    /// version combination (if the associated ValueSet.compose.include.version element
    /// is present).
    pub copyright: super::string::String,
    /// Select concepts by specifying a matching criterion based on the properties
    /// (including relationships) defined by the system, or on filters defined by the
    /// system. If multiple filters are specified within the include, they SHALL all
    /// be true.
    pub filter: Vec<super::value_set::ValueSetFilter>,
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
    /// An absolute URI which is the code system from which the selected codes come
    /// from.
    pub system: super::uri::Uri,
    /// Selects the concepts found in this value set (based on its value set
    /// definition). This is an absolute URI that is a reference to ValueSet.url.  If
    /// multiple value sets are specified this includes the intersection of the contents
    /// of all of the referenced value sets.
    pub value_set: Vec<super::canonical::Canonical>,
    /// The version of the code system that the codes are selected from, or the special
    /// version '*' for all versions.
    pub version: super::string::String,
}

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug, Clone, PartialEq)]
pub struct ValueSetParameter {
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
    /// Name of the input parameter to the $expand operation; may be a server-assigned
    /// name for additional default or other server-supplied parameters used to control
    /// the expansion process.
    pub name: super::string::String,
    /// The value of the parameter.
    pub value_boolean: bool,
    /// The value of the parameter.
    pub value_code: String,
    /// The value of the parameter.
    pub value_date_time: String,
    /// The value of the parameter.
    pub value_decimal: f64,
    /// The value of the parameter.
    pub value_integer: i64,
    /// The value of the parameter.
    pub value_string: String,
    /// The value of the parameter.
    pub value_uri: String,
}

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug, Clone, PartialEq)]
pub struct ValueSetProperty {
    /// A code that is used to identify the property. The code is used in
    /// ValueSet.expansion.contains.property.code.
    pub code: super::code::Code,
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
    /// Reference to the formal meaning of the property. One possible source of meaning
    /// is the [Concept Properties](codesystem-concept-properties.html) code system.
    pub uri: super::uri::Uri,
}

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug, Clone, PartialEq)]
pub struct ValueSetProperty1 {
    /// A code that is a reference to ValueSet.expansion.property.code.
    pub code: super::code::Code,
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
    /// A subproperty value for this concept.
    pub sub_property: Vec<super::value_set::ValueSetSubProperty>,
    /// The value of this property.
    pub value_boolean: bool,
    /// The value of this property.
    pub value_code: String,
    /// The value of this property.
    pub value_coding: super::coding::Coding,
    /// The value of this property.
    pub value_date_time: String,
    /// The value of this property.
    pub value_decimal: f64,
    /// The value of this property.
    pub value_integer: i64,
    /// The value of this property.
    pub value_string: String,
}

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug, Clone, PartialEq)]
pub struct ValueSetScope {
    /// Criteria describing which concepts or codes should be excluded and why.
    pub exclusion_criteria: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Criteria describing which concepts or codes should be included and why.
    pub inclusion_criteria: super::string::String,
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

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [CodeSystem](codesystem.html) definitions and their use in [coded elements]
/// (terminologies.html).
#[derive(Debug, Clone, PartialEq)]
pub struct ValueSetSubProperty {
    /// A code that is a reference to ValueSet.expansion.property.code.
    pub code: super::code::Code,
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
    /// The value of this subproperty.
    pub value_boolean: bool,
    /// The value of this subproperty.
    pub value_code: String,
    /// The value of this subproperty.
    pub value_coding: super::coding::Coding,
    /// The value of this subproperty.
    pub value_date_time: String,
    /// The value of this subproperty.
    pub value_decimal: f64,
    /// The value of this subproperty.
    pub value_integer: i64,
    /// The value of this subproperty.
    pub value_string: String,
}
