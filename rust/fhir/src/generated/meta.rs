/// The metadata about a resource. This is content in the resource that is
/// maintained by the infrastructure. Changes to the content might not always be
/// associated with version changes to the resource.
#[derive(Debug, Clone, PartialEq)]
pub struct Meta {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// When the resource last changed - e.g. when the version changed.
    pub last_updated: super::instant::Instant,
    /// A list of profiles (references to [[[StructureDefinition]]] resources)
    /// that this resource claims to conform to. The URL is a reference to
    /// [[[StructureDefinition.url]]].
    pub profile: Vec<super::canonical::Canonical>,
    /// Security labels applied to this resource. These tags connect specific resources
    /// to the overall security policy and infrastructure.
    pub security: Vec<super::coding::Coding>,
    /// A uri that identifies the source system of the resource. This provides a
    /// minimal amount of [[[Provenance]]] information that can be used to track or
    /// differentiate the source of information in the resource. The source may identify
    /// another FHIR server, document, message, database, etc.
    pub source: super::uri::Uri,
    /// Tags applied to this resource. Tags are intended to be used to identify and
    /// relate resources to process and workflow, and applications are not required to
    /// consider the tags when interpreting the meaning of a resource.
    pub tag: Vec<super::coding::Coding>,
    /// The version specific identifier, as it appears in the version portion of the
    /// URL. This value changes when the resource is created, updated, or deleted.
    pub version_id: super::id::Id,
}
