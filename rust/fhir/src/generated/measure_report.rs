/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureReport {
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Indicates whether the data submitted in a data-exchange report represents
    /// a snapshot or incremental update. A snapshot update replaces all previously
    /// submitted data for the receiver, whereas an incremental update represents only
    /// updated and/or changed data and should be applied as a differential update to
    /// the existing submitted data for the receiver.
    pub data_update_type: super::code::Code,
    /// The date this measure was calculated.
    pub date: super::date_time::DateTime,
    /// Evaluated resources are used to capture what data was involved in the
    /// calculation of a measure. This usage is only allowed for individual reports to
    /// ensure that the size of the MeasureReport resource is bounded.
    pub evaluated_resource: Vec<super::reference::Reference>,
    /// The results of the calculation, one for each population group in the measure.
    pub group: Vec<super::measure_report::MeasureReportGroup>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this MeasureReport when it is
    /// represented in other formats or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Whether improvement in the measure is noted by an increase or decrease in the
    /// measure score.
    pub improvement_notation: super::codeable_concept::CodeableConcept,
    /// A reference to a Parameters resource (typically represented using a contained
    /// resource) that represents any input parameters that were provided to the
    /// operation that generated the report.
    pub input_parameters: super::reference::Reference,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// A reference to the location for which the data is being reported.
    pub location: super::reference::Reference,
    /// A reference to the Measure that was calculated to produce this report.
    pub measure: super::canonical::Canonical,
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
    /// The reporting period for which the report was calculated.
    pub period: super::period::Period,
    /// The individual or organization that is reporting the data.
    pub reporter: super::reference::Reference,
    /// A reference to the vendor who queried the data, calculated results and/
    /// or generated the report. The ‘reporting vendor’ is intended to represent
    /// the submitting entity when it is not the same as the reporting entity. This
    /// extension is used when the Receiver is interested in getting vendor information
    /// in the report.
    pub reporting_vendor: super::reference::Reference,
    /// This is a MeasureReport resource
    pub resource_type: String,
    /// Indicates how the calculation is performed for the measure, including
    /// proportion, ratio, continuous-variable, and cohort. The value set is extensible,
    /// allowing additional measure scoring types to be represented. It is expected to
    /// be the same as the scoring element on the referenced Measure.
    pub scoring: super::codeable_concept::CodeableConcept,
    /// The MeasureReport status. No data will be available until the MeasureReport
    /// status is complete.
    pub status: super::code::Code,
    /// Optional subject identifying the individual or individuals the report is for.
    pub subject: super::reference::Reference,
    /// A reference to a Resource that represents additional information collected
    /// for the report. If the value of the supplemental data is not a Resource (i.e.
    /// evaluating the supplementalData expression for this case in the measure results
    /// in a value that is not a FHIR Resource), it is reported as a reference to a
    /// contained Observation resource.
    pub supplemental_data: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// The type of measure report. This may be an individual report, which provides
    /// the score for the measure for an individual member of the population; a subject-
    /// listing, which returns the list of members that meet the various criteria in
    /// the measure; a summary report, which returns a population count for each of the
    /// criteria in the measure; or a data-collection, which enables the MeasureReport
    /// to be used to exchange the data-of-interest for a quality measure.
    pub r#type: super::code::Code,
}

/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureReportComponent {
    /// The code for the stratum component value.
    pub code: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The stratifier component from the Measure that corresponds to this stratifier
    /// component in the MeasureReport resource.
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
    /// The stratum component value.
    pub value_boolean: bool,
    /// The stratum component value.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The stratum component value.
    pub value_quantity: super::quantity::Quantity,
    /// The stratum component value.
    pub value_range: super::range::Range,
    /// The stratum component value.
    pub value_reference: super::reference::Reference,
}

/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureReportGroup {
    /// The meaning of the population group as defined in the measure definition.
    pub code: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The group from the Measure that corresponds to this group in the MeasureReport
    /// resource.
    pub link_id: super::string::String,
    /// The measure score for this population group, calculated as appropriate for the
    /// measure type and scoring method, and based on the contents of the populations
    /// defined in the group.
    pub measure_score_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The measure score for this population group, calculated as appropriate for the
    /// measure type and scoring method, and based on the contents of the populations
    /// defined in the group.
    pub measure_score_date_time: String,
    /// The measure score for this population group, calculated as appropriate for the
    /// measure type and scoring method, and based on the contents of the populations
    /// defined in the group.
    pub measure_score_duration: super::duration::Duration,
    /// The measure score for this population group, calculated as appropriate for the
    /// measure type and scoring method, and based on the contents of the populations
    /// defined in the group.
    pub measure_score_period: super::period::Period,
    /// The measure score for this population group, calculated as appropriate for the
    /// measure type and scoring method, and based on the contents of the populations
    /// defined in the group.
    pub measure_score_quantity: super::quantity::Quantity,
    /// The measure score for this population group, calculated as appropriate for the
    /// measure type and scoring method, and based on the contents of the populations
    /// defined in the group.
    pub measure_score_range: super::range::Range,
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
    /// The populations that make up the population group, one for each type of
    /// population appropriate for the measure.
    pub population: Vec<super::measure_report::MeasureReportPopulation>,
    /// When a measure includes multiple stratifiers, there will be a stratifier group
    /// for each stratifier defined by the measure.
    pub stratifier: Vec<super::measure_report::MeasureReportStratifier>,
    /// Optional subject identifying the individual or individuals the report is for.
    pub subject: super::reference::Reference,
}

