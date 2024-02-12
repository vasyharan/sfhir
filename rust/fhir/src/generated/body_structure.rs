/// Record details about an anatomical structure.  This resource may be used when a
/// coded concept does not provide the necessary detail needed for the use case.
#[derive(Debug, Clone, PartialEq)]
pub struct BodyStructure {
    /// Whether this body site is in active use.
    pub active: super::boolean::Boolean,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A summary, characterization or explanation of the body structure.
    pub description: super::markdown::Markdown,
    /// The anatomical location(s) or region(s) not occupied or represented by the
    /// specimen, lesion, or body structure.
    pub excluded_structure: Vec<super::body_structure::BodyStructureIncludedStructure>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifier for this instance of the anatomical structure.
    pub identifier: Vec<super::identifier::Identifier>,
    /// Image or images used to identify a location.
    pub image: Vec<super::attachment::Attachment>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The anatomical location(s) or region(s) of the specimen, lesion, or body
    /// structure.
    pub included_structure: Vec<super::body_structure::BodyStructureIncludedStructure>,
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
    /// The kind of structure being represented by the body structure at
    /// `BodyStructure.location`.  This can define both normal and abnormal
    /// morphologies.
    pub morphology: super::codeable_concept::CodeableConcept,
    /// The person to which the body site belongs.
    pub patient: super::reference::Reference,
    /// This is a BodyStructure resource
    pub resource_type: String,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// Record details about an anatomical structure.  This resource may be used when a
/// coded concept does not provide the necessary detail needed for the use case.
#[derive(Debug, Clone, PartialEq)]
pub struct BodyStructureBodyLandmarkOrientation {
    /// An description of the direction away from a landmark something is located based
    /// on a radial clock dial.
    pub clock_face_position: Vec<super::codeable_concept::CodeableConcept>,
    /// The distance in centimeters a certain observation is made from a body landmark.
    pub distance_from_landmark: Vec<super::body_structure::BodyStructureDistanceFromLandmark>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A description of a landmark on the body used as a reference to locate something
    /// else.
    pub landmark_description: Vec<super::codeable_concept::CodeableConcept>,
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
    /// The surface area a body location is in relation to a landmark.
    pub surface_orientation: Vec<super::codeable_concept::CodeableConcept>,
}

/// Record details about an anatomical structure.  This resource may be used when a
/// coded concept does not provide the necessary detail needed for the use case.
#[derive(Debug, Clone, PartialEq)]
pub struct BodyStructureDistanceFromLandmark {
    /// An instrument, tool, analyzer, etc. used in the measurement.
    pub device: Vec<super::codeable_reference::CodeableReference>,
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
    /// The measured distance (e.g., in cm) from a body landmark.
    pub value: Vec<super::quantity::Quantity>,
}

/// Record details about an anatomical structure.  This resource may be used when a
/// coded concept does not provide the necessary detail needed for the use case.
#[derive(Debug, Clone, PartialEq)]
pub struct BodyStructureIncludedStructure {
    /// Body locations in relation to a specific body landmark (tatoo, scar, other body
    /// structure).
    pub body_landmark_orientation: Vec<super::body_structure::BodyStructureBodyLandmarkOrientation>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Code that represents the included structure laterality.
    pub laterality: super::codeable_concept::CodeableConcept,
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
    /// Code that represents the included structure qualifier.
    pub qualifier: Vec<super::codeable_concept::CodeableConcept>,
    /// XY or XYZ-coordinate orientation for structure.
    pub spatial_reference: Vec<super::reference::Reference>,
    /// Code that represents the included structure.
    pub structure: super::codeable_concept::CodeableConcept,
}
