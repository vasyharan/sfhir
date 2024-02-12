/// A GenomicStudy is a set of analyses performed to analyze and generate genomic
/// data.
#[derive(Debug, Clone, PartialEq)]
pub struct GenomicStudy {
    /// The details about a specific analysis that was performed in this GenomicStudy.
    pub analysis: Vec<super::genomic_study::GenomicStudyAnalysis>,
    /// Event resources that the genomic study is based on.
    pub based_on: Vec<super::reference::Reference>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Description of the genomic study.
    pub description: super::markdown::Markdown,
    /// The healthcare event with which this genomics study is associated.
    pub encounter: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifiers for this genomic study.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The defined protocol that describes the study.
    pub instantiates_canonical: super::canonical::Canonical,
    /// The URL pointing to an externally maintained protocol that describes the study.
    pub instantiates_uri: super::uri::Uri,
    /// Healthcare professionals who interpreted the genomic study.
    pub interpreter: Vec<super::reference::Reference>,
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
    /// Comments related to the genomic study.
    pub note: Vec<super::annotation::Annotation>,
    /// Why the genomic study was performed.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// Healthcare professional who requested or referred the genomic study.
    pub referrer: super::reference::Reference,
    /// This is a GenomicStudy resource
    pub resource_type: String,
    /// When the genomic study was started.
    pub start_date: super::date_time::DateTime,
    /// The status of the genomic study.
    pub status: super::code::Code,
    /// The primary subject of the genomic study.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// The type of the study, e.g., Familial variant segregation, Functional variation
    /// detection, or Gene expression profiling.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
}

/// A GenomicStudy is a set of analyses performed to analyze and generate genomic
/// data.
#[derive(Debug, Clone, PartialEq)]
pub struct GenomicStudyAnalysis {
    /// Type of the genomic changes studied in the analysis, e.g., DNA, RNA, or amino
    /// acid change.
    pub change_type: Vec<super::codeable_concept::CodeableConcept>,
    /// The date of the analysis event.
    pub date: super::date_time::DateTime,
    /// Devices used for the analysis (e.g., instruments, software), with settings and
    /// parameters.
    pub device: Vec<super::genomic_study::GenomicStudyDevice>,
    /// The focus of a genomic analysis when it is not the patient of record
    /// representing something or someone associated with the patient such
    /// as a spouse, parent, child, or sibling. For example, in trio testing,
    /// the GenomicStudy.subject would be the child (proband) and the
    /// GenomicStudy.analysis.focus of a specific analysis would be the parent.
    pub focus: Vec<super::reference::Reference>,
    /// The reference genome build that is used in this analysis.
    pub genome_build: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Identifiers for the analysis event.
    pub identifier: Vec<super::identifier::Identifier>,
    /// Inputs for the analysis event.
    pub input: Vec<super::genomic_study::GenomicStudyInput>,
    /// The defined protocol that describes the analysis.
    pub instantiates_canonical: super::canonical::Canonical,
    /// The URL pointing to an externally maintained protocol that describes the
    /// analysis.
    pub instantiates_uri: super::uri::Uri,
    /// Type of the methods used in the analysis, e.g., Fluorescence in situ
    /// hybridization (FISH), Karyotyping, or Microsatellite instability testing (MSI).
    pub method_type: Vec<super::codeable_concept::CodeableConcept>,
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
    /// Any notes capture with the analysis event.
    pub note: Vec<super::annotation::Annotation>,
    /// Outputs for the analysis event.
    pub output: Vec<super::genomic_study::GenomicStudyOutput>,
    /// Performer for the analysis event.
    pub performer: Vec<super::genomic_study::GenomicStudyPerformer>,
    /// The protocol that was performed for the analysis event.
    pub protocol_performed: super::reference::Reference,
    /// Genomic regions actually called in the analysis event (BED file).
    pub regions_called: Vec<super::reference::Reference>,
    /// The genomic regions to be studied in the analysis (BED file).
    pub regions_studied: Vec<super::reference::Reference>,
    /// The specimen used in the analysis event.
    pub specimen: Vec<super::reference::Reference>,
    /// Name of the analysis event (human friendly).
    pub title: super::string::String,
}

/// A GenomicStudy is a set of analyses performed to analyze and generate genomic
/// data.
#[derive(Debug, Clone, PartialEq)]
pub struct GenomicStudyDevice {
    /// Device used for the analysis.
    pub device: super::reference::Reference,
    /// Specific function for the device used for the analysis.
    pub function: super::codeable_concept::CodeableConcept,
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

/// A GenomicStudy is a set of analyses performed to analyze and generate genomic
/// data.
#[derive(Debug, Clone, PartialEq)]
pub struct GenomicStudyInput {
    /// File containing input data.
    pub file: super::reference::Reference,
    /// The analysis event or other GenomicStudy that generated this input file.
    pub generated_by_identifier: super::identifier::Identifier,
    /// The analysis event or other GenomicStudy that generated this input file.
    pub generated_by_reference: super::reference::Reference,
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
    /// Type of input data, e.g., BAM, CRAM, or FASTA.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// A GenomicStudy is a set of analyses performed to analyze and generate genomic
/// data.
#[derive(Debug, Clone, PartialEq)]
pub struct GenomicStudyOutput {
    /// File containing output data.
    pub file: super::reference::Reference,
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
    /// Type of output data, e.g., VCF, MAF, or BAM.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// A GenomicStudy is a set of analyses performed to analyze and generate genomic
/// data.
#[derive(Debug, Clone, PartialEq)]
pub struct GenomicStudyPerformer {
    /// The organization, healthcare professional, or others who participated in
    /// performing this analysis.
    pub actor: super::reference::Reference,
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
    /// Role of the actor for this analysis.
    pub role: super::codeable_concept::CodeableConcept,
}
