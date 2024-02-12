/// The Measure resource provides the definition of a quality measure.
#[derive(Debug, Clone, PartialEq)]
pub struct Measure {
    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// An individiual or organization primarily involved in the creation and
    /// maintenance of the content.
    pub author: Vec<super::contact_detail::ContactDetail>,
    /// The population basis specifies the type of elements in the population. For a
    /// subject-based measure, this is boolean (because the subject and the population
    /// basis are the same, and the population criteria define yes/no values for each
    /// individual in the population). For measures that have a population basis that
    /// is different than the subject, this element specifies the type of the population
    /// basis. For example, an encounter-based measure has a subject of Patient and
    /// a population basis of Encounter, and the population criteria all return lists
    /// of Encounters.
    pub basis: super::code::Code,
    /// Provides a summary of relevant clinical guidelines or other clinical
    /// recommendations supporting the measure.
    pub clinical_recommendation_statement: super::markdown::Markdown,
    /// If this is a composite measure, the scoring method used to combine the component
    /// measures to determine the composite score.
    pub composite_scoring: super::codeable_concept::CodeableConcept,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the measure and/or its contents. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// measure.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the measure was last significantly changed.
    /// The date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content
    /// of the measure changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the measure from a consumer's
    /// perspective.
    pub description: super::markdown::Markdown,
    /// Notices and disclaimers regarding the use of the measure or related to
    /// intellectual property (such as code systems) referenced by the measure.
    pub disclaimer: super::markdown::Markdown,
    /// An individual or organization primarily responsible for internal coherence of
    /// the content.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the measure content was or is planned to be in active
    /// use.
    pub effective_period: super::period::Period,
    /// An individual or organization asserted by the publisher to be responsible for
    /// officially endorsing the content for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A Boolean value to indicate that this measure is authored for testing purposes
    /// (or education/evaluation/marketing) and is not intended to be used for genuine
    /// usage.
    pub experimental: super::boolean::Boolean,
    /// A group of population criteria for the measure.
    pub group: Vec<super::measure::MeasureGroup>,
    /// Additional guidance for the measure including how it can be used in a clinical
    /// context, and the intent of the measure.
    pub guidance: super::markdown::Markdown,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this measure when it is represented
    /// in other formats, or referenced in a specification, model, design or an
    /// instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Information on whether an increase or decrease in score is the preferred result
    /// (e.g., a higher score indicates better quality OR a lower score indicates better
    /// quality OR quality is within a range).
    pub improvement_notation: super::codeable_concept::CodeableConcept,
    /// A legal or geographic region in which the measure is intended to be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The date on which the resource content was last reviewed. Review happens
    /// periodically after approval but does not change the original approval date.
    pub last_review_date: super::date::Date,
    /// A reference to a Library resource containing the formal logic used by the
    /// measure.
    pub library: Vec<super::canonical::Canonical>,
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
    /// A natural language name identifying the measure. This name should be usable as
    /// an identifier for the module by machine processing applications such as code
    /// generation.
    pub name: super::string::String,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the measure.
    pub publisher: super::string::String,
    /// Explanation of why this measure is needed and why it has been designed as it
    /// has.
    pub purpose: super::markdown::Markdown,
    /// Describes how to combine the information calculated, based on logic in each of
    /// several populations, into one summarized result.
    pub rate_aggregation: super::markdown::Markdown,
    /// Provides a succinct statement of the need for the measure. Usually includes
    /// statements pertaining to importance criterion: impact, gap in care, and
    /// evidence.
    pub rationale: super::markdown::Markdown,
    /// Related artifacts such as additional documentation, justification, or
    /// bibliographic references.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a Measure resource
    pub resource_type: String,
    /// An individual or organization asserted by the publisher to be primarily
    /// responsible for review of some aspect of the content.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// A description of the risk adjustment factors that may impact the resulting score
    /// for the measure and how they may be accounted for when computing and reporting
    /// measure results.
    pub risk_adjustment: super::markdown::Markdown,
    /// Indicates how the calculation is performed for the measure, including
    /// proportion, ratio, continuous-variable, and cohort. The value set is extensible,
    /// allowing additional measure scoring types to be represented.
    pub scoring: super::codeable_concept::CodeableConcept,
    /// Defines the expected units of measure for the measure score. This element SHOULD
    /// be specified as a UCUM unit.
    pub scoring_unit: super::codeable_concept::CodeableConcept,
    /// The status of this measure. Enables tracking the life-cycle of the content.
    pub status: super::code::Code,
    /// The intended subjects for the measure. If this element is not provided, a
    /// Patient subject is assumed, but the subject of the measure can be anything.
    pub subject_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The intended subjects for the measure. If this element is not provided, a
    /// Patient subject is assumed, but the subject of the measure can be anything.
    pub subject_reference: super::reference::Reference,
    /// An explanatory or alternate title for the measure giving additional information
    /// about its content.
    pub subtitle: super::string::String,
    /// The supplemental data criteria for the measure report, specified as either the
    /// name of a valid CQL expression within a referenced library, or a valid FHIR
    /// Resource Path.
    pub supplemental_data: Vec<super::measure::MeasureSupplementalData>,
    /// Provides a description of an individual term used within the measure.
    pub term: Vec<super::measure::MeasureTerm>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the measure.
    pub title: super::string::String,
    /// Descriptive topics related to the content of the measure. Topics provide a high-
    /// level categorization grouping types of measures that can be useful for filtering
    /// and searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// Indicates whether the measure is used to examine a process, an outcome over
    /// time, a patient-reported outcome, or a structure measure such as utilization.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
    /// An absolute URI that is used to identify this measure when it is referenced
    /// in a specification, model, design or an instance; also called its canonical
    /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
    /// which an authoritative instance of this measure is (or will be) published. This
    /// URL can be the target of a canonical reference. It SHALL remain the same when
    /// the measure is stored on different servers.
    pub url: super::uri::Uri,
    /// A detailed description, from a clinical perspective, of how the measure is used.
    pub usage: super::markdown::Markdown,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate measure instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the measure when it is
    /// referenced in a specification, model, design or instance. This is an arbitrary
    /// value managed by the measure author and is not expected to be globally unique.
    /// For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is
    /// not available. There is also no expectation that versions can be placed in a
    /// lexicographical sequence. To provide a version consistent with the Decision
    /// Support Service specification, use the format Major.Minor.Revision (e.g.
    /// 1.0.0). For more information on versioning knowledge assets, refer to the
    /// Decision Support Service specification. Note that a version is required for non-
    /// experimental active artifacts.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// The Measure resource provides the definition of a quality measure.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureComponent {
    /// Indicates a meaning for the stratifier component. This can be as simple as a
    /// unique identifier, or it can establish meaning in a broader context by drawing
    /// from a terminology, allowing stratifiers to be correlated across measures.
    pub code: super::codeable_concept::CodeableConcept,
    /// An expression that specifies the criteria for this component of the stratifier.
    /// This is typically the name of an expression defined within a referenced library,
    /// but it may also be a path to a stratifier element.
    pub criteria: super::expression::Expression,
    /// The human readable description of this stratifier criteria component.
    pub description: super::markdown::Markdown,
    /// A Group resource that defines this population as a set of characteristics.
    pub group_definition: super::reference::Reference,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// An identifier that is unique within the Measure allowing linkage to the
    /// equivalent item in a MeasureReport resource.
    pub link_id: super::string::String,
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

/// The Measure resource provides the definition of a quality measure.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureGroup {
    /// The population basis specifies the type of elements in the population. For a
    /// subject-based measure, this is boolean (because the subject and the population
    /// basis are the same, and the population criteria define yes/no values for each
    /// individual in the population). For measures that have a population basis that
    /// is different than the subject, this element specifies the type of the population
    /// basis. For example, an encounter-based measure has a subject of Patient and
    /// a population basis of Encounter, and the population criteria all return lists
    /// of Encounters.
    pub basis: super::code::Code,
    /// Indicates a meaning for the group. This can be as simple as a unique identifier,
    /// or it can establish meaning in a broader context by drawing from a terminology,
    /// allowing groups to be correlated across measures.
    pub code: super::codeable_concept::CodeableConcept,
    /// The human readable description of this population group.
    pub description: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Information on whether an increase or decrease in score is the preferred result
    /// (e.g., a higher score indicates better quality OR a lower score indicates better
    /// quality OR quality is within a range).
    pub improvement_notation: super::codeable_concept::CodeableConcept,
    /// A reference to a Library resource containing the formal logic used by the
    /// measure group.
    pub library: Vec<super::canonical::Canonical>,
    /// An identifier that is unique within the Measure allowing linkage to the
    /// equivalent item in a MeasureReport resource.
    pub link_id: super::string::String,
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
    /// A population criteria for the measure.
    pub population: Vec<super::measure::MeasurePopulation>,
    /// Describes how to combine the information calculated, based on logic in each of
    /// several populations, into one summarized result.
    pub rate_aggregation: super::markdown::Markdown,
    /// Indicates how the calculation is performed for the measure, including
    /// proportion, ratio, continuous-variable, and cohort. The value set is extensible,
    /// allowing additional measure scoring types to be represented.
    pub scoring: super::codeable_concept::CodeableConcept,
    /// Defines the expected units of measure for the measure score. This element SHOULD
    /// be specified as a UCUM unit.
    pub scoring_unit: super::codeable_concept::CodeableConcept,
    /// The stratifier criteria for the measure report, specified as either the name
    /// of a valid CQL expression defined within a referenced library or a valid FHIR
    /// Resource Path.
    pub stratifier: Vec<super::measure::MeasureStratifier>,
    /// The intended subjects for the measure. If this element is not provided, a
    /// Patient subject is assumed, but the subject of the measure can be anything.
    pub subject_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The intended subjects for the measure. If this element is not provided, a
    /// Patient subject is assumed, but the subject of the measure can be anything.
    pub subject_reference: super::reference::Reference,
    /// Indicates whether the measure is used to examine a process, an outcome over
    /// time, a patient-reported outcome, or a structure measure such as utilization.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
}

