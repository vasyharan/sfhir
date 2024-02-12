/// The findings and interpretation of diagnostic tests performed on patients,
/// groups of patients, products, substances, devices, and locations, and/or
/// specimens derived from these. The report includes clinical context such as
/// requesting provider information, and some mix of atomic results, images, textual
/// and coded interpretations, and formatted representation of diagnostic reports.
/// The report also includes non-clinical context such as batch analysis and
/// stability reporting of products and substances.
#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticReport {
    /// Details concerning a service requested.
    pub based_on: Vec<super::reference::Reference>,
    /// A code that classifies the clinical discipline, department or diagnostic service
    /// that created the report (e.g. cardiology, biochemistry, hematology, MRI). This
    /// is used for searching, sorting and display purposes.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// A code or name that describes this diagnostic report.
    pub code: super::codeable_concept::CodeableConcept,
    /// Reference to a Composition resource instance that provides structure for
    /// organizing the contents of the DiagnosticReport.
    pub composition: super::reference::Reference,
    /// Concise and clinically contextualized summary conclusion (interpretation/
    /// impression) of the diagnostic report.
    pub conclusion: super::markdown::Markdown,
    /// One or more codes that represent the summary conclusion (interpretation/
    /// impression) of the diagnostic report.
    pub conclusion_code: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The time or time-period the observed values are related to. When the subject of
    /// the report is a patient, this is usually either the time of the procedure or of
    /// specimen collection(s), but very often the source of the date/time is not known,
    /// only the date/time itself.
    pub effective_date_time: String,
    /// The time or time-period the observed values are related to. When the subject of
    /// the report is a patient, this is usually either the time of the procedure or of
    /// specimen collection(s), but very often the source of the date/time is not known,
    /// only the date/time itself.
    pub effective_period: super::period::Period,
    /// The healthcare event  (e.g. a patient and healthcare provider interaction) which
    /// this DiagnosticReport is about.
    pub encounter: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifiers assigned to this report by the performer or other systems.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The date and time that this version of the report was made available to
    /// providers, typically after the report was reviewed and verified.
    pub issued: super::instant::Instant,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// A list of key images or data associated with this report. The images or data
    /// are generally created during the diagnostic process, and may be directly of the
    /// patient, or of treated specimens (i.e. slides of interest).
    pub media: Vec<super::diagnostic_report::DiagnosticReportMedia>,
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
    /// Comments about the diagnostic report.
    pub note: Vec<super::annotation::Annotation>,
    /// The diagnostic service that is responsible for issuing the report.
    pub performer: Vec<super::reference::Reference>,
    /// Rich text representation of the entire result as issued by the diagnostic
    /// service. Multiple formats are allowed but they SHALL be semantically equivalent.
    pub presented_form: Vec<super::attachment::Attachment>,
    /// This is a DiagnosticReport resource
    pub resource_type: String,
    /// [Observations](observation.html)  that are part of this diagnostic report.
    pub result: Vec<super::reference::Reference>,
    /// The practitioner or organization that is responsible for the report's
    /// conclusions and interpretations.
    pub results_interpreter: Vec<super::reference::Reference>,
    /// Details about the specimens on which this diagnostic report is based.
    pub specimen: Vec<super::reference::Reference>,
    /// The status of the diagnostic report.
    pub status: super::code::Code,
    /// One or more links to full details of any study performed during the diagnostic
    /// investigation. An ImagingStudy might comprise a set of radiologic images
    /// obtained via a procedure that are analyzed as a group. Typically, this is
    /// imaging performed by DICOM enabled modalities, but this is not required. A fully
    /// enabled PACS viewer can use this information to provide views of the source
    /// images. A GenomicStudy might comprise one or more analyses, each serving a
    /// specific purpose. These analyses may vary in method (e.g., karyotyping, CNV, or
    /// SNV detection), performer, software, devices used, or regions targeted.
    pub study: Vec<super::reference::Reference>,
    /// The subject of the report. Usually, but not always, this is a patient. However,
    /// diagnostic services also perform analyses on specimens collected from a variety
    /// of other sources.
    pub subject: super::reference::Reference,
    /// This backbone element contains supporting information that was used in the
    /// creation of the report not included in the results already included in the
    /// report.
    pub supporting_info: Vec<super::diagnostic_report::DiagnosticReportSupportingInfo>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// The findings and interpretation of diagnostic tests performed on patients,
/// groups of patients, products, substances, devices, and locations, and/or
/// specimens derived from these. The report includes clinical context such as
/// requesting provider information, and some mix of atomic results, images, textual
/// and coded interpretations, and formatted representation of diagnostic reports.
/// The report also includes non-clinical context such as batch analysis and
/// stability reporting of products and substances.
#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticReportMedia {
    /// A comment about the image or data. Typically, this is used to provide an
    /// explanation for why the image or data is included, or to draw the viewer's
    /// attention to important features.
    pub comment: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Reference to the image or data source.
    pub link: super::reference::Reference,
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

/// The findings and interpretation of diagnostic tests performed on patients,
/// groups of patients, products, substances, devices, and locations, and/or
/// specimens derived from these. The report includes clinical context such as
/// requesting provider information, and some mix of atomic results, images, textual
/// and coded interpretations, and formatted representation of diagnostic reports.
/// The report also includes non-clinical context such as batch analysis and
/// stability reporting of products and substances.
#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticReportSupportingInfo {
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
    /// The reference for the supporting information in the diagnostic report.
    pub reference: super::reference::Reference,
    /// The code value for the role of the supporting information in the diagnostic
    /// report.
    pub r#type: super::codeable_concept::CodeableConcept,
}
