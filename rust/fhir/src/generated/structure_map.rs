/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMap {
    /// Definition of a constant value used in the map rules.
    pub r#const: Vec<super::structure_map::StructureMapConst>,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the structure map and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the structure map.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the structure map was last significantly
    /// changed. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the structure map changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the structure map from a consumer's
    /// perspective.
    pub description: super::markdown::Markdown,
    /// A Boolean value to indicate that this structure map is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// Organizes the mapping into managable chunks for human review/ease of
    /// maintenance.
    pub group: Vec<super::structure_map::StructureMapGroup>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this structure map when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Other maps used by this map (canonical URLs).
    pub import: Vec<super::canonical::Canonical>,
    /// A legal or geographic region in which the structure map is intended to be used.
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
    /// A natural language name identifying the structure map. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub name: super::string::String,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the structure map.
    pub publisher: super::string::String,
    /// Explanation of why this structure map is needed and why it has been designed as
    /// it has.
    pub purpose: super::markdown::Markdown,
    /// This is a StructureMap resource
    pub resource_type: String,
    /// The status of this structure map. Enables tracking the life-cycle of the
    /// content.
    pub status: super::code::Code,
    /// A structure definition used by this map. The structure definition may describe
    /// instances that are converted, or the instances that are produced.
    pub structure: Vec<super::structure_map::StructureMapStructure>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the structure map.
    pub title: super::string::String,
    /// An absolute URI that is used to identify this structure map when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this structure map is (or will be)
    /// published. This URL can be the target of a canonical reference. It SHALL remain
    /// the same when the structure map is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...)
    /// or may be references to specific programs (insurance plans, studies, ...) and
    /// may be used to assist with indexing and searching for appropriate structure
    /// map instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the structure map
    /// when it is referenced in a specification, model, design or instance. This is
    /// an arbitrary value managed by the structure map author and is not expected to
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

/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapConst {
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
    /// Other maps used by this map (canonical URLs).
    pub name: super::id::Id,
    /// A FHIRPath expression that is the value of this variable.
    pub value: super::string::String,
}

/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapDependent {
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
    /// Name of a rule or group to apply.
    pub name: super::id::Id,
    /// Parameter to pass to the rule or group.
    pub parameter: Vec<super::structure_map::StructureMapParameter>,
}

/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapGroup {
    /// Additional supporting documentation that explains the purpose of the group and
    /// the types of mappings within it.
    pub documentation: super::string::String,
    /// Another group that this group adds rules to.
    pub extends: super::id::Id,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A name assigned to an instance of data. The instance must be provided when the
    /// mapping is invoked.
    pub input: Vec<super::structure_map::StructureMapInput>,
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
    /// A unique name for the group for the convenience of human readers.
    pub name: super::id::Id,
    /// Transform Rule from source to target.
    pub rule: Vec<super::structure_map::StructureMapRule>,
    /// If this is the default rule set to apply for the source type or this combination
    /// of types.
    pub type_mode: super::code::Code,
}

/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapInput {
    /// Documentation for this instance of data.
    pub documentation: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Mode for this instance of data.
    pub mode: super::code::Code,
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
    /// Name for this instance of data.
    pub name: super::id::Id,
    /// Type for this instance of data.
    pub r#type: super::string::String,
}

/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapParameter {
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
    /// Parameter value - variable or literal.
    pub value_boolean: bool,
    /// Parameter value - variable or literal.
    pub value_date: String,
    /// Parameter value - variable or literal.
    pub value_date_time: String,
    /// Parameter value - variable or literal.
    pub value_decimal: f64,
    /// Parameter value - variable or literal.
    pub value_id: String,
    /// Parameter value - variable or literal.
    pub value_integer: i64,
    /// Parameter value - variable or literal.
    pub value_string: String,
    /// Parameter value - variable or literal.
    pub value_time: String,
}

/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapRule {
    /// Which other rules to apply in the context of this rule.
    pub dependent: Vec<super::structure_map::StructureMapDependent>,
    /// Documentation for this instance of data.
    pub documentation: super::string::String,
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
    /// Name of the rule for internal references.
    pub name: super::id::Id,
    /// Rules contained in this rule.
    pub rule: Vec<super::structure_map::StructureMapRule>,
    /// Source inputs to the mapping.
    pub source: Vec<super::structure_map::StructureMapSource>,
    /// Content to create because of this mapping rule.
    pub target: Vec<super::structure_map::StructureMapTarget>,
}

/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapSource {
    /// FHIRPath expression  - must be true or the mapping engine throws an error
    /// instead of completing.
    pub check: super::string::String,
    /// FHIRPath expression  - must be true or the rule does not apply.
    pub condition: super::string::String,
    /// Type or variable this rule applies to.
    pub context: super::id::Id,
    /// A value to use if there is no existing value in the source object.
    pub default_value: super::string::String,
    /// Optional field for this source.
    pub element: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// How to handle the list mode for this element.
    pub list_mode: super::code::Code,
    /// A FHIRPath expression which specifies a message to put in the transform log when
    /// content matching the source rule is found.
    pub log_message: super::string::String,
    /// Specified maximum cardinality for the element - a number or a "*". This is
    /// optional; if present, it acts an implicit check on the input content (* just
    /// serves as documentation; it's the default value).
    pub max: super::string::String,
    /// Specified minimum cardinality for the element. This is optional; if present, it
    /// acts an implicit check on the input content.
    pub min: super::integer::Integer,
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
    /// Specified type for the element. This works as a condition on the mapping - use
    /// for polymorphic elements.
    pub r#type: super::string::String,
    /// Named context for field, if a field is specified.
    pub variable: super::id::Id,
}

/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapStructure {
    /// The name used for this type in the map.
    pub alias: super::string::String,
    /// Documentation that describes how the structure is used in the mapping.
    pub documentation: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// How the referenced structure is used in this mapping.
    pub mode: super::code::Code,
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
    /// The canonical reference to the structure.
    pub url: super::canonical::Canonical,
}

/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureMapTarget {
    /// Variable this rule applies to.
    pub context: super::string::String,
    /// Field to create in the context.
    pub element: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// If field is a list, how to manage the list.
    pub list_mode: Vec<super::code::Code>,
    /// Internal rule reference for shared list items.
    pub list_rule_id: super::id::Id,
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
    /// Parameters to the transform.
    pub parameter: Vec<super::structure_map::StructureMapParameter>,
    /// How the data is copied / created.
    pub transform: super::code::Code,
    /// Named context for field, if desired, and a field is specified.
    pub variable: super::id::Id,
}
