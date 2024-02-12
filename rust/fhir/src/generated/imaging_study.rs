/// Representation of the content produced in a DICOM imaging study. A study
/// comprises a set of series, each of which includes a set of Service-Object
/// Pair Instances (SOP Instances - images or other data) acquired or produced
/// in a common context.  A series is of only one modality (e.g. X-ray, CT, MR,
/// ultrasound), but a study may have multiple series of different modalities.
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingStudy {
    /// A list of the diagnostic requests that resulted in this imaging study being
    /// performed.
    pub based_on: Vec<super::reference::Reference>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The Imaging Manager description of the study. Institution-generated description
    /// or classification of the Study (component) performed.
    pub description: super::string::String,
    /// The healthcare event (e.g. a patient and healthcare provider interaction) during
    /// which this ImagingStudy is made.
    pub encounter: super::reference::Reference,
    /// The network service providing access (e.g., query, view, or retrieval) for the
    /// study. See implementation notes for information about using DICOM endpoints. A
    /// study-level endpoint applies to each series in the study, unless overridden by a
    /// series-level endpoint with the same Endpoint.connectionType.
    pub endpoint: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifiers for the ImagingStudy such as DICOM Study Instance UID.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The principal physical location where the ImagingStudy was performed.
    pub location: super::reference::Reference,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// A list of all the distinct values of series.modality. This may include both
    /// acquisition and non-acquisition modalities.
    pub modality: Vec<super::codeable_concept::CodeableConcept>,
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
    /// Per the recommended DICOM mapping, this element is derived from the Study
    /// Description attribute (0008,1030). Observations or findings about the imaging
    /// study should be recorded in another resource, e.g. Observation, and not in this
    /// element.
    pub note: Vec<super::annotation::Annotation>,
    /// Number of SOP Instances in Study. This value given may be larger than the
    /// number of instance elements this resource contains due to resource availability,
    /// security, or other factors. This element should be present if any instance
    /// elements are present.
    pub number_of_instances: super::unsigned_int::UnsignedInt,
    /// Number of Series in the Study. This value given may be larger than the number of
    /// series elements this Resource contains due to resource availability, security,
    /// or other factors. This element should be present if any series elements are
    /// present.
    pub number_of_series: super::unsigned_int::UnsignedInt,
    /// A larger event of which this particular ImagingStudy is a component or step.
    /// For example,  an ImagingStudy as part of a procedure.
    pub part_of: Vec<super::reference::Reference>,
    /// This field corresponds to the DICOM Procedure Code Sequence (0008,1032). This is
    /// different from the FHIR Procedure resource that may include the ImagingStudy.
    pub procedure: Vec<super::codeable_reference::CodeableReference>,
    /// Description of clinical condition indicating why the ImagingStudy was requested,
    /// and/or Indicates another resource whose existence justifies this Study.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// The requesting/referring physician.
    pub referrer: super::reference::Reference,
    /// This is a ImagingStudy resource
    pub resource_type: String,
    /// Each study has one or more series of images or other content.
    pub series: Vec<super::imaging_study::ImagingStudySeries>,
    /// Date and time the study started.
    pub started: super::date_time::DateTime,
    /// The current state of the ImagingStudy resource. This is not the status of any
    /// ServiceRequest or Task resources associated with the ImagingStudy.
    pub status: super::code::Code,
    /// The subject, typically a patient, of the imaging study.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// Representation of the content produced in a DICOM imaging study. A study
/// comprises a set of series, each of which includes a set of Service-Object
/// Pair Instances (SOP Instances - images or other data) acquired or produced
/// in a common context.  A series is of only one modality (e.g. X-ray, CT, MR,
/// ultrasound), but a study may have multiple series of different modalities.
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingStudyInstance {
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
    /// The number of instance in the series.
    pub number: super::unsigned_int::UnsignedInt,
    /// DICOM instance  type.
    pub sop_class: super::coding::Coding,
    /// The description of the instance.
    pub title: super::string::String,
    /// The DICOM SOP Instance UID for this image or other DICOM content.
    pub uid: super::id::Id,
}

/// Representation of the content produced in a DICOM imaging study. A study
/// comprises a set of series, each of which includes a set of Service-Object
/// Pair Instances (SOP Instances - images or other data) acquired or produced
/// in a common context.  A series is of only one modality (e.g. X-ray, CT, MR,
/// ultrasound), but a study may have multiple series of different modalities.
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingStudyPerformer {
    /// Indicates who or what performed the series.
    pub actor: super::reference::Reference,
    /// Distinguishes the type of involvement of the performer in the series.
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

/// Representation of the content produced in a DICOM imaging study. A study
/// comprises a set of series, each of which includes a set of Service-Object
/// Pair Instances (SOP Instances - images or other data) acquired or produced
/// in a common context.  A series is of only one modality (e.g. X-ray, CT, MR,
/// ultrasound), but a study may have multiple series of different modalities.
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingStudySeries {
    /// The anatomic structures examined. See DICOM Part 16 Annex L (http://
    /// dicom.nema.org/medical/dicom/current/output/chtml/part16/chapter_L.html)
    /// for DICOM to SNOMED-CT mappings. The bodySite may indicate the laterality
    /// of body part imaged; if so, it shall be consistent with any content of
    /// ImagingStudy.series.laterality.
    pub body_site: super::codeable_reference::CodeableReference,
    /// A description of the series.
    pub description: super::string::String,
    /// The network service providing access (e.g., query, view, or retrieval) for this
    /// series. See implementation notes for information about using DICOM endpoints.
    /// A series-level endpoint, if present, has precedence over a study-level endpoint
    /// with the same Endpoint.connectionType.
    pub endpoint: Vec<super::reference::Reference>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A single SOP instance within the series, e.g. an image, or presentation state.
    pub instance: Vec<super::imaging_study::ImagingStudyInstance>,
    /// The laterality of the (possibly paired) anatomic structures examined. E.g., the
    /// left knee, both lungs, or unpaired abdomen. If present, shall be consistent with
    /// any laterality information indicated in ImagingStudy.series.bodySite.
    pub laterality: super::codeable_concept::CodeableConcept,
    /// The distinct modality for this series. This may include both acquisition and
    /// non-acquisition modalities.
    pub modality: super::codeable_concept::CodeableConcept,
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
    /// The numeric identifier of this series in the study.
    pub number: super::unsigned_int::UnsignedInt,
    /// Number of SOP Instances in the Study. The value given may be larger than the
    /// number of instance elements this resource contains due to resource availability,
    /// security, or other factors. This element should be present if any instance
    /// elements are present.
    pub number_of_instances: super::unsigned_int::UnsignedInt,
    /// Indicates who or what performed the series and how they were involved.
    pub performer: Vec<super::imaging_study::ImagingStudyPerformer>,
    /// The specimen imaged, e.g., for whole slide imaging of a biopsy.
    pub specimen: Vec<super::reference::Reference>,
    /// The date and time the series was started.
    pub started: super::date_time::DateTime,
    /// The DICOM Series Instance UID for the series.
    pub uid: super::id::Id,
}