/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureReportPopulation {
    /// The type of the population.
    pub code: super::codeable_concept::CodeableConcept,
    /// The number of members of the population.
    pub count: super::integer::Integer,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The population from the Measure that corresponds to this population in the
    /// MeasureReport resource.
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
    /// A reference to an individual level MeasureReport resource for a member of the
    /// population.
    pub subject_report: Vec<super::reference::Reference>,
    /// This element refers to a List of individual level MeasureReport resources, one
    /// for each subject in this population.
    pub subject_results: super::reference::Reference,
    /// Optional Group identifying the individuals that make up the population.
    pub subjects: super::reference::Reference,
}

/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureReportPopulation1 {
    /// The type of the population.
    pub code: super::codeable_concept::CodeableConcept,
    /// The number of members of the population in this stratum.
    pub count: super::integer::Integer,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The population from the Measure that corresponds to this population in the
    /// MeasureReport resource.
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
    /// A reference to an individual level MeasureReport resource for a member of the
    /// population.
    pub subject_report: Vec<super::reference::Reference>,
    /// This element refers to a List of individual level MeasureReport resources, one
    /// for each subject in this population in this stratum.
    pub subject_results: super::reference::Reference,
    /// Optional Group identifying the individuals that make up the population.
    pub subjects: super::reference::Reference,
}

/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureReportStratifier {
    /// The meaning of this stratifier, as defined in the measure definition.
    pub code: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The stratifier from the Measure that corresponds to this stratifier in the
    /// MeasureReport resource.
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
    /// This element contains the results for a single stratum within the stratifier.
    /// For example, when stratifying on administrative gender, there will be four
    /// strata, one for each possible gender value.
    pub stratum: Vec<super::measure_report::MeasureReportStratum>,
}

/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureReportStratum {
    /// A stratifier component value.
    pub component: Vec<super::measure_report::MeasureReportComponent>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The measure score for this stratum, calculated as appropriate for the measure
    /// type and scoring method, and based on only the members of this stratum.
    pub measure_score_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The measure score for this stratum, calculated as appropriate for the measure
    /// type and scoring method, and based on only the members of this stratum.
    pub measure_score_date_time: String,
    /// The measure score for this stratum, calculated as appropriate for the measure
    /// type and scoring method, and based on only the members of this stratum.
    pub measure_score_duration: super::duration::Duration,
    /// The measure score for this stratum, calculated as appropriate for the measure
    /// type and scoring method, and based on only the members of this stratum.
    pub measure_score_period: super::period::Period,
    /// The measure score for this stratum, calculated as appropriate for the measure
    /// type and scoring method, and based on only the members of this stratum.
    pub measure_score_quantity: super::quantity::Quantity,
    /// The measure score for this stratum, calculated as appropriate for the measure
    /// type and scoring method, and based on only the members of this stratum.
    pub measure_score_range: super::range::Range,
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
    /// The populations that make up the stratum, one for each type of population
    /// appropriate to the measure.
    pub population: Vec<super::measure_report::MeasureReportPopulation1>,
    /// The value for this stratum, expressed as a CodeableConcept. When defining
    /// stratifiers on complex values, the value must be rendered such that the value
    /// for each stratum within the stratifier is unique.
    pub value_boolean: bool,
    /// The value for this stratum, expressed as a CodeableConcept. When defining
    /// stratifiers on complex values, the value must be rendered such that the value
    /// for each stratum within the stratifier is unique.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The value for this stratum, expressed as a CodeableConcept. When defining
    /// stratifiers on complex values, the value must be rendered such that the value
    /// for each stratum within the stratifier is unique.
    pub value_quantity: super::quantity::Quantity,
    /// The value for this stratum, expressed as a CodeableConcept. When defining
    /// stratifiers on complex values, the value must be rendered such that the value
    /// for each stratum within the stratifier is unique.
    pub value_range: super::range::Range,
    /// The value for this stratum, expressed as a CodeableConcept. When defining
    /// stratifiers on complex values, the value must be rendered such that the value
    /// for each stratum within the stratifier is unique.
    pub value_reference: super::reference::Reference,
}
