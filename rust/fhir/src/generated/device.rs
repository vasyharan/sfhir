/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity. The device may be a
/// medical or non-medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct Device {
    /// The availability of the device.
    pub availability_status: super::codeable_concept::CodeableConcept,
    /// An identifier that supports traceability to the event during which material in
    /// this product from one or more biological entities was obtained or pooled.
    pub biological_source_event: super::identifier::Identifier,
    /// Devices may be associated with one or more categories.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// Identifies the standards, specifications, or formal guidances for the
    /// capabilities supported by the device. The device may be certified as conformant
    /// to these specifications e.g., communication, performance, process, measurement,
    /// or specialization standards.
    pub conforms_to: Vec<super::device::DeviceConformsTo>,
    /// Contact details for an organization or a particular human that is responsible
    /// for the device.
    pub contact: Vec<super::contact_point::ContactPoint>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The series of occurrences that repeats during the operation of the device.
    pub cycle: super::count::Count,
    /// The reference to the definition for the device.
    pub definition: super::codeable_reference::CodeableReference,
    /// The name used to display by default when the device is referenced. Based on
    /// intent of use by the resource creator, this may reflect one of the names in
    /// Device.name, or may be another simple name.
    pub display_name: super::string::String,
    /// A measurement of time during the device's operation (e.g., days, hours, mins,
    /// etc.).
    pub duration: super::duration::Duration,
    /// Technical endpoints providing access to services provided by the device defined
    /// at this resource.
    pub endpoint: Vec<super::reference::Reference>,
    /// The date and time beyond which this device is no longer valid or should not be
    /// used (if applicable).
    pub expiration_date: super::date_time::DateTime,
    /// The linked device acting as a communication controller, data collector,
    /// translator, or concentrator for the current device (e.g., mobile phone
    /// application that relays a blood pressure device's data).
    pub gateway: Vec<super::codeable_reference::CodeableReference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Unique instance identifiers assigned to a device by manufacturers other
    /// organizations or owners.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The place where the device can be found.
    pub location: super::reference::Reference,
    /// Lot number assigned by the manufacturer.
    pub lot_number: super::string::String,
    /// The date and time when the device was manufactured.
    pub manufacture_date: super::date_time::DateTime,
    /// A name of the manufacturer or entity legally responsible for the device.
    pub manufacturer: super::string::String,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// The designated condition for performing a task with the device.
    pub mode: super::codeable_concept::CodeableConcept,
    /// The manufacturer's model number for the device.
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
    /// This represents the manufacturer's name of the device as provided by the device,
    /// from a UDI label, or by a person describing the Device.  This typically would be
    /// used when a person provides the name(s) or when the device represents one of the
    /// names available from DeviceDefinition.
    pub name: Vec<super::device::DeviceName>,
    /// Descriptive information, usage information or implantation information that is
    /// not captured in an existing element.
    pub note: Vec<super::annotation::Annotation>,
    /// An organization that is responsible for the provision and ongoing maintenance of
    /// the device.
    pub owner: super::reference::Reference,
    /// The higher level or encompassing device that this device is a logical part of.
    pub parent: super::reference::Reference,
    /// The part number or catalog number of the device.
    pub part_number: super::string::String,
    /// Static or essentially fixed characteristics or features of the device (e.g.,
    /// time or timing attributes, resolution, accuracy, intended use or instructions
    /// for use, and physical attributes) that are not otherwise captured in more
    /// specific attributes.
    pub property: Vec<super::device::DeviceProperty>,
    /// This is a Device resource
    pub resource_type: String,
    /// Provides additional safety characteristics about a medical device.  For example
    /// devices containing latex.
    pub safety: Vec<super::codeable_concept::CodeableConcept>,
    /// The serial number assigned by the organization when the device was manufactured.
    pub serial_number: super::string::String,
    /// The Device record status. This is not the status of the device like
    /// availability.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// The kind or type of device. A device instance may have more than one type -
    /// in which case those are the types that apply to the specific instance of the
    /// device.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
    /// Unique device identifier (UDI) assigned to device label or package.  Note that
    /// the Device may include multiple udiCarriers as it either may include just the
    /// udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it
    /// could have been sold.
    pub udi_carrier: Vec<super::device::DeviceUdiCarrier>,
    /// A network address on which the device may be contacted directly.
    pub url: super::uri::Uri,
    /// The actual design of the device or software version running on the device.
    pub version: Vec<super::device::DeviceVersion>,
}

