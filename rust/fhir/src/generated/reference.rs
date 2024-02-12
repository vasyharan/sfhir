/// A reference from one resource to another.
#[derive(Debug, Clone, PartialEq)]
pub struct Reference {
    /// Plain text narrative that identifies the resource in addition to the resource
    /// reference.
    pub display: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// An identifier for the target resource. This is used when there is no way to
    /// reference the other resource directly, either because the entity it represents
    /// is not available through a FHIR server, or because there is no way for the
    /// author of the resource to convert a known identifier to an actual location.
    /// There is no requirement that a Reference.identifier point to something that is
    /// actually exposed as a FHIR instance, but it SHALL point to a business concept
    /// that would be expected to be exposed as a FHIR instance, and that instance would
    /// need to be of a FHIR resource type allowed by the reference.
    pub identifier: super::identifier::Identifier,
    /// A reference to a location at which the other resource is found. The reference
    /// may be a relative reference, in which case it is relative to the service base
    /// URL, or an absolute URL that resolves to the location where the resource is
    /// found. The reference may be version specific or not. If the reference is not
    /// to a FHIR RESTful server, then it should be assumed to be version specific.
    /// Internal fragment references (start with '#') refer to contained resources.
    pub reference: super::string::String,
    /// The expected type of the target of the reference. If both Reference.type and
    /// Reference.reference are populated and Reference.reference is a FHIR URL, both
    /// SHALL be consistent.
    ///
    /// The type is the Canonical URL of Resource Definition that is the type this
    /// reference refers to. References are URLs that are relative to http://hl7.org/
    /// fhir/StructureDefinition/ e.g. "Patient" is a reference to http://hl7.org/fhir/
    /// StructureDefinition/Patient. Absolute URLs are only allowed for logical models
    /// (and can only be used in references in logical models, not resources).
    pub r#type: super::uri::Uri,
}
