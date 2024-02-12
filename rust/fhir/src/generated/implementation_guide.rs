/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuide {
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the implementation guide and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the implementation guide.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the implementation guide was last
    /// significantly changed. The date must change when the business version changes
    /// and it must change if the status code changes. In addition, it should change
    /// when the substantive content of the implementation guide changes.
    pub date: super::date_time::DateTime,
    /// The information needed by an IG publisher tool to publish the whole
    /// implementation guide.
    pub definition: super::implementation_guide::ImplementationGuideDefinition,
    /// Another implementation guide that this implementation depends on. Typically,
    /// an implementation guide uses value sets, profiles etc.defined in other
    /// implementation guides.
    pub depends_on: Vec<super::implementation_guide::ImplementationGuideDependsOn>,
    /// A free text natural language description of the implementation guide from a
    /// consumer's perspective.
    pub description: super::markdown::Markdown,
    /// A Boolean value to indicate that this implementation guide is authored for
    /// testing purposes (or education/evaluation/marketing) and is not intended to be
    /// used for genuine usage.
    pub experimental: super::boolean::Boolean,
    /// The version(s) of the FHIR specification that this ImplementationGuide targets
    /// - e.g. describes how to use. The value of this element is the formal version
    /// of the specification, without the revision number, e.g. [publication].[major].
    /// [minor], which is 4.6.0. for this version.
    pub fhir_version: Vec<super::code::Code>,
    /// A set of profiles that all resources covered by this implementation guide must
    /// conform to.
    pub global: Vec<super::implementation_guide::ImplementationGuideGlobal>,
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
    /// A legal or geographic region in which the implementation guide is intended to
    /// be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The license that applies to this Implementation Guide, using an SPDX license
    /// code, or 'not-open-source'.
    pub license: super::code::Code,
    /// Information about an assembled implementation guide, created by the publication
    /// tooling.
    pub manifest: super::implementation_guide::ImplementationGuideManifest,
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
    /// A natural language name identifying the implementation guide. This name should
    /// be usable as an identifier for the module by machine processing applications
    /// such as code generation.
    pub name: super::string::String,
    /// The NPM package name for this Implementation Guide, used in the NPM package
    /// distribution, which is the primary mechanism by which FHIR based tooling manages
    /// IG dependencies. This value must be globally unique, and should be assigned
    /// with care.
    pub package_id: super::id::Id,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the implementation guide.
    pub publisher: super::string::String,
    /// Explanation of why this implementation guide is needed and why it has been
    /// designed as it has.
    pub purpose: super::markdown::Markdown,
    /// This is a ImplementationGuide resource
    pub resource_type: String,
    /// The status of this implementation guide. Enables tracking the life-cycle of
    /// the content.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the implementation guide.
    pub title: super::string::String,
    /// An absolute URI that is used to identify this implementation guide when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this implementation guide is
    /// (or will be) published. This URL can be the target of a canonical reference.
    /// It SHALL remain the same when the implementation guide is stored on different
    /// servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate implementation
    /// guide instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the implementation guide
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the implementation guide author and is not expected
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

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideDefinition {
    /// A logical group of resources. Logical groups can be used when building pages.
    pub grouping: Vec<super::implementation_guide::ImplementationGuideGrouping>,
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
    /// A page / section in the implementation guide. The root page is the
    /// implementation guide home page.
    pub page: super::implementation_guide::ImplementationGuidePage,
    /// A set of parameters that defines how the implementation guide is built. The
    /// parameters are defined by the relevant tools that build the implementation
    /// guides.
    pub parameter: Vec<super::implementation_guide::ImplementationGuideParameter>,
    /// A resource that is part of the implementation guide. Conformance resources
    /// (value set, structure definition, capability statements etc.) are obvious
    /// candidates for inclusion, but any kind of resource can be included as an example
    /// resource.
    pub resource: Vec<super::implementation_guide::ImplementationGuideResource>,
    /// A template for building resources.
    pub template: Vec<super::implementation_guide::ImplementationGuideTemplate>,
}

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideDependsOn {
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
    /// The NPM package name for the Implementation Guide that this IG depends on.
    pub package_id: super::id::Id,
    /// A description explaining the nature of the dependency on the listed IG.
    pub reason: super::markdown::Markdown,
    /// A canonical reference to the Implementation guide for the dependency.
    pub uri: super::canonical::Canonical,
    /// The version of the IG that is depended on, when the correct version is required
    /// to understand the IG correctly.
    pub version: super::string::String,
}

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideGlobal {
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
    /// A reference to the profile that all instances must conform to.
    pub profile: super::canonical::Canonical,
    /// The type of resource that all instances must conform to.
    pub r#type: super::code::Code,
}

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideGrouping {
    /// Human readable text describing the package.
    pub description: super::markdown::Markdown,
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
    /// The human-readable title to display for the package of resources when rendering
    /// the implementation guide.
    pub name: super::string::String,
}

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideManifest {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Indicates a relative path to an image that exists within the IG.
    pub image: Vec<super::string::String>,
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
    /// Indicates the relative path of an additional non-page, non-image file that is
    /// part of the IG - e.g. zip, jar and similar files that could be the target of a
    /// hyperlink in a derived IG.
    pub other: Vec<super::string::String>,
    /// Information about a page within the IG.
    pub page: Vec<super::implementation_guide::ImplementationGuidePage1>,
    /// A pointer to official web page, PDF or other rendering of the implementation
    /// guide.
    pub rendering: super::url::Url,
    /// A resource that is part of the implementation guide. Conformance resources
    /// (value set, structure definition, capability statements etc.) are obvious
    /// candidates for inclusion, but any kind of resource can be included as an example
    /// resource.
    pub resource: Vec<super::implementation_guide::ImplementationGuideResource1>,
}

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuidePage {
    /// A code that indicates how the page is generated.
    pub generation: super::code::Code,
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
    /// The url by which the page should be known when published.
    pub name: super::url::Url,
    /// Nested Pages/Sections under this page.
    pub page: Vec<super::implementation_guide::ImplementationGuidePage>,
    /// Indicates the URL or the actual content to provide for the page.
    pub source_markdown: String,
    /// Indicates the URL or the actual content to provide for the page.
    pub source_string: String,
    /// Indicates the URL or the actual content to provide for the page.
    pub source_url: String,
    /// A short title used to represent this page in navigational structures such as
    /// table of contents, bread crumbs, etc.
    pub title: super::string::String,
}

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuidePage1 {
    /// The name of an anchor available on the page.
    pub anchor: Vec<super::string::String>,
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
    /// Relative path to the page.
    pub name: super::string::String,
    /// Label for the page intended for human display.
    pub title: super::string::String,
}

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideParameter {
    /// A tool-specific code that defines the parameter.
    pub code: super::coding::Coding,
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
    /// Value for named type.
    pub value: super::string::String,
}

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideResource {
    /// A description of the reason that a resource has been included in the
    /// implementation guide.
    pub description: super::markdown::Markdown,
    /// Indicates the FHIR Version(s) this artifact is intended to apply to. If no
    /// versions are specified, the resource is assumed to apply to all the versions
    /// stated in ImplementationGuide.fhirVersion.
    pub fhir_version: Vec<super::code::Code>,
    /// Reference to the id of the grouping this resource appears in.
    pub grouping_id: super::id::Id,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// If true, indicates the resource is an example instance.
    pub is_example: super::boolean::Boolean,
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
    /// A human assigned name for the resource. All resources SHOULD have a name, but
    /// the name may be extracted from the resource (e.g. ValueSet.name).
    pub name: super::string::String,
    /// If present, indicates profile(s) the instance is valid against.
    pub profile: Vec<super::canonical::Canonical>,
    /// Where this resource is found.
    pub reference: super::reference::Reference,
}

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideResource1 {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// If true, indicates the resource is an example instance.
    pub is_example: super::boolean::Boolean,
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
    /// If present, indicates profile(s) the instance is valid against.
    pub profile: Vec<super::canonical::Canonical>,
    /// Where this resource is found.
    pub reference: super::reference::Reference,
    /// The relative path for primary page for this resource within the IG.
    pub relative_path: super::url::Url,
}

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used
/// to gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideTemplate {
    /// Type of template specified.
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
    /// The scope in which the template applies.
    pub scope: super::string::String,
    /// The source location for the template.
    pub source: super::string::String,
}