/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity. The device may be a
/// medical or non-medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceConformsTo {
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
    /// Code that identifies the specific standard, specification, protocol, formal
    /// guidance, regulation, legislation, or certification scheme to which the device
    /// adheres.
    pub specification: super::codeable_concept::CodeableConcept,
    /// Identifies the specific form or variant of the standard, specification, or
    /// formal guidance. This may be a 'version number', release, document edition,
    /// publication year, or other label.
    pub version: super::string::String,
}

/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity. The device may be a
/// medical or non-medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceName {
    /// Indicates the default or preferred name to be displayed.
    pub display: super::boolean::Boolean,
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
    /// Indicates the kind of name. RegisteredName | UserFriendlyName |
    /// PatientReportedName.
    pub r#type: super::code::Code,
    /// The actual name that identifies the device.
    pub value: super::string::String,
}

/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity. The device may be a
/// medical or non-medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceProperty {
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
    /// Code that specifies the property, such as resolution, color, size, being
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

/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity. The device may be a
/// medical or non-medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceUdiCarrier {
    /// The full UDI carrier of the Automatic Identification and Data Capture (AIDC)
    /// technology representation of the barcode string as printed on the packaging of
    /// the device - e.g., a barcode or RFID.   Because of limitations on character sets
    /// in XML and the need to round-trip JSON data through XML, AIDC Formats *SHALL* be
    /// base64 encoded.
    pub carrier_aidc: super::base_64_binary::Base64Binary,
    /// The full UDI carrier as the human readable form (HRF) representation of the
    /// barcode string as printed on the packaging of the device.
    pub carrier_hrf: super::string::String,
    /// The device identifier (DI) is a mandatory, fixed portion of a UDI that
    /// identifies the labeler and the specific version or model of a device.
    pub device_identifier: super::string::String,
    /// A coded entry to indicate how the data was entered.
    pub entry_type: super::code::Code,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Organization that is charged with issuing UDIs for devices. For example, the US
    /// FDA issuers include:
    /// 1) GS1: http://hl7.org/fhir/NamingSystem/gs1-di,
    /// 2) HIBCC: http://hl7.org/fhir/NamingSystem/hibcc-diI,
    /// 3) ICCBBA for blood containers: http://hl7.org/fhir/NamingSystem/iccbba-blood-
    /// di,
    /// 4) ICCBA for other devices: http://hl7.org/fhir/NamingSystem/iccbba-other-di #
    /// Informationsstelle für Arzneispezialitäten (IFA GmbH) (EU only): http://hl7.org/
    /// fhir/NamingSystem/ifa-gmbh-di.
    pub issuer: super::uri::Uri,
    /// The identity of the authoritative source for UDI generation within a
    /// jurisdiction. All UDIs are globally unique within a single namespace with the
    /// appropriate repository uri as the system. For example, UDIs of devices managed
    /// in the U.S. by the FDA, the value is http://hl7.org/fhir/NamingSystem/us-fda-
    /// udi or in the European Union by the European Commission http://hl7.org/fhir/
    /// NamingSystem/eu-ec-udi.
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
}

/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity. The device may be a
/// medical or non-medical device.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceVersion {
    /// The hardware or software module of the device to which the version applies.
    pub component: super::identifier::Identifier,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The date the version was installed on the device.
    pub install_date: super::date_time::DateTime,
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
