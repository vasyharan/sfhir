use super::*;
/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug,Clone,PartialEq)]
pub struct TestScriptOperation {
/// The mime-type to use for RESTful operation in the 'Accept' header.
pub accept: Code,
/// The mime-type to use for RESTful operation in the 'Content-Type' header.
pub content_type: Code,
/// The description would be used by test engines for tracking and reporting
/// purposes.
pub description: String,
/// The server where the request message is destined for.  Must be one of the server
/// numbers listed in TestScript.destination section.
pub destination: Integer,
/// Whether or not to implicitly send the request url in encoded format. The
/// default is true to match the standard RESTful client behavior. Set to false when
/// communicating with a server that does not support encoded url paths.
pub encode_request_url: Boolean,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The label would be used for tracking/logging purposes by test engines.
pub label: String,
/// The HTTP method the test engine MUST use for this operation regardless of any
/// other operation details.
pub method: Code,
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
/// The server where the request message originates from.  Must be one of the server
/// numbers listed in TestScript.origin section.
pub origin: Integer,
/// Path plus parameters after [type].  Used to set parts of the request URL
/// explicitly.
pub params: String,
/// Header elements would be used to set HTTP headers.
pub request_header: Vec<TestScriptRequestHeader>,
/// The fixture id (maybe new) to map to the request.
pub request_id: Id,
/// The type of the FHIR resource. See the [resource list](resourcelist.html). Data
/// type of uri is needed when non-HL7 artifacts are identified.
pub resource: Uri,
/// The fixture id (maybe new) to map to the response.
pub response_id: Id,
/// The id of the fixture used as the body of a PUT or POST request.
pub source_id: Id,
/// Id of fixture used for extracting the [id],  [type], and [vid] for GET requests.
pub target_id: Id,
/// Server interaction or operation type.
pub r#type: Coding,
/// Complete request URL.
pub url: String,
}
