/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinition {
    /// Billing code or reference associated with the device.
    pub charge_item: Vec<super::device_definition::DeviceDefinitionChargeItem>,
    /// What kind of device or device system this is.
    pub classification: Vec<super::device_definition::DeviceDefinitionClassification>,
    /// Identifies the standards, specifications, or formal guidances for the
    /// capabilities supported by the device. The device may be certified as conformant
    /// to these specifications e.g., communication, performance, process, measurement,
    /// or specialization standards.
    pub conforms_to: Vec<super::device_definition::DeviceDefinitionConformsTo>,
    /// Contact details for an organization or a particular human that is responsible
    /// for the device.
    pub contact: Vec<super::contact_point::ContactPoint>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Tracking of latest field safety corrective action.
    pub corrective_action: super::device_definition::DeviceDefinitionCorrectiveAction,
    /// Additional information to describe the device.
    pub description: super::markdown::Markdown,
    /// The name or names of the device as given by the manufacturer.
    pub device_name: Vec<super::device_definition::DeviceDefinitionDeviceName>,
    /// Information aimed at providing directions for the usage of this model of device.
    pub guideline: super::device_definition::DeviceDefinitionGuideline,
    /// A device that is part (for example a component) of the present device.
    pub has_part: Vec<super::device_definition::DeviceDefinitionHasPart>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Unique instance identifiers assigned to a device by the software, manufacturers,
    /// other organizations or owners. For example: handle ID. The identifier is
    /// typically valued if the udiDeviceIdentifier, partNumber or modelNumber is
    /// not valued and represents a different type of identifier.  However, it is
    /// permissible to still include those identifiers in DeviceDefinition.identifier
    /// with the appropriate identifier.type.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Language code for the human-readable text strings produced by the device (all
    /// supported).
    pub language_code: Vec<super::codeable_concept::CodeableConcept>,
    /// An associated device, attached to, used with, communicating with or linking a
    /// previous or new device model to the focal device.
    pub link: Vec<super::device_definition::DeviceDefinitionLink>,
    /// A name of the manufacturer  or legal representative e.g. labeler. Whether
    /// this is the actual manufacturer or the labeler or responsible depends on
    /// implementation and jurisdiction.
    pub manufacturer: super::reference::Reference,
    /// A substance used to create the material(s) of which the device is made.
    pub material: Vec<super::device_definition::DeviceDefinitionMaterial>,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// The model number for the device for example as defined by the manufacturer or
    /// labeler, or other agency.
    pub model_number: super::string::String,
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
    /// Descriptive information, usage information or implantation information that is
    /// not captured in an existing element.
    pub note: Vec<super::annotation::Annotation>,
    /// An organization that is responsible for the provision and ongoing maintenance of
    /// the device.
    pub owner: super::reference::Reference,
    /// Information about the packaging of the device, i.e. how the device is packaged.
    pub packaging: Vec<super::device_definition::DeviceDefinitionPackaging>,
    /// The part number or catalog number of the device.
    pub part_number: super::string::String,
    /// Indicates the production identifier(s) that are expected to appear in the UDI
    /// carrier on the device label.
    pub production_identifier_in_udi: Vec<super::code::Code>,
    /// Static or essentially fixed characteristics or features of this kind of device
    /// that are otherwise not captured in more specific attributes, e.g., time or
    /// timing attributes, resolution, accuracy, and physical attributes.
    pub property: Vec<super::device_definition::DeviceDefinitionProperty>,
    /// Identifier associated with the regulatory documentation (certificates, technical
    /// documentation, post-market surveillance documentation and reports) of a set of
    /// device models sharing the same intended purpose, risk class and essential design
    /// and manufacturing characteristics. One example is the Basic UDI-DI in Europe.
    pub regulatory_identifier: Vec<super::device_definition::DeviceDefinitionRegulatoryIdentifier>,
    /// This is a DeviceDefinition resource
    pub resource_type: String,
    /// Safety characteristics of the device.
    pub safety: Vec<super::codeable_concept::CodeableConcept>,
    /// Shelf Life and storage information.
    pub shelf_life_storage: Vec<super::product_shelf_life::ProductShelfLife>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Unique device identifier (UDI) assigned to device label or package.  Note that
    /// the Device may include multiple udiCarriers as it either may include just the
    /// udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it
    /// could have been sold.
    pub udi_device_identifier: Vec<super::device_definition::DeviceDefinitionUdiDeviceIdentifier>,
    /// The version of the device or software.
    pub version: Vec<super::device_definition::DeviceDefinitionVersion>,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionChargeItem {
    /// The code or reference for the charge item.
    pub charge_item_code: super::codeable_reference::CodeableReference,
    /// Coefficient applicable to the billing code.
    pub count: super::quantity::Quantity,
    /// A specific time period in which this charge item applies.
    pub effective_period: super::period::Period,
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
    /// The context to which this charge item applies.
    pub use_context: Vec<super::usage_context::UsageContext>,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionClassification {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Further information qualifying this classification of the device model.
    pub justification: Vec<super::related_artifact::RelatedArtifact>,
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
    /// A classification or risk class of the device model.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionConformsTo {
    /// Describes the type of the standard, specification, or formal guidance.
    pub category: super::codeable_concept::CodeableConcept,
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
    /// Standard, regulation, certification, or guidance website, document, or other
    /// publication, or similar, supporting the conformance.
    pub source: Vec<super::related_artifact::RelatedArtifact>,
    /// Code that identifies the specific standard, specification, protocol, formal
    /// guidance, regulation, legislation, or certification scheme to which the device
    /// adheres.
    pub specification: super::codeable_concept::CodeableConcept,
    /// Identifies the specific form or variant of the standard, specification, or
    /// formal guidance. This may be a 'version number', release, document edition,
    /// publication year, or other label.
    pub version: Vec<super::string::String>,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionCorrectiveAction {
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
    /// Start and end dates of the  corrective action.
    pub period: super::period::Period,
    /// Whether the last corrective action known for this device was a recall.
    pub recall: super::boolean::Boolean,
    /// The scope of the corrective action - whether the action targeted all units of a
    /// given device model, or only a specific set of batches identified by lot numbers,
    /// or individually identified devices identified by the serial name.
    pub scope: super::code::Code,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionDeviceName {
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
    /// A human-friendly name that is used to refer to the device - depending on the
    /// type, it can be the brand name, the common name or alias, or other.
    pub name: super::string::String,
    /// The type of deviceName.
    /// RegisteredName | UserFriendlyName | PatientReportedName.
    pub r#type: super::code::Code,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionDistributor {
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
    /// Distributor's human-readable name.
    pub name: super::string::String,
    /// Distributor as an Organization resource.
    pub organization_reference: Vec<super::reference::Reference>,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionGuideline {
    /// A specific situation when a device should not be used because it may cause harm.
    pub contraindication: Vec<super::codeable_concept::CodeableConcept>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A clinical condition for which the device was designed to be used.
    pub indication: Vec<super::codeable_concept::CodeableConcept>,
    /// A description of the general purpose or medical use of the device or its
    /// function.
    pub intended_use: super::string::String,
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
    /// A source of information or reference for this guideline.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// Detailed written and visual directions for the user on how to use the device.
    pub usage_instruction: super::markdown::Markdown,
    /// The circumstances that form the setting for using the device.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// Specific hazard alert information that a user needs to know before using the
    /// device.
    pub warning: Vec<super::codeable_concept::CodeableConcept>,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionHasPart {
    /// Number of instances of the component device in the current device.
    pub count: super::integer::Integer,
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
    /// Reference to the device that is part of the current device.
    pub reference: super::reference::Reference,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionLink {
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
    /// A reference to the linked device.
    pub related_device: super::codeable_reference::CodeableReference,
    /// The type indicates the relationship of the related device to the device
    /// instance.
    pub relation: super::coding::Coding,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionMarketDistribution {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Begin and end dates for the commercial distribution of the device.
    pub market_period: super::period::Period,
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
    /// National state or territory to which the marketDistribution recers, typically
    /// where the device is commercialized.
    pub sub_jurisdiction: super::uri::Uri,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionMaterial {
    /// Whether the substance is a known or suspected allergen.
    pub allergenic_indicator: super::boolean::Boolean,
    /// Indicates an alternative material of the device.
    pub alternate: super::boolean::Boolean,
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
    /// A substance that the device contains, may contain, or is made of - for example
    /// latex - to be used to determine patient compatibility. This is not intended to
    /// represent the composition of the device, only the clinically relevant materials.
    pub substance: super::codeable_concept::CodeableConcept,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionPackaging {
    /// The number of items contained in the package (devices or sub-packages).
    pub count: super::integer::Integer,
    /// An organization that distributes the packaged device.
    pub distributor: Vec<super::device_definition::DeviceDefinitionDistributor>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The business identifier of the packaged medication.
    pub identifier: super::identifier::Identifier,
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
    /// Allows packages within packages.
    pub packaging: Vec<super::device_definition::DeviceDefinitionPackaging>,
    /// A code that defines the specific type of packaging.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// Unique Device Identifier (UDI) Barcode string on the packaging.
    pub udi_device_identifier: Vec<super::device_definition::DeviceDefinitionUdiDeviceIdentifier>,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionProperty {
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
    /// Code that specifies the property such as a resolution or color being
    /// represented.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The value of the property specified by the associated property.type code.
    pub value_attachment: super::attachment::Attachment,
    /// The value of the property specified by the associated property.type code.
    pub value_boolean: bool,
    /// The value of the property specified by the associated property.type code.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The value of the property specified by the associated property.type code.
    pub value_integer: i64,
    /// The value of the property specified by the associated property.type code.
    pub value_quantity: super::quantity::Quantity,
    /// The value of the property specified by the associated property.type code.
    pub value_range: super::range::Range,
    /// The value of the property specified by the associated property.type code.
    pub value_string: String,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionRegulatoryIdentifier {
    /// The identifier itself.
    pub device_identifier: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The organization that issued this identifier.
    pub issuer: super::uri::Uri,
    /// The jurisdiction to which the deviceIdentifier applies.
    pub jurisdiction: super::uri::Uri,
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
    /// The type of identifier itself.
    pub r#type: super::code::Code,
}

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionUdiDeviceIdentifier {
    /// The identifier that is to be associated with every Device that references
    /// this DeviceDefintiion for the issuer and jurisdiction provided in the
    /// DeviceDefinition.udiDeviceIdentifier.
    pub device_identifier: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The organization that assigns the identifier algorithm.
    pub issuer: super::uri::Uri,
    /// The jurisdiction to which the deviceIdentifier applies.
    pub jurisdiction: super::uri::Uri,
    /// Indicates where and when the device is available on the market.
    pub market_distribution: Vec<super::device_definition::DeviceDefinitionMarketDistribution>,
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

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDefinitionVersion {
    /// The hardware or software module of the device to which the version applies.
    pub component: super::identifier::Identifier,
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
    /// The type of the device version, e.g. manufacturer, approved, internal.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The version text.
    pub value: super::string::String,
}
