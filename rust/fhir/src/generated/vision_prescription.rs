/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.
#[derive(Debug, Clone, PartialEq)]
pub struct VisionPrescription {
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The date this resource was created.
    pub created: super::date_time::DateTime,
    /// The date (and perhaps time) when the prescription was written.
    pub date_written: super::date_time::DateTime,
    /// A reference to a resource that identifies the particular occurrence of contact
    /// between patient and health care provider during which the prescription was
    /// issued.
    pub encounter: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A unique identifier assigned to this vision prescription.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Contain the details of  the individual lens specifications and serves as the
    /// authorization for the fullfillment by certified professionals.
    pub lens_specification: Vec<super::vision_prescription::VisionPrescriptionLensSpecification>,
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
    /// A resource reference to the person to whom the vision prescription applies.
    pub patient: super::reference::Reference,
    /// The healthcare professional responsible for authorizing the prescription.
    pub prescriber: super::reference::Reference,
    /// This is a VisionPrescription resource
    pub resource_type: String,
    /// The status of the resource instance.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.
#[derive(Debug, Clone, PartialEq)]
pub struct VisionPrescriptionLensSpecification {
    /// Power adjustment for multifocal lenses measured in dioptres (0.25 units).
    pub add: super::decimal::Decimal,
    /// Adjustment for astigmatism measured in integer degrees.
    pub axis: super::integer::Integer,
    /// Back curvature measured in millimetres.
    pub back_curve: super::decimal::Decimal,
    /// Brand recommendations or restrictions.
    pub brand: super::string::String,
    /// Special color or pattern.
    pub color: super::string::String,
    /// Power adjustment for astigmatism measured in dioptres (0.25 units).
    pub cylinder: super::decimal::Decimal,
    /// Contact lens diameter measured in millimetres.
    pub diameter: super::decimal::Decimal,
    /// The recommended maximum wear period for the lens.
    pub duration: super::quantity::Quantity,
    /// The eye for which the lens specification applies.
    pub eye: super::code::Code,
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
    /// Notes for special requirements such as coatings and lens materials.
    pub note: Vec<super::annotation::Annotation>,
    /// Contact lens power measured in dioptres (0.25 units).
    pub power: super::decimal::Decimal,
    /// Allows for adjustment on two axis.
    pub prism: Vec<super::vision_prescription::VisionPrescriptionPrism>,
    /// Identifies the type of vision correction product which is required for the
    /// patient.
    pub product: super::codeable_concept::CodeableConcept,
    /// Lens power measured in dioptres (0.25 units).
    pub sphere: super::decimal::Decimal,
}

/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.
#[derive(Debug, Clone, PartialEq)]
pub struct VisionPrescriptionPrism {
    /// Amount of prism to compensate for eye alignment in fractional units.
    pub amount: super::decimal::Decimal,
    /// The relative base, or reference lens edge, for the prism.
    pub base: super::code::Code,
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
