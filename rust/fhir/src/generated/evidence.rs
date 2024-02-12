/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (e.g., population, exposures/
/// interventions, comparators, outcomes, measured variables, confounding
/// variables), the statistics, and the certainty of this evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct Evidence {
    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// Declarative description of the Evidence.
    pub assertion: super::markdown::Markdown,
    /// An individiual, organization, or device primarily involved in the creation and
    /// maintenance of the content.
    pub author: Vec<super::contact_detail::ContactDetail>,
    /// Assessment of certainty, confidence in the estimates, or quality of the
    /// evidence.
    pub certainty: Vec<super::evidence::EvidenceCertainty>,
    /// Citation Resource or display of suggested citation for this evidence.
    pub cite_as_markdown: String,
    /// Citation Resource or display of suggested citation for this evidence.
    pub cite_as_reference: super::reference::Reference,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the Evidence and/or its contents. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// Evidence.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the summary was last significantly changed.
    /// The date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content
    /// of the summary changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the evidence from a consumer's
    /// perspective.
    pub description: super::markdown::Markdown,
    /// An individiual, organization, or device primarily responsible for internal
    /// coherence of the content.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the {{title}} content was or is planned to be in active
    /// use.
    pub effective_period: super::period::Period,
    /// An individiual, organization, or device responsible for officially endorsing the
    /// content for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A Boolean value to indicate that this resource is authored for testing purposes
    /// (or education/evaluation/marketing) and is not intended to be used for genuine
    /// usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this summary when it is represented
    /// in other formats, or referenced in a specification, model, design or an
    /// instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the {{title}} is intended to be used.
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
    /// A natural language name identifying the evidence. This name should be usable
    /// as an identifier for the module by machine processing applications such as code
    /// generation.
    pub name: super::string::String,
    /// Footnotes and/or explanatory notes.
    pub note: Vec<super::annotation::Annotation>,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the evidence.
    pub publisher: super::string::String,
    /// Explanation of why this Evidence is needed and why it has been designed as it
    /// has.
    pub purpose: super::markdown::Markdown,
    /// Link or citation to artifact associated with the summary.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a Evidence resource
    pub resource_type: String,
    /// An individiual, organization, or device primarily responsible for review of some
    /// aspect of the content.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// Values and parameters for a single statistic.
    pub statistic: Vec<super::evidence::EvidenceStatistic>,
    /// The status of this summary. Enables tracking the life-cycle of the content.
    pub status: super::code::Code,
    /// The design of the study that produced this evidence. The design is described
    /// with any number of study design characteristics.
    pub study_design: Vec<super::codeable_concept::CodeableConcept>,
    /// The method to combine studies.
    pub synthesis_type: super::codeable_concept::CodeableConcept,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the summary.
    pub title: super::string::String,
    /// Descriptive topics related to the content of the {{title}}. Topics provide a
    /// high-level categorization as well as keywords for the {{title}} that can be
    /// useful for filtering and searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// An absolute URI that is used to identify this evidence when it is referenced
    /// in a specification, model, design or an instance; also called its canonical
    /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
    /// which an authoritative instance of this summary is (or will be) published. This
    /// URL can be the target of a canonical reference. It SHALL remain the same when
    /// the summary is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...)
    /// or may be references to specific programs (insurance plans, studies, ...) and
    /// may be used to assist with indexing and searching for appropriate evidence
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// Evidence variable such as population, exposure, or outcome.
    pub variable_definition: Vec<super::evidence::EvidenceVariableDefinition>,
    /// The identifier that is used to identify this version of the summary when it is
    /// referenced in a specification, model, design or instance. This is an arbitrary
    /// value managed by the summary author and is not expected to be globally unique.
    /// For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is
    /// not available. There is also no expectation that versions can be placed in a
    /// lexicographical sequence.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (e.g., population, exposures/
/// interventions, comparators, outcomes, measured variables, confounding
/// variables), the statistics, and the certainty of this evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceAttributeEstimate {
    /// A nested attribute estimate; which is the attribute estimate of an attribute
    /// estimate.
    pub attribute_estimate: Vec<super::evidence::EvidenceAttributeEstimate>,
    /// Human-readable summary of the estimate.
    pub description: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Use 95 for a 95% confidence interval.
    pub level: super::decimal::Decimal,
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
    /// Footnote or explanatory note about the estimate.
    pub note: Vec<super::annotation::Annotation>,
    /// The singular quantity of the attribute estimate, for attribute estimates
    /// represented as single values; also used to report unit of measure.
    pub quantity: super::quantity::Quantity,
    /// Lower bound of confidence interval.
    pub range: super::range::Range,
    /// The type of attribute estimate, e.g., confidence interval or p value.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (e.g., population, exposures/
/// interventions, comparators, outcomes, measured variables, confounding
/// variables), the statistics, and the certainty of this evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceCertainty {
    /// Textual description of certainty.
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
    /// Footnotes and/or explanatory notes.
    pub note: Vec<super::annotation::Annotation>,
    /// Individual or group who did the rating.
    pub rater: super::string::String,
    /// Assessment or judgement of the aspect.
    pub rating: super::codeable_concept::CodeableConcept,
    /// A domain or subdomain of certainty.
    pub subcomponent: Vec<super::evidence::EvidenceCertainty>,
    /// Aspect of certainty being rated.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (e.g., population, exposures/
/// interventions, comparators, outcomes, measured variables, confounding
/// variables), the statistics, and the certainty of this evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceModelCharacteristic {
    /// An attribute of the statistic used as a model characteristic.
    pub attribute_estimate: Vec<super::evidence::EvidenceAttributeEstimate>,
    /// Description of a component of the method to generate the statistic.
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
    /// Further specification of the quantified value of the component of the method to
    /// generate the statistic.
    pub value: super::quantity::Quantity,
    /// A variable adjusted for in the adjusted analysis.
    pub variable: Vec<super::evidence::EvidenceVariable>,
}

