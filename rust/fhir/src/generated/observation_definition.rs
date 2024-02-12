/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationDefinition {
    /// The date on which the asset content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// An individiual or organization primarily involved in the creation and
    /// maintenance of the {{title}}.
    pub author: Vec<super::contact_detail::ContactDetail>,
    /// The site on the subject's body where the  observation is to be made.
    pub body_site: super::codeable_concept::CodeableConcept,
    /// A code that classifies the general type of observation.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// Describes what will be observed. Sometimes this is called the observation
    /// "name".
    pub code: super::codeable_concept::CodeableConcept,
    /// Some observations have multiple component observations, expressed as separate
    /// code value pairs.
    pub component: Vec<super::observation_definition::ObservationDefinitionComponent>,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Copyright statement relating to the ObservationDefinition and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the ObservationDefinition.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date (and optionally time) when the ObservationDefinition was last
    /// significantly changed. The date must change when the business version changes
    /// and it must change if the status code changes. In addition, it should change
    /// when the substantive content of the ObservationDefinition changes.
    pub date: super::date_time::DateTime,
    /// The canonical URL pointing to another FHIR-defined ObservationDefinition that is
    /// adhered to in whole or in part by this definition.
    pub derived_from_canonical: Vec<super::canonical::Canonical>,
    /// The URL pointing to an externally-defined observation definition, guideline or
    /// other definition that is adhered to in whole or in part by this definition.
    pub derived_from_uri: Vec<super::uri::Uri>,
    /// A free text natural language description of the ObservationDefinition from the
    /// consumer's perspective.
    pub description: super::markdown::Markdown,
    /// The measurement model of device or actual device used to produce observations of
    /// this type.
    pub device: Vec<super::reference::Reference>,
    /// An individual or organization primarily responsible for internal coherence of
    /// the {{title}}.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the ObservationDefinition content was or is planned to
    /// be effective.
    pub effective_period: super::period::Period,
    /// An individual or organization asserted by the publisher to be responsible for
    /// officially endorsing the {{title}} for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A flag to indicate that this ObservationDefinition is authored for testing
    /// purposes (or education/evaluation/marketing), and is not intended to be used for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// This ObservationDefinition defines a group  observation (e.g. a battery, a panel
    /// of tests, a set of vital sign measurements) that includes the target as a member
    /// of the group.
    pub has_member: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this ObservationDefinition. by the performer
    /// and/or other systems. These identifiers remain constant as the resource is
    /// updated and propagates from server to server.
    pub identifier: super::identifier::Identifier,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A jurisdiction in which the ObservationDefinition is intended to be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The date on which the asset content was last reviewed. Review happens
    /// periodically after that, but doesn't change the original approval date.
    pub last_review_date: super::date::Date,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// The method or technique used to perform the observation.
    pub method: super::codeable_concept::CodeableConcept,
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
    /// Multiple results allowed for observations conforming to this
    /// ObservationDefinition.
    pub multiple_results_allowed: super::boolean::Boolean,
    /// A natural language name identifying the ObservationDefinition. This name should
    /// be usable as an identifier for the module by machine processing applications
    /// such as code generation.
    pub name: super::string::String,
    /// The type of individual/organization/device that is expected to act upon
    /// instances of this definition.
    pub performer_type: super::codeable_concept::CodeableConcept,
    /// The data types allowed for the value element of the instance observations
    /// conforming to this ObservationDefinition.
    pub permitted_data_type: Vec<super::code::Code>,
    /// Units allowed for the valueQuantity element in the instance observations
    /// conforming to this ObservationDefinition.
    pub permitted_unit: Vec<super::coding::Coding>,
    /// The preferred name to be used when reporting the results of observations
    /// conforming to this ObservationDefinition.
    pub preferred_report_name: super::string::String,
    /// Helps establish the "authority/credibility" of the ObservationDefinition. May
    /// also allow for contact.
    pub publisher: super::string::String,
    /// Explains why this ObservationDefinition is needed and why it has been designed
    /// as it has.
    pub purpose: super::markdown::Markdown,
    /// A set of qualified values associated with a context and a set of conditions -
    /// provides a range for quantitative and ordinal observations and a collection of
    /// value sets for qualitative observations.
    pub qualified_value: Vec<super::observation_definition::ObservationDefinitionQualifiedValue>,
    /// Related artifacts such as additional documentation, justification, dependencies,
    /// bibliographic references, and predecessor and successor artifacts.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a ObservationDefinition resource
    pub resource_type: String,
    /// An individual or organization asserted by the publisher to be primarily
    /// responsible for review of some aspect of the {{title}}.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// The kind of specimen that this type of observation is produced on.
    pub specimen: Vec<super::reference::Reference>,
    /// The current state of the ObservationDefinition.
    pub status: super::code::Code,
    /// A code that describes the intended kind of subject of Observation instances
    /// conforming to this ObservationDefinition.
    pub subject: Vec<super::codeable_concept::CodeableConcept>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the ObservationDefinition.
    pub title: super::string::String,
    /// Descriptive topics related to the content of the {{title}}. Topics provide a
    /// high-level categorization as well as keywords for the {{title}} that can be
    /// useful for filtering and searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// An absolute URL that is used to identify this ObservationDefinition when it
    /// is referenced in a specification, model, design or an instance. This SHALL
    /// be a URL, SHOULD be globally unique, and SHOULD be an address at which this
    /// ObservationDefinition is (or will be) published. The URL SHOULD include the
    /// major version of the ObservationDefinition. For more information see Technical
    /// and Business Versions.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...)
    /// or may be references to specific programs (insurance plans, studies, ...)
    /// and may be used to assist with indexing and searching for appropriate
    /// ObservationDefinition instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the
    /// ObservationDefinition when it is referenced in a specification, model, design
    /// or instance. This is an arbitrary value managed by the ObservationDefinition
    /// author and is not expected to be globally unique. For example, it might be a
    /// timestamp (e.g. yyyymmdd) if a managed version is not available. There is also
    /// no expectation that versions are orderable.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationDefinitionComponent {
    /// Describes what will be observed.
    pub code: super::codeable_concept::CodeableConcept,
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
    /// The data types allowed for the value element of the instance of this component
    /// observations.
    pub permitted_data_type: Vec<super::code::Code>,
    /// Units allowed for the valueQuantity element in the instance observations
    /// conforming to this ObservationDefinition.
    pub permitted_unit: Vec<super::coding::Coding>,
    /// A set of qualified values associated with a context and a set of conditions -
    /// provides a range for quantitative and ordinal observations and a collection of
    /// value sets for qualitative observations.
    pub qualified_value: Vec<super::observation_definition::ObservationDefinitionQualifiedValue>,
}

