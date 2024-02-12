/// A formal computable definition of a graph of resources - that is, a coherent
/// set of resources that form a graph by following references. The Graph Definition
/// resource defines a set and makes rules about the set.
#[derive(Debug, Clone, PartialEq)]
pub struct GraphDefinition {
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the graph definition and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the graph definition.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the graph definition was last significantly
    /// changed. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the graph definition changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the graph definition from a
    /// consumer's perspective.
    pub description: super::markdown::Markdown,
    /// A Boolean value to indicate that this graph definition is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this GraphDefinition when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the graph definition is intended to be
    /// used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Links this graph makes rules about.
    pub link: Vec<super::graph_definition::GraphDefinitionLink>,
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
    /// A natural language name identifying the graph definition. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub name: super::string::String,
    /// Potential target for the link.
    pub node: Vec<super::graph_definition::GraphDefinitionNode>,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the graph definition.
    pub publisher: super::string::String,
    /// Explanation of why this graph definition is needed and why it has been designed
    /// as it has.
    pub purpose: super::markdown::Markdown,
    /// This is a GraphDefinition resource
    pub resource_type: String,
    /// The Node at which instances of this graph start. If there is no nominated start,
    /// the graph can start at any of the nodes.
    pub start: super::id::Id,
    /// The status of this graph definition. Enables tracking the life-cycle of the
    /// content.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the capability statement.
    pub title: super::string::String,
    /// An absolute URI that is used to identify this graph definition when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this graph definition is (or will
    /// be) published. This URL can be the target of a canonical reference. It SHALL
    /// remain the same when the graph definition is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate graph definition
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the graph definition
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the graph definition author and is not expected to
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

/// A formal computable definition of a graph of resources - that is, a coherent
/// set of resources that form a graph by following references. The Graph Definition
/// resource defines a set and makes rules about the set.
#[derive(Debug, Clone, PartialEq)]
pub struct GraphDefinitionCompartment {
    /// Identifies the compartment.
    pub code: super::code::Code,
    /// Documentation for FHIRPath expression.
    pub description: super::string::String,
    /// Custom rule, as a FHIRPath expression.
    pub expression: super::string::String,
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
    /// identical | matching | different | no-rule | custom.
    pub rule: super::code::Code,
    /// Defines how the compartment rule is used - whether it it is used to test
    /// whether resources are subject to the rule, or whether it is a rule that must
    /// be followed.
    pub r#use: super::code::Code,
}

/// A formal computable definition of a graph of resources - that is, a coherent
/// set of resources that form a graph by following references. The Graph Definition
/// resource defines a set and makes rules about the set.
#[derive(Debug, Clone, PartialEq)]
pub struct GraphDefinitionLink {
    /// Compartment Consistency Rules.
    pub compartment: Vec<super::graph_definition::GraphDefinitionCompartment>,
    /// Information about why this link is of interest in this graph definition.
    pub description: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Maximum occurrences for this link.
    pub max: super::string::String,
    /// Minimum occurrences for this link.
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
    /// A set of parameters to look up.
    pub params: super::string::String,
    /// A FHIRPath expression that identifies one of FHIR References to other resources.
    pub path: super::string::String,
    /// Which slice (if profiled).
    pub slice_name: super::string::String,
    /// The source node for this link.
    pub source_id: super::id::Id,
    /// The target node for this link.
    pub target_id: super::id::Id,
}

/// A formal computable definition of a graph of resources - that is, a coherent
/// set of resources that form a graph by following references. The Graph Definition
/// resource defines a set and makes rules about the set.
#[derive(Debug, Clone, PartialEq)]
pub struct GraphDefinitionNode {
    /// Information about why this node is of interest in this graph definition.
    pub description: super::string::String,
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
    /// Internal ID of node - target for link references.
    pub node_id: super::id::Id,
    /// Profile for the target resource.
    pub profile: super::canonical::Canonical,
    /// Type of resource this link refers to.
    pub r#type: super::code::Code,
}
