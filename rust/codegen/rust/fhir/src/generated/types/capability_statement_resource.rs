use super::*;
/// A Capability Statement documents a set of capabilities (behaviors) of a
/// FHIR Server or Client for a particular version of FHIR that may be used as a
/// statement of actual server functionality or a statement of required or desired
/// server implementation.
#[derive(Debug,Clone,PartialEq)]
pub struct CapabilityStatementResource {
/// A flag that indicates that the server supports conditional create.
pub conditional_create: Boolean,
/// A code that indicates how the server supports conditional delete.
pub conditional_delete: Code,
/// A flag that indicates that the server supports conditional patch.
pub conditional_patch: Boolean,
/// A code that indicates how the server supports conditional read.
pub conditional_read: Code,
/// A flag that indicates that the server supports conditional update.
pub conditional_update: Boolean,
/// Additional information about the resource type used by the system.
pub documentation: Markdown,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Identifies a restful operation supported by the solution.
pub interaction: Vec<CapabilityStatementInteraction>,
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
pub modifier_extension: Vec<Extension>,
/// Definition of an operation or a named query together with its parameters and
/// their meaning and type. Consult the definition of the operation for details
/// about how to invoke the operation, and the parameters.
pub operation: Vec<CapabilityStatementOperation>,
/// A system-wide profile that is applied across *all* instances of the resource
/// supported by the system. For example, if declared on Observation, this profile
/// is the "superset" of capabilities for laboratory *and* vitals *and* other
/// domains. See further discussion in [Using Profiles](profiling.html#profile-
/// uses).
pub profile: Canonical,
/// A flag for whether the server is able to return past versions as part of the
/// vRead operation.
pub read_history: Boolean,
/// A set of flags that defines how references are supported.
pub reference_policy: Vec<Code>,
/// A list of _include values supported by the server.
pub search_include: Vec<String>,
/// Search parameters for implementations to support and/or make use of - either
/// references to ones defined in the specification, or additional ones defined for/
/// by the implementation.
pub search_param: Vec<CapabilityStatementSearchParam>,
/// A list of _revinclude (reverse include) values supported by the server.
pub search_rev_include: Vec<String>,
/// A list of profiles representing different use cases the system hosts/produces.
/// A supported profile is a statement about the functionality of the data and
/// services provided by the server (or the client) for supported use cases. For
/// example, a system can define and declare multiple Observation profiles for
/// laboratory observations, vital sign observations, etc. By declaring supported
/// profiles, systems provide a way to determine whether individual resources are
/// conformant. See further discussion in [Using Profiles](profiling.html#profile-
/// uses).
pub supported_profile: Vec<Canonical>,
/// A type of resource exposed via the restful interface.
pub r#type: Code,
/// A flag to indicate that the server allows or needs to allow the client to create
/// new identities on the server (that is, the client PUTs to a location where there
/// is no existing resource). Allowing this operation means that the server allows
/// the client to create new identities on the server.
pub update_create: Boolean,
/// This field is set to no-version to specify that the system does not support
/// (server) or use (client) versioning for this resource type. If this has some
/// other value, the server must at least correctly track and populate the versionId
/// meta-property on resources. If the value is 'versioned-update', then the
/// server supports all the versioning features, including using e-tags for version
/// integrity in the API.
pub versioning: Code,
}