/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationDefinitionQualifiedValue {
    /// The set of abnormal coded results for qualitative observations  that match the
    /// criteria of this set of qualified values.
    pub abnormal_coded_value_set: super::canonical::Canonical,
    /// The age range this  set of qualified values applies to.
    pub age: super::range::Range,
    /// The target population this  set of qualified values applies to.
    pub applies_to: Vec<super::codeable_concept::CodeableConcept>,
    /// Text based condition for which the the set of qualified values is valid.
    pub condition: super::string::String,
    /// A concept defining the context for this set of qualified values.
    pub context: super::codeable_concept::CodeableConcept,
    /// The set of critical coded results for qualitative observations  that match the
    /// criteria of this set of qualified values.
    pub critical_coded_value_set: super::canonical::Canonical,
    /// The gender this  set of qualified values applies to.
    pub gender: super::code::Code,
    /// The gestational age this  set of qualified values applies to.
    pub gestational_age: super::range::Range,
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
    /// The set of normal coded results for qualitative observations  that match the
    /// criteria of this set of qualified values.
    pub normal_coded_value_set: super::canonical::Canonical,
    /// The range of values defined for continuous or ordinal observations that match
    /// the criteria of this set of qualified values.
    pub range: super::range::Range,
    /// The category of range of values for continuous or ordinal observations that
    /// match the criteria of this set of qualified values.
    pub range_category: super::code::Code,
    /// The set of valid coded results for qualitative observations  that match the
    /// criteria of this set of qualified values.
    pub valid_coded_value_set: super::canonical::Canonical,
}
