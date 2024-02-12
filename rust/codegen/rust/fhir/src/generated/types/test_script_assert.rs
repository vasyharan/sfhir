use super::*;
/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug,Clone,PartialEq)]
pub struct TestScriptAssert {
/// The FHIRPath expression for a specific value to evaluate against the source
/// fixture. When compareToSourceId is defined, either compareToSourceExpression or
/// compareToSourcePath must be defined, but not both.
pub compare_to_source_expression: String,
/// Id of the source fixture used as the contents to be evaluated by either the
/// "source/expression" or "sourceId/path" definition.
pub compare_to_source_id: String,
/// XPath or JSONPath expression to evaluate against the source fixture.
/// When compareToSourceId is defined, either compareToSourceExpression or
/// compareToSourcePath must be defined, but not both.
pub compare_to_source_path: String,
/// The mime-type contents to compare against the request or response message
/// 'Content-Type' header.
pub content_type: Code,
/// The default manual completion outcome applied to this assertion.
pub default_manual_completion: Code,
/// The description would be used by test engines for tracking and reporting
/// purposes.
pub description: String,
/// The direction to use for the assertion.
pub direction: Code,
/// The FHIRPath expression to be evaluated against the request or response message
/// contents - HTTP headers and payload.
pub expression: String,
/// The HTTP header field name e.g. 'Location'.
pub header_field: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The label would be used for tracking/logging purposes by test engines.
pub label: String,
/// The ID of a fixture. Asserts that the response contains at a minimum the fixture
/// specified by minimumId.
pub minimum_id: String,
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
/// Whether or not the test execution performs validation on the bundle navigation
/// links.
pub navigation_links: Boolean,
/// The operator type defines the conditional behavior of the assert.
pub operator: Code,
/// The XPath or JSONPath expression to be evaluated against the fixture
/// representing the response received from server.
pub path: String,
/// The request method or HTTP operation code to compare against that used by the
/// client system under test.
pub request_method: Code,
/// The value to use in a comparison against the request URL path string.
pub request_url: String,
/// Links or references providing traceability to the testing requirements for this
/// assert.
pub requirement: Vec<TestScriptRequirement>,
/// The type of the resource.  See the [resource list](resourcelist.html).
pub resource: Uri,
/// continue | switchingProtocols | okay | created | accepted |
/// nonAuthoritativeInformation | noContent | resetContent | partialContent
/// | multipleChoices | movedPermanently | found | seeOther | notModified |
/// useProxy | temporaryRedirect | permanentRedirect | badRequest | unauthorized
/// | paymentRequired | forbidden | notFound | methodNotAllowed | notAcceptable |
/// proxyAuthenticationRequired | requestTimeout | conflict | gone | lengthRequired
/// | preconditionFailed | contentTooLarge | uriTooLong | unsupportedMediaType
/// | rangeNotSatisfiable | expectationFailed | misdirectedRequest |
/// unprocessableContent | upgradeRequired | internalServerError | notImplemented |
/// badGateway | serviceUnavailable | gatewayTimeout | httpVersionNotSupported.
pub response: Code,
/// The value of the HTTP response code to be tested.
pub response_code: String,
/// Fixture to evaluate the XPath/JSONPath expression or the headerField  against.
pub source_id: Id,
/// Whether or not the current test execution will stop on failure for this assert.
pub stop_test_on_fail: Boolean,
/// The ID of the Profile to validate against.
pub validate_profile_id: Id,
/// The value to compare to.
pub value: String,
/// Whether or not the test execution will produce a warning only on error for this
/// assert.
pub warning_only: Boolean,
}
