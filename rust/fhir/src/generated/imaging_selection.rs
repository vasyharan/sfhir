/// A selection of DICOM SOP instances and/or frames within a single Study and
/// Series. This might include additional specifics such as an image region, an
/// Observation UID or a Segmentation Number, allowing linkage to an Observation
/// Resource or transferring this information along with the ImagingStudy Resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingSelection {
    /// A list of the diagnostic requests that resulted in this imaging selection being
    /// performed.
    pub based_on: Vec<super::reference::Reference>,
    /// The anatomic structures examined. See DICOM Part 16 Annex L (http://
    /// dicom.nema.org/medical/dicom/current/output/chtml/part16/chapter_L.html) for
    /// DICOM to SNOMED-CT mappings.
    pub body_site: super::codeable_reference::CodeableReference,
    /// Classifies the imaging selection.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// Reason for referencing the selected content.
    pub code: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The imaging study from which the imaging selection is made.
    pub derived_from: Vec<super::reference::Reference>,
    /// The network service providing retrieval access to the selected images, frames,
    /// etc. See implementation notes for information about using DICOM endpoints.
    pub endpoint: Vec<super::reference::Reference>,
    /// The actual focus of an observation when it is not the patient of record
    /// representing something or someone associated with the patient such as a
    /// spouse, parent, fetus, or donor. For example, fetus observations in a mother's
    /// record.  The focus of an observation could also be an existing condition,  an
    /// intervention, the subject's diet,  another observation of the subject,  or a
    /// body structure such as tumor or implanted device.   An example use case would
    /// be using the Observation resource to capture whether the mother is trained to
    /// change her child's tracheostomy tube. In this example, the child is the patient
    /// of record and the mother is the focus.
    pub focus: Vec<super::reference::Reference>,
    /// The Frame of Reference UID identifying the coordinate system that conveys
    /// spatial and/or temporal information for the selected images or frames.
    pub frame_of_reference_uid: super::id::Id,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A unique identifier assigned to this imaging selection.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Each imaging selection includes one or more selected DICOM SOP instances.
    pub instance: Vec<super::imaging_selection::ImagingSelectionInstance>,
    /// The date and time this imaging selection was created.
    pub issued: super::instant::Instant,
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
    /// Selector of the instances – human or machine.
    pub performer: Vec<super::imaging_selection::ImagingSelectionPerformer>,
    /// This is a ImagingSelection resource
    pub resource_type: String,
    /// The Series Number for the DICOM Series from which the images were selected.
    pub series_number: super::unsigned_int::UnsignedInt,
    /// The Series Instance UID for the DICOM Series from which the images were
    /// selected.
    pub series_uid: super::id::Id,
    /// The current state of the ImagingSelection resource. This is not the status
    /// of any ImagingStudy, ServiceRequest, or Task resources associated with the
    /// ImagingSelection.
    pub status: super::code::Code,
    /// The Study Instance UID for the DICOM Study from which the images were selected.
    pub study_uid: super::id::Id,
    /// The patient, or group of patients, location, device, organization, procedure or
    /// practitioner this imaging selection is about and into whose or what record the
    /// imaging selection is placed.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A selection of DICOM SOP instances and/or frames within a single Study and
/// Series. This might include additional specifics such as an image region, an
/// Observation UID or a Segmentation Number, allowing linkage to an Observation
/// Resource or transferring this information along with the ImagingStudy Resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingSelectionImageRegion2D {
    /// The coordinates describing the image region. Encoded as a set of (column, row)
    /// pairs that denote positions in the selected image / frames specified with sub-
    /// pixel resolution.
    ///        The origin at the TLHC of the TLHC pixel is 0.0\0.0, the BRHC of the TLHC
    /// pixel is 1.0\1.0, and the BRHC of the BRHC pixel is the number of columns\rows
    /// in the image / frames. The values must be within the range 0\0 to the number of
    /// columns\rows in the image / frames.
    pub coordinate: Vec<super::decimal::Decimal>,
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
    /// Specifies the type of image region.
    pub region_type: super::code::Code,
}

/// A selection of DICOM SOP instances and/or frames within a single Study and
/// Series. This might include additional specifics such as an image region, an
/// Observation UID or a Segmentation Number, allowing linkage to an Observation
/// Resource or transferring this information along with the ImagingStudy Resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingSelectionImageRegion3D {
    /// The coordinates describing the image region. Encoded as an ordered set
    /// of (x,y,z) triplets (in mm and may be negative) that define a region of
    /// interest in the patient-relative Reference Coordinate System defined by
    /// ImagingSelection.frameOfReferenceUid element.
    pub coordinate: Vec<super::decimal::Decimal>,
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
    /// Specifies the type of image region.
    pub region_type: super::code::Code,
}

/// A selection of DICOM SOP instances and/or frames within a single Study and
/// Series. This might include additional specifics such as an image region, an
/// Observation UID or a Segmentation Number, allowing linkage to an Observation
/// Resource or transferring this information along with the ImagingStudy Resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingSelectionInstance {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Each imaging selection instance or frame list might includes an image region,
    /// specified by a region type and a set of 2D coordinates.
    ///        If the parent imagingSelection.instance contains a subset element of type
    /// frame, the image region applies to all frames in the subset list.
    pub image_region_2_d: Vec<super::imaging_selection::ImagingSelectionImageRegion2D>,
    /// Each imaging selection might includes a 3D image region, specified by a region
    /// type and a set of 3D coordinates.
    pub image_region_3_d: Vec<super::imaging_selection::ImagingSelectionImageRegion3D>,
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
    /// The Instance Number for the selected DICOM instance.
    pub number: super::unsigned_int::UnsignedInt,
    /// The SOP Class UID for the selected DICOM instance.
    pub sop_class: super::coding::Coding,
    /// Selected subset of the SOP Instance. The content and format of the subset item
    /// is determined by the SOP Class of the selected instance.
    ///        May be one of:
    ///        - A list of frame numbers selected from a multiframe SOP Instance.
    ///        - A list of Content Item Observation UID values selected from a DICOM SR
    /// or other structured document SOP Instance.
    ///        - A list of segment numbers selected from a segmentation SOP Instance.
    ///        - A list of Region of Interest (ROI) numbers selected from a radiotherapy
    /// structure set SOP Instance.
    pub subset: Vec<super::string::String>,
    /// The SOP Instance UID for the selected DICOM instance.
    pub uid: super::id::Id,
}

/// A selection of DICOM SOP instances and/or frames within a single Study and
/// Series. This might include additional specifics such as an image region, an
/// Observation UID or a Segmentation Number, allowing linkage to an Observation
/// Resource or transferring this information along with the ImagingStudy Resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ImagingSelectionPerformer {
    /// Author – human or machine.
    pub actor: super::reference::Reference,
    /// Distinguishes the type of involvement of the performer.
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
