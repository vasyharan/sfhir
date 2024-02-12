/// A definition of a FHIR structure. This resource is used to describe the
/// underlying resources, data types defined in FHIR, and also for describing
/// extensions and constraints on resources and data types.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureDefinition {
    /// Whether structure this definition describes is abstract or not  - that is,
    /// whether the structure is not intended to be instantiated. For Resources and Data
    /// types, abstract types will never be exchanged  between systems.
    pub r#abstract: super::boolean::Boolean,
    /// An absolute URI that is the base structure from which this type is derived,
    /// either by specialization or constraint.
    pub base_definition: super::canonical::Canonical,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Identifies the types of resource or data type elements to which the extension
    /// can be applied. For more guidance on using the 'context' element, see the
    /// [defining extensions page](defining-extensions.html#context).
    pub context: Vec<super::structure_definition::StructureDefinitionContext>,
    /// A set of rules as FHIRPath Invariants about when the extension can be used (e.g.
    /// co-occurrence variants for the extension). All the rules must be true.
    pub context_invariant: Vec<super::string::String>,
    /// A copyright statement relating to the structure definition and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the structure definition.  The short copyright declaration (e.g. (c) '2015+
    /// xyz organization' should be sent in the copyrightLabel element.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the structure definition was last
    /// significantly changed. The date must change when the business version changes
    /// and it must change if the status code changes. In addition, it should change
    /// when the substantive content of the structure definition changes.
    pub date: super::date_time::DateTime,
    /// How the type relates to the baseDefinition.
    pub derivation: super::code::Code,
    /// A free text natural language description of the structure definition from a
    /// consumer's perspective.
    pub description: super::markdown::Markdown,
    /// A differential view is expressed relative to the base StructureDefinition - a
    /// statement of differences that it applies.
    pub differential: super::structure_definition::StructureDefinitionDifferential,
    /// A Boolean value to indicate that this structure definition is authored for
    /// testing purposes (or education/evaluation/marketing) and is not intended to be
    /// used for genuine usage.
    pub experimental: super::boolean::Boolean,
    /// The version of the FHIR specification on which this StructureDefinition is based
    /// - this is the formal version of the specification, without the revision number,
    /// e.g. [publication].[major].[minor], which is 4.6.0. for this version.
    pub fhir_version: super::code::Code,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this structure definition when it
    /// is represented in other formats, or referenced in a specification, model, design
    /// or an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the structure definition is intended to
    /// be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// (DEPRECATED) A set of key words or terms from external terminologies that may be
    /// used to assist with indexing and searching of templates nby describing the use
    /// of this structure definition, or the content it describes.
    pub keyword: Vec<super::coding::Coding>,
    /// Defines the kind of structure that this definition is describing.
    pub kind: super::code::Code,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// An external specification that the content is mapped to.
    pub mapping: Vec<super::structure_definition::StructureDefinitionMapping>,
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
    /// A natural language name identifying the structure definition. This name should
    /// be usable as an identifier for the module by machine processing applications
    /// such as code generation.
    pub name: super::string::String,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the structure definition.
    pub publisher: super::string::String,
    /// Explanation of why this structure definition is needed and why it has been
    /// designed as it has.
    pub purpose: super::markdown::Markdown,
    /// This is a StructureDefinition resource
    pub resource_type: String,
    /// A snapshot view is expressed in a standalone form that can be used and
    /// interpreted without considering the base StructureDefinition.
    pub snapshot: super::structure_definition::StructureDefinitionSnapshot,
    /// The status of this structure definition. Enables tracking the life-cycle of
    /// the content.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the structure definition.
    pub title: super::string::String,
    /// The type this structure describes. If the derivation kind is 'specialization'
    /// then this is the master definition for a type, and there is always one of these
    /// (a data type, an extension, a resource, including abstract ones). Otherwise
    /// the structure definition is a constraint on the stated type (and in this case,
    /// the type cannot be an abstract type).  References are URLs that are relative to
    /// http://hl7.org/fhir/StructureDefinition e.g. "string" is a reference to http://
    /// hl7.org/fhir/StructureDefinition/string. Absolute URLs are only allowed in
    /// logical models, where they are required.
    pub r#type: super::uri::Uri,
    /// An absolute URI that is used to identify this structure definition when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this structure definition is
    /// (or will be) published. This URL can be the target of a canonical reference.
    /// It SHALL remain the same when the structure definition is stored on different
    /// servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...)
    /// or may be references to specific programs (insurance plans, studies, ...) and
    /// may be used to assist with indexing and searching for appropriate structure
    /// definition instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the structure definition
    /// when it is referenced in a specification, model, design or instance. This
    /// is an arbitrary value managed by the structure definition author and is not
    /// expected to be globally unique. There is no expectation that versions can be
    /// placed in a lexicographical sequence, so authors are encouraged to populate the
    /// StructureDefinition.versionAlgorithm[x] element to enable comparisons. If there
    /// is no managed version available, authors can consider using ISO date/time syntax
    /// (e.g., '2023-01-01').
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// A definition of a FHIR structure. This resource is used to describe the
/// underlying resources, data types defined in FHIR, and also for describing
/// extensions and constraints on resources and data types.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureDefinitionContext {
    /// An expression that defines where an extension can be used in resources.
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
    /// Defines how to interpret the expression that defines what the context of the
    /// extension is.
    pub r#type: super::code::Code,
}

/// A definition of a FHIR structure. This resource is used to describe the
/// underlying resources, data types defined in FHIR, and also for describing
/// extensions and constraints on resources and data types.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureDefinitionDifferential {
    /// Captures constraints on each element within the resource.
    pub element: Vec<super::element_definition::ElementDefinition>,
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

/// A definition of a FHIR structure. This resource is used to describe the
/// underlying resources, data types defined in FHIR, and also for describing
/// extensions and constraints on resources and data types.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureDefinitionMapping {
    /// Comments about this mapping, including version notes, issues, scope limitations,
    /// and other important notes for usage.
    pub comment: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// An Internal id that is used to identify this mapping set when specific mappings
    /// are made.
    pub identity: super::id::Id,
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
    /// A name for the specification that is being mapped to.
    pub name: super::string::String,
    /// An absolute URI that identifies the specification that this mapping is expressed
    /// to.
    pub uri: super::uri::Uri,
}

/// A definition of a FHIR structure. This resource is used to describe the
/// underlying resources, data types defined in FHIR, and also for describing
/// extensions and constraints on resources and data types.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureDefinitionSnapshot {
    /// Captures constraints on each element within the resource.
    pub element: Vec<super::element_definition::ElementDefinition>,
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