/// The Measure resource provides the definition of a quality measure.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasurePopulation {
    /// Specifies which method should be used to aggregate measure observation values.
    /// For most scoring types, this is implied by scoring (e.g. a proportion measure
    /// counts members of the populations). For continuous variables, however, this
    /// information must be specified to ensure correct calculation.
    pub aggregate_method: super::codeable_concept::CodeableConcept,
    /// The type of population criteria.
    pub code: super::codeable_concept::CodeableConcept,
    /// An expression that specifies the criteria for the population, typically the name
    /// of an expression in a library.
    pub criteria: super::expression::Expression,
    /// The human readable description of this population criteria.
    pub description: super::markdown::Markdown,
    /// A Group resource that defines this population as a set of characteristics.
    pub group_definition: super::reference::Reference,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The id of a population element in this measure that provides the input for this
    /// population criteria. In most cases, the scoring structure of the measure implies
    /// specific relationships (e.g. the Numerator uses the Denominator as the source in
    /// a proportion scoring). In some cases, however, multiple possible choices exist
    /// and must be resolved explicitly. For example in a ratio measure with multiple
    /// initial populations, the denominator must specify which population should be
    /// used as the starting point.
    pub input_population_id: super::string::String,
    /// An identifier that is unique within the Measure allowing linkage to the
    /// equivalent population in a MeasureReport resource.
    pub link_id: super::string::String,
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

