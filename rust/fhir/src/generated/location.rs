/// Details and position information for a place where services are provided and
/// resources and participants may be stored, found, contained, or accommodated.
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    /// Physical location.
    pub address: super::address::Address,
    /// A list of alternate names that the location is known as, or was known as, in
    /// the past.
    pub alias: Vec<super::string::String>,
    /// Collection of characteristics (attributes).
    pub characteristic: Vec<super::codeable_concept::CodeableConcept>,
    /// The contact details of communication devices available at the location. This can
    /// include addresses, phone numbers, fax numbers, mobile numbers, email addresses
    /// and web sites.
    pub contact: Vec<super::extended_contact_detail::ExtendedContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Description of the Location, which helps in finding or referencing the place.
    pub description: super::markdown::Markdown,
    /// Technical endpoints providing access to services operated for the location.
    pub endpoint: Vec<super::reference::Reference>,
    /// Physical form of the location, e.g. building, room, vehicle, road, virtual.
    pub form: super::codeable_concept::CodeableConcept,
    /// What days/times during a week is this location usually open, and any exceptions
    /// where the location is not available.
    pub hours_of_operation: Vec<super::availability::Availability>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Unique code or number identifying the location to its users.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The organization responsible for the provisioning and upkeep of the location.
    pub managing_organization: super::reference::Reference,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// Indicates whether a resource instance represents a specific location or a class
    /// of locations.
    pub mode: super::code::Code,
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
    /// Name of the location as used by humans. Does not need to be unique.
    pub name: super::string::String,
    /// The operational status covers operation values most relevant to beds (but can
    /// also apply to rooms/units/chairs/etc. such as an isolation unit/dialysis chair).
    /// This typically covers concepts such as contamination, housekeeping, and other
    /// activities like maintenance.
    pub operational_status: super::coding::Coding,
    /// Another Location of which this Location is physically a part of.
    pub part_of: super::reference::Reference,
    /// The absolute geographic location of the Location, expressed using the WGS84
    /// datum (This is the same co-ordinate system used in KML).
    pub position: super::location::LocationPosition,
    /// This is a Location resource
    pub resource_type: String,
    /// The status property covers the general availability of the resource, not the
    /// current value which may be covered by the operationStatus, or by a schedule/
    /// slots if they are configured for the location.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Indicates the type of function performed at the location.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
    /// Connection details of a virtual service (e.g. shared conference call facility
    /// with dedicated number/details).
    pub virtual_service: Vec<super::virtual_service_detail::VirtualServiceDetail>,
}

/// Details and position information for a place where services are provided and
/// resources and participants may be stored, found, contained, or accommodated.
#[derive(Debug, Clone, PartialEq)]
pub struct LocationPosition {
    /// Altitude. The value domain and the interpretation are the same as for the text
    /// of the altitude element in KML (see notes on Location main page).
    pub altitude: super::decimal::Decimal,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Latitude. The value domain and the interpretation are the same as for the text
    /// of the latitude element in KML (see notes on Location main page).
    pub latitude: super::decimal::Decimal,
    /// Longitude. The value domain and the interpretation are the same as for the text
    /// of the longitude element in KML (see notes on Location main page).
    pub longitude: super::decimal::Decimal,
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
