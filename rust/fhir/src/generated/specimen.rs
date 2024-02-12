/// A sample to be used for analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct Specimen {
    /// The identifier assigned by the lab when accessioning specimen(s). This is
    /// not necessarily the same as the specimen identifier, depending on local lab
    /// procedures.
    pub accession_identifier: super::identifier::Identifier,
    /// Details concerning the specimen collection.
    pub collection: super::specimen::SpecimenCollection,
    /// This element signifies if the specimen is part of a group or pooled.
    pub combined: super::code::Code,
    /// A mode or state of being that describes the nature of the specimen.
    pub condition: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The container holding the specimen.  The recursive nature of containers; i.e.
    /// blood in tube in tray in rack is not addressed here.
    pub container: Vec<super::specimen::SpecimenContainer>,
    /// A physical feature or landmark on a specimen, highlighted for context by the
    /// collector of the specimen (e.g. surgeon), that identifies the type of feature
    /// as well as its meaning (e.g. the red ink indicating the resection margin of
    /// the right lobe of the excised prostate tissue or wire loop at radiologically
    /// suspected tumor location).
    pub feature: Vec<super::specimen::SpecimenFeature>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Id for specimen.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
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
    /// To communicate any details or issues about the specimen or during the specimen
    /// collection. (for example: broken vial, sent with patient, frozen).
    pub note: Vec<super::annotation::Annotation>,
    /// Reference to the parent (source) specimen which is used when the specimen was
    /// either derived from or a component of another specimen.
    pub parent: Vec<super::reference::Reference>,
    /// Details concerning processing and processing steps for the specimen.
    pub processing: Vec<super::specimen::SpecimenProcessing>,
    /// Time when specimen is received by the testing laboratory for processing or
    /// testing.
    pub received_time: super::date_time::DateTime,
    /// Details concerning a service request that required a specimen to be collected.
    pub request: Vec<super::reference::Reference>,
    /// This is a Specimen resource
    pub resource_type: String,
    /// The role or reason for the specimen in the testing workflow.
    pub role: Vec<super::codeable_concept::CodeableConcept>,
    /// The availability of the specimen.
    pub status: super::code::Code,
    /// Where the specimen came from. This may be from patient(s), from a location
    /// (e.g., the source of an environmental sample), or a sampling of a substance, a
    /// biologically-derived product, or a device.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// The kind of material that forms the specimen.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// A sample to be used for analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenCollection {
    /// Anatomical location from which the specimen was collected (if subject is a
    /// patient). This is the target site.  This element is not used for environmental
    /// specimens.
    pub body_site: super::codeable_reference::CodeableReference,
    /// Time when specimen was collected from subject - the physiologically relevant
    /// time.
    pub collected_date_time: String,
    /// Time when specimen was collected from subject - the physiologically relevant
    /// time.
    pub collected_period: super::period::Period,
    /// Person who collected the specimen.
    pub collector: super::reference::Reference,
    /// A coded value specifying the technique that is used to perform the procedure.
    pub device: super::codeable_reference::CodeableReference,
    /// The span of time over which the collection of a specimen occurred.
    pub duration: super::duration::Duration,
    /// Abstinence or reduction from some or all food, drink, or both, for a period of
    /// time prior to sample collection.
    pub fasting_status_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Abstinence or reduction from some or all food, drink, or both, for a period of
    /// time prior to sample collection.
    pub fasting_status_duration: super::duration::Duration,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A coded value specifying the technique that is used to perform the procedure.
    pub method: super::codeable_concept::CodeableConcept,
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
    /// The procedure event during which the specimen was collected (e.g. the surgery
    /// leading to the collection of a pathology sample).
    pub procedure: super::reference::Reference,
    /// The quantity of specimen collected; for instance the volume of a blood sample,
    /// or the physical measurement of an anatomic pathology sample.
    pub quantity: super::quantity::Quantity,
}

/// A sample to be used for analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenContainer {
    /// The device resource for the the container holding the specimen. If the container
    /// is in a holder then the referenced device will point to a parent device.
    pub device: super::reference::Reference,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The location of the container holding the specimen.
    pub location: super::reference::Reference,
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
    /// The quantity of specimen in the container; may be volume, dimensions, or other
    /// appropriate measurements, depending on the specimen type.
    pub specimen_quantity: super::quantity::Quantity,
}

/// A sample to be used for analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenFeature {
    /// Description of the feature of the specimen.
    pub description: super::string::String,
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
    /// The landmark or feature being highlighted.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// A sample to be used for analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenProcessing {
    /// Material used in the processing step.
    pub additive: Vec<super::reference::Reference>,
    /// Textual description of procedure.
    pub description: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A coded value specifying the method used to process the specimen.
    pub method: super::codeable_concept::CodeableConcept,
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
    /// A record of the time or period when the specimen processing occurred.  For
    /// example the time of sample fixation or the period of time the sample was in
    /// formalin.
    pub time_date_time: String,
    /// A record of the time or period when the specimen processing occurred.  For
    /// example the time of sample fixation or the period of time the sample was in
    /// formalin.
    pub time_period: super::period::Period,
}