/// The Measure resource provides the definition of a quality measure.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureStratifier {
    /// Indicates a meaning for the stratifier. This can be as simple as a unique
    /// identifier, or it can establish meaning in a broader context by drawing from a
    /// terminology, allowing stratifiers to be correlated across measures.
    pub code: super::codeable_concept::CodeableConcept,
    /// A component of the stratifier criteria for the measure report, specified as
    /// either the name of a valid CQL expression defined within a referenced library or
    /// a valid FHIR Resource Path.
    pub component: Vec<super::measure::MeasureComponent>,
    /// An expression that specifies the criteria for the stratifier. This is typically
    /// the name of an expression defined within a referenced library, but it may also
    /// be a path to a stratifier element.
    pub criteria: super::expression::Expression,
    /// The human readable description of this stratifier criteria.
    pub description: super::markdown::Markdown,
    /// A Group resource that defines this population as a set of characteristics.
    pub group_definition: super::reference::Reference,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// An identifier that is unique within the Measure allowing linkage to the
    /// equivalent item in a MeasureReport resource.
    pub link_id: super::string::String,
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

/// The Measure resource provides the definition of a quality measure.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureSupplementalData {
    /// Indicates a meaning for the supplemental data. This can be as simple as a unique
    /// identifier, or it can establish meaning in a broader context by drawing from a
    /// terminology, allowing supplemental data to be correlated across measures.
    pub code: super::codeable_concept::CodeableConcept,
    /// The criteria for the supplemental data. This is typically the name of a valid
    /// expression defined within a referenced library, but it may also be a path to
    /// a specific data element. The criteria defines the data to be returned for this
    /// element.
    pub criteria: super::expression::Expression,
    /// The human readable description of this supplemental data.
    pub description: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// An identifier that is unique within the Measure allowing linkage to the
    /// equivalent item in a MeasureReport resource.
    pub link_id: super::string::String,
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
    /// An indicator of the intended usage for the supplemental data element.
    /// Supplemental data indicates the data is additional information requested to
    /// augment the measure information. Risk adjustment factor indicates the data is
    /// additional information used to calculate risk adjustment factors when applying a
    /// risk model to the measure calculation.
    pub usage: Vec<super::codeable_concept::CodeableConcept>,
}

/// The Measure resource provides the definition of a quality measure.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureTerm {
    /// A codeable representation of the defined term.
    pub code: super::codeable_concept::CodeableConcept,
    /// Provides a definition for the term as used within the measure.
    pub definition: super::markdown::Markdown,
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
