/// A search parameter that defines a named search item that can be used to search/
/// filter on a resource.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchParameter {
    /// The base resource type(s) that this search parameter can be used against.
    pub base: Vec<super::code::Code>,
    /// Contains the names of any search parameters which may be chained to the
    /// containing search parameter. Chained parameters may be added to search
    /// parameters of type reference and specify that resources will only be returned
    /// if they contain a reference to a resource which matches the chained parameter
    /// value. Values for this field should be drawn from SearchParameter.code for a
    /// parameter on the target resource type.
    pub chain: Vec<super::string::String>,
    /// The label that is recommended to be used in the URL or the parameter name in a
    /// parameters resource for this search parameter.  In some cases, servers may need
    /// to use a different CapabilityStatement searchParam.name to differentiate between
    /// multiple SearchParameters that happen to have the same code.
    pub code: super::code::Code,
    /// Comparators supported for the search parameter.
    pub comparator: Vec<super::code::Code>,
    /// Used to define the parts of a composite search parameter.
    pub component: Vec<super::search_parameter::SearchParameterComponent>,
    /// FHIRPath expression that defines/sets a complex constraint for when this
    /// SearchParameter is applicable.
    pub constraint: super::string::String,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the search parameter and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the search parameter.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the search parameter was last significantly
    /// changed. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the search parameter changes.
    pub date: super::date_time::DateTime,
    /// Where this search parameter is originally defined. If a derivedFrom is provided,
    /// then the details in the search parameter must be consistent with the definition
    /// from which it is defined. i.e. the parameter should have the same meaning, and
    /// (usually) the functionality should be a proper subset of the underlying search
    /// parameter.
    pub derived_from: super::canonical::Canonical,
    /// And how it used.
    pub description: super::markdown::Markdown,
    /// A Boolean value to indicate that this search parameter is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// A FHIRPath expression that returns a set of elements for the search parameter.
    pub expression: super::string::String,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this search parameter when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the search parameter is intended to be
    /// used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// A modifier supported for the search parameter.
    pub modifier: Vec<super::code::Code>,
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
    /// Whether multiple parameters are allowed - e.g. more than one parameter with the
    /// same name. The search matches if all the parameters match.
    pub multiple_and: super::boolean::Boolean,
    /// Whether multiple values are allowed for each time the parameter exists. Values
    /// are separated by commas, and the parameter matches if any of the values match.
    pub multiple_or: super::boolean::Boolean,
    /// A natural language name identifying the search parameter. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub name: super::string::String,
    /// How the search parameter relates to the set of elements returned by evaluating
    /// the expression query.
    pub processing_mode: super::code::Code,
    /// The name of the organization or individual tresponsible for the release and
    /// ongoing maintenance of the search parameter.
    pub publisher: super::string::String,
    /// Explanation of why this search parameter is needed and why it has been designed
    /// as it has.
    pub purpose: super::markdown::Markdown,
    /// This is a SearchParameter resource
    pub resource_type: String,
    /// The status of this search parameter. Enables tracking the life-cycle of the
    /// content.
    pub status: super::code::Code,
    /// Types of resource (if a resource is referenced).
    pub target: Vec<super::code::Code>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the search parameter.
    pub title: super::string::String,
    /// The type of value that a search parameter may contain, and how the content is
    /// interpreted.
    pub r#type: super::code::Code,
    /// An absolute URI that is used to identify this search parameter when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this search parameter is (or will
    /// be) published. This URL can be the target of a canonical reference. It SHALL
    /// remain the same when the search parameter is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate search parameter
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the search parameter
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the search parameter author and is not expected to
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

/// A search parameter that defines a named search item that can be used to search/
/// filter on a resource.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchParameterComponent {
    /// The definition of the search parameter that describes this part.
    pub definition: super::canonical::Canonical,
    /// A sub-expression that defines how to extract values for this component from the
    /// output of the main SearchParameter.expression.
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
}