/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (e.g., population, exposures/
/// interventions, comparators, outcomes, measured variables, confounding
/// variables), the statistics, and the certainty of this evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceSampleSize {
    /// Human-readable summary of population sample size.
    pub description: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Number of participants with known results for measured variables.
    pub known_data_count: super::unsigned_int::UnsignedInt,
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
    /// Footnote or explanatory note about the sample size.
    pub note: Vec<super::annotation::Annotation>,
    /// A human-readable string to clarify or explain concepts about the sample size.
    pub number_of_participants: super::unsigned_int::UnsignedInt,
    /// Number of participants in the population.
    pub number_of_studies: super::unsigned_int::UnsignedInt,
}

/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (e.g., population, exposures/
/// interventions, comparators, outcomes, measured variables, confounding
/// variables), the statistics, and the certainty of this evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceStatistic {
    /// A statistical attribute of the statistic such as a measure of heterogeneity.
    pub attribute_estimate: Vec<super::evidence::EvidenceAttributeEstimate>,
    /// When the measured variable is handled categorically, the category element is
    /// used to define which category the statistic is reporting.
    pub category: super::codeable_concept::CodeableConcept,
    /// A description of the content value of the statistic.
    pub description: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A component of the method to generate the statistic.
    pub model_characteristic: Vec<super::evidence::EvidenceModelCharacteristic>,
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
    /// Footnotes and/or explanatory notes.
    pub note: Vec<super::annotation::Annotation>,
    /// The number of participants affected where the unit of analysis is the same as
    /// sampleSize.knownDataCount and sampleSize.numberOfParticipants.
    pub number_affected: super::unsigned_int::UnsignedInt,
    /// The number of events associated with the statistic, where the unit of
    /// analysis is different from numberAffected, sampleSize.knownDataCount and
    /// sampleSize.numberOfParticipants.
    pub number_of_events: super::unsigned_int::UnsignedInt,
    /// Statistic value.
    pub quantity: super::quantity::Quantity,
    /// Number of samples in the statistic.
    pub sample_size: super::evidence::EvidenceSampleSize,
    /// Type of statistic, e.g., relative risk.
    pub statistic_type: super::codeable_concept::CodeableConcept,
}

/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (e.g., population, exposures/
/// interventions, comparators, outcomes, measured variables, confounding
/// variables), the statistics, and the certainty of this evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariable {
    /// How the variable is classified for use in adjusted analysis.
    pub handling: super::code::Code,
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
    /// Description for grouping of ordinal or polychotomous variables.
    pub value_category: Vec<super::codeable_concept::CodeableConcept>,
    /// Discrete value for grouping of ordinal or polychotomous variables.
    pub value_quantity: Vec<super::quantity::Quantity>,
    /// Range of values for grouping of ordinal or polychotomous variables.
    pub value_range: Vec<super::range::Range>,
    /// Description of the variable.
    pub variable_definition: super::reference::Reference,
}

/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (e.g., population, exposures/
/// interventions, comparators, outcomes, measured variables, confounding
/// variables), the statistics, and the certainty of this evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariableDefinition {
    /// A text description or summary of the variable.
    pub description: super::markdown::Markdown,
    /// Indication of quality of match between intended variable to actual variable.
    pub directness_match: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Definition of the intended variable related to the Evidence.
    pub intended: super::reference::Reference,
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
    /// Footnotes and/or explanatory notes.
    pub note: Vec<super::annotation::Annotation>,
    /// Definition of the actual variable related to the statistic(s).
    pub observed: super::reference::Reference,
    /// population | subpopulation | exposure | referenceExposure | measuredVariable
    /// | confounder.
    pub variable_role: super::codeable_concept::CodeableConcept,
}
