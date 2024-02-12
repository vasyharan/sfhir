/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).
#[derive(Debug, Clone, PartialEq)]
pub struct OperationDefinition {
    /// Whether the operation affects state. Side effects such as producing audit trail
    /// entries do not count as 'affecting  state'.
    pub affects_state: super::boolean::Boolean,
    /// Indicates that this operation definition is a constraining profile on the base.
    pub base: super::canonical::Canonical,
    /// The label that is recommended to be used in the URL for this operation. In some
    /// cases, servers may need to use a different CapabilityStatement operation.name
    /// to differentiate between multiple SearchParameters that happen to have the same
    /// code.
    pub code: super::code::Code,
    /// Additional information about how to use this operation or named query.
    pub comment: super::markdown::Markdown,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the operation definition and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the operation definition.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the operation definition was last
    /// significantly changed. The date must change when the business version changes
    /// and it must change if the status code changes. In addition, it should change
    /// when the substantive content of the operation definition changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the operation definition from a
    /// consumer's perspective.
    pub description: super::markdown::Markdown,
    /// A Boolean value to indicate that this operation definition is authored for
    /// testing purposes (or education/evaluation/marketing) and is not intended for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this implementation guide when it
    /// is represented in other formats, or referenced in a specification, model, design
    /// or an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Additional validation information for the in parameters - a single profile
    /// that covers all the parameters. The profile is a constraint on the parameters
    /// resource as a whole.
    pub input_profile: super::canonical::Canonical,
    /// Indicates whether this operation can be invoked on a particular instance of one
    /// of the given types.
    pub instance: super::boolean::Boolean,
    /// A legal or geographic region in which the operation definition is intended to
    /// be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// Whether this is an operation or a named query.
    pub kind: super::code::Code,
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
    /// A natural language name identifying the operation definition. This name should
    /// be usable as an identifier for the module by machine processing applications
    /// such as code generation.
    pub name: super::string::String,
    /// Additional validation information for the out parameters - a single profile
    /// that covers all the parameters. The profile is a constraint on the parameters
    /// resource.
    pub output_profile: super::canonical::Canonical,
    /// Defines an appropriate combination of parameters to use when invoking this
    /// operation, to help code generators when generating overloaded parameter sets for
    /// this operation.
    pub overload: Vec<super::operation_definition::OperationDefinitionOverload>,
    /// The parameters for the operation/query.
    pub parameter: Vec<super::operation_definition::OperationDefinitionParameter>,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the operation definition.
    pub publisher: super::string::String,
    /// Explanation of why this operation definition is needed and why it has been
    /// designed as it has.
    pub purpose: super::markdown::Markdown,
    /// The types on which this operation can be executed.
    pub resource: Vec<super::code::Code>,
    /// This is a OperationDefinition resource
    pub resource_type: String,
    /// The current state of this operation definition.
    pub status: super::code::Code,
    /// Indicates whether this operation or named query can be invoked at the system
    /// level (e.g. without needing to choose a resource type for the context).
    pub system: super::boolean::Boolean,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the operation definition.
    pub title: super::string::String,
    /// Indicates whether this operation or named query can be invoked at the resource
    /// type level for any given resource type level (e.g. without needing to choose a
    /// specific resource id for the context).
    pub r#type: super::boolean::Boolean,
    /// An absolute URI that is used to identify this operation definition when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this operation definition is
    /// (or will be) published. This URL can be the target of a canonical reference.
    /// It SHALL remain the same when the operation definition is stored on different
    /// servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...)
    /// or may be references to specific programs (insurance plans, studies, ...) and
    /// may be used to assist with indexing and searching for appropriate operation
    /// definition.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the operation definition
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the operation definition author and is not expected
    /// to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
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

/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).
#[derive(Debug, Clone, PartialEq)]
pub struct OperationDefinitionBinding {
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
    /// Indicates the degree of conformance expectations associated with this binding
    /// - that is, the degree to which the provided value set must be adhered to in
    /// the instances.
    pub strength: super::code::Code,
    /// Points to the value set or external definition (e.g. implicit value set) that
    /// identifies the set of codes to be used.
    pub value_set: super::canonical::Canonical,
}

/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).
#[derive(Debug, Clone, PartialEq)]
pub struct OperationDefinitionOverload {
    /// Comments to go on overload.
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
    /// Name of parameter to include in overload.
    pub parameter_name: Vec<super::string::String>,
}

/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).
#[derive(Debug, Clone, PartialEq)]
pub struct OperationDefinitionParameter {
    /// Support for polymorphic types. If the parameter type is abstract, this element
    /// lists allowed sub-types for the parameter.
    pub allowed_type: Vec<super::code::Code>,
    /// Binds to a value set if this parameter is coded (code, Coding, CodeableConcept).
    pub binding: super::operation_definition::OperationDefinitionBinding,
    /// Describes the meaning or use of this parameter.
    pub documentation: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The maximum number of times this element is permitted to appear in the request
    /// or response.
    pub max: super::string::String,
    /// The minimum number of times this parameter SHALL appear in the request or
    /// response.
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
    /// The name of used to identify the parameter.
    pub name: super::code::Code,
    /// The parts of a nested Parameter.
    pub part: Vec<super::operation_definition::OperationDefinitionParameter>,
    /// Identifies other resource parameters within the operation invocation that are
    /// expected to resolve to this resource.
    pub referenced_from: Vec<super::operation_definition::OperationDefinitionReferencedFrom>,
    /// If present, indicates that the parameter applies when the operation is being
    /// invoked at the specified level.
    pub scope: Vec<super::code::Code>,
    /// How the parameter is understood if/when it used as search parameter. This is
    /// only used if the parameter is a string.
    pub search_type: super::code::Code,
    /// Used when the type is "Reference" or "canonical", and identifies a profile
    /// structure or implementation Guide that applies to the target of the reference
    /// this parameter refers to. If any profiles are specified, then the content
    /// must conform to at least one of them. The URL can be a local reference - to a
    /// contained StructureDefinition, or a reference to another StructureDefinition
    /// or Implementation Guide by a canonical URL. When an implementation guide is
    /// specified, the target resource SHALL conform to at least one profile defined in
    /// the implementation guide.
    pub target_profile: Vec<super::canonical::Canonical>,
    /// The type for this parameter.
    pub r#type: super::code::Code,
    /// Whether this is an input or an output parameter.
    pub r#use: super::code::Code,
}

/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).
#[derive(Debug, Clone, PartialEq)]
pub struct OperationDefinitionReferencedFrom {
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
    /// The name of the parameter or dot-separated path of parameter names pointing to
    /// the resource parameter that is expected to contain a reference to this resource.
    pub source: super::string::String,
    /// The id of the element in the referencing resource that is expected to resolve to
    /// this resource.
    pub source_id: super::string::String,
}
