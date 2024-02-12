/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScript {
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the test script and/or its contents. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// test script.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date (and optionally time) when the test script was last significantly
    /// changed. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the test script changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the test script from a consumer's
    /// perspective.
    pub description: super::markdown::Markdown,
    /// An abstract server used in operations within this test script in the destination
    /// element.
    pub destination: Vec<super::test_script::TestScriptDestination>,
    /// A Boolean value to indicate that this test script is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// Fixture in the test script - by reference (uri). All fixtures are required for
    /// the test script to execute.
    pub fixture: Vec<super::test_script::TestScriptFixture>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this test script when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the test script is intended to be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// The required capability must exist and are assumed to function correctly on the
    /// FHIR server being tested.
    pub metadata: super::test_script::TestScriptMetadata,
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
    /// A natural language name identifying the test script. This name should be usable
    /// as an identifier for the module by machine processing applications such as code
    /// generation.
    pub name: super::string::String,
    /// An abstract server used in operations within this test script in the origin
    /// element.
    pub origin: Vec<super::test_script::TestScriptOrigin>,
    /// Reference to the profile to be used for validation.
    pub profile: Vec<super::canonical::Canonical>,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the test script.
    pub publisher: super::string::String,
    /// Explanation of why this test script is needed and why it has been designed as
    /// it has.
    pub purpose: super::markdown::Markdown,
    /// This is a TestScript resource
    pub resource_type: String,
    /// The scope indicates a conformance artifact that is tested by the test(s)
    /// within this test case and the expectation of the test outcome(s) as well as the
    /// intended test phase inclusion.
    pub scope: Vec<super::test_script::TestScriptScope>,
    /// A series of required setup operations before tests are executed.
    pub setup: super::test_script::TestScriptSetup,
    /// The status of this test script. Enables tracking the life-cycle of the content.
    pub status: super::code::Code,
    /// A series of operations required to clean up after all the tests are executed
    /// (successfully or otherwise).
    pub teardown: super::test_script::TestScriptTeardown,
    /// A test in this script.
    pub test: Vec<super::test_script::TestScriptTest>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the test script.
    pub title: super::string::String,
    /// An absolute URI that is used to identify this test script when it is referenced
    /// in a specification, model, design or an instance; also called its canonical
    /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
    /// which an authoritative instance of this test script is (or will be) published.
    /// This URL can be the target of a canonical reference. It SHALL remain the same
    /// when the test script is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...)
    /// or may be references to specific programs (insurance plans, studies, ...) and
    /// may be used to assist with indexing and searching for appropriate test script
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// Variable is set based either on element value in response body or on header
    /// field value in the response headers.
    pub variable: Vec<super::test_script::TestScriptVariable>,
    /// The identifier that is used to identify this version of the test script when
    /// it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the test script author and is not expected to be
    /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptAction {
    /// Evaluates the results of previous operations to determine if the server under
    /// test behaves appropriately.
    pub assert: super::test_script::TestScriptAssert,
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
    /// The operation to perform.
    pub operation: super::test_script::TestScriptOperation,
}

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptAction1 {
    /// Evaluates the results of previous operations to determine if the server under
    /// test behaves appropriately.
    pub assert: super::test_script::TestScriptAssert,
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
    /// An operation would involve a REST request to a server.
    pub operation: super::test_script::TestScriptOperation,
}

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptAction2 {
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
    /// An operation would involve a REST request to a server.
    pub operation: super::test_script::TestScriptOperation,
}

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptAssert {
    /// The FHIRPath expression for a specific value to evaluate against the source
    /// fixture. When compareToSourceId is defined, either compareToSourceExpression or
    /// compareToSourcePath must be defined, but not both.
    pub compare_to_source_expression: super::string::String,
    /// Id of the source fixture used as the contents to be evaluated by either the
    /// "source/expression" or "sourceId/path" definition.
    pub compare_to_source_id: super::string::String,
    /// XPath or JSONPath expression to evaluate against the source fixture.
    /// When compareToSourceId is defined, either compareToSourceExpression or
    /// compareToSourcePath must be defined, but not both.
    pub compare_to_source_path: super::string::String,
    /// The mime-type contents to compare against the request or response message
    /// 'Content-Type' header.
    pub content_type: super::code::Code,
    /// The default manual completion outcome applied to this assertion.
    pub default_manual_completion: super::code::Code,
    /// The description would be used by test engines for tracking and reporting
    /// purposes.
    pub description: super::string::String,
    /// The direction to use for the assertion.
    pub direction: super::code::Code,
    /// The FHIRPath expression to be evaluated against the request or response message
    /// contents - HTTP headers and payload.
    pub expression: super::string::String,
    /// The HTTP header field name e.g. 'Location'.
    pub header_field: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The label would be used for tracking/logging purposes by test engines.
    pub label: super::string::String,
    /// The ID of a fixture. Asserts that the response contains at a minimum the fixture
    /// specified by minimumId.
    pub minimum_id: super::string::String,
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
    /// Whether or not the test execution performs validation on the bundle navigation
    /// links.
    pub navigation_links: super::boolean::Boolean,
    /// The operator type defines the conditional behavior of the assert.
    pub operator: super::code::Code,
    /// The XPath or JSONPath expression to be evaluated against the fixture
    /// representing the response received from server.
    pub path: super::string::String,
    /// The request method or HTTP operation code to compare against that used by the
    /// client system under test.
    pub request_method: super::code::Code,
    /// The value to use in a comparison against the request URL path string.
    pub request_url: super::string::String,
    /// Links or references providing traceability to the testing requirements for this
    /// assert.
    pub requirement: Vec<super::test_script::TestScriptRequirement>,
    /// The type of the resource.  See the [resource list](resourcelist.html).
    pub resource: super::uri::Uri,
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
    pub response: super::code::Code,
    /// The value of the HTTP response code to be tested.
    pub response_code: super::string::String,
    /// Fixture to evaluate the XPath/JSONPath expression or the headerField  against.
    pub source_id: super::id::Id,
    /// Whether or not the current test execution will stop on failure for this assert.
    pub stop_test_on_fail: super::boolean::Boolean,
    /// The ID of the Profile to validate against.
    pub validate_profile_id: super::id::Id,
    /// The value to compare to.
    pub value: super::string::String,
    /// Whether or not the test execution will produce a warning only on error for this
    /// assert.
    pub warning_only: super::boolean::Boolean,
}

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptCapability {
    /// Minimum capabilities required of server for test script to execute successfully.
    /// If server does not meet at a minimum the referenced capability statement, then
    /// all tests in this script are skipped.
    pub capabilities: super::canonical::Canonical,
    /// Description of the capabilities that this test script is requiring the server
    /// to support.
    pub description: super::string::String,
    /// Which server these requirements apply to.
    pub destination: super::integer::Integer,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Links to the FHIR specification that describes this interaction and the
    /// resources involved in more detail.
    pub link: Vec<super::uri::Uri>,
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
    /// Which origin server these requirements apply to.
    pub origin: Vec<super::integer::Integer>,
    /// Whether or not the test execution will require the given capabilities of the
    /// server in order for this test script to execute.
    pub required: super::boolean::Boolean,
    /// Whether or not the test execution will validate the given capabilities of the
    /// server in order for this test script to execute.
    pub validated: super::boolean::Boolean,
}

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptDestination {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Abstract name given to a destination server in this test script.  The name is
    /// provided as a number starting at 1.
    pub index: super::integer::Integer,
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
    /// The type of destination profile the test system supports.
    pub profile: super::coding::Coding,
    /// The explicit url path of the destination server used in this test script.
    pub url: super::url::Url,
}

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptFixture {
    /// Whether or not to implicitly create the fixture during setup. If true,
    /// the fixture is automatically created on each server being tested during
    /// setup, therefore no create operation is required for this fixture in the
    /// TestScript.setup section.
    pub autocreate: super::boolean::Boolean,
    /// Whether or not to implicitly delete the fixture during teardown. If true,
    /// the fixture is automatically deleted on each server being tested during
    /// teardown, therefore no delete operation is required for this fixture in the
    /// TestScript.teardown section.
    pub autodelete: super::boolean::Boolean,
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
    /// Reference to the resource (containing the contents of the resource needed for
    /// operations). This is allowed to be a Parameters resource.
    pub resource: super::reference::Reference,
}

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptLink {
    /// Short description of the link.
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
    /// URL to a particular requirement or feature within the FHIR specification.
    pub url: super::uri::Uri,
}

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptMetadata {
    /// Capabilities that must exist and are assumed to function correctly on the FHIR
    /// server being tested.
    pub capability: Vec<super::test_script::TestScriptCapability>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A link to the FHIR specification that this test is covering.
    pub link: Vec<super::test_script::TestScriptLink>,
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

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptOperation {
    /// The mime-type to use for RESTful operation in the 'Accept' header.
    pub accept: super::code::Code,
    /// The mime-type to use for RESTful operation in the 'Content-Type' header.
    pub content_type: super::code::Code,
    /// The description would be used by test engines for tracking and reporting
    /// purposes.
    pub description: super::string::String,
    /// The server where the request message is destined for.  Must be one of the server
    /// numbers listed in TestScript.destination section.
    pub destination: super::integer::Integer,
    /// Whether or not to implicitly send the request url in encoded format. The
    /// default is true to match the standard RESTful client behavior. Set to false when
    /// communicating with a server that does not support encoded url paths.
    pub encode_request_url: super::boolean::Boolean,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The label would be used for tracking/logging purposes by test engines.
    pub label: super::string::String,
    /// The HTTP method the test engine MUST use for this operation regardless of any
    /// other operation details.
    pub method: super::code::Code,
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
    /// The server where the request message originates from.  Must be one of the server
    /// numbers listed in TestScript.origin section.
    pub origin: super::integer::Integer,
    /// Path plus parameters after [type].  Used to set parts of the request URL
    /// explicitly.
    pub params: super::string::String,
    /// Header elements would be used to set HTTP headers.
    pub request_header: Vec<super::test_script::TestScriptRequestHeader>,
    /// The fixture id (maybe new) to map to the request.
    pub request_id: super::id::Id,
    /// The type of the FHIR resource. See the [resource list](resourcelist.html). Data
    /// type of uri is needed when non-HL7 artifacts are identified.
    pub resource: super::uri::Uri,
    /// The fixture id (maybe new) to map to the response.
    pub response_id: super::id::Id,
    /// The id of the fixture used as the body of a PUT or POST request.
    pub source_id: super::id::Id,
    /// Id of fixture used for extracting the [id],  [type], and [vid] for GET requests.
    pub target_id: super::id::Id,
    /// Server interaction or operation type.
    pub r#type: super::coding::Coding,
    /// Complete request URL.
    pub url: super::string::String,
}

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptOrigin {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Abstract name given to an origin server in this test script.  The name is
    /// provided as a number starting at 1.
    pub index: super::integer::Integer,
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
    /// The type of origin profile the test system supports.
    pub profile: super::coding::Coding,
    /// The explicit url path of the origin server used in this test script.
    pub url: super::url::Url,
}

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptRequestHeader {
    /// The HTTP header field e.g. "Accept".
    pub field: super::string::String,
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
    /// The value of the header e.g. "application/fhir+xml".
    pub value: super::string::String,
}

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptRequirement {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Link or reference providing traceability to the testing requirement for this
    /// test.
    pub link_canonical: String,
    /// Link or reference providing traceability to the testing requirement for this
    /// test.
    pub link_uri: String,
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

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptScope {
    /// The specific conformance artifact being tested. The canonical reference can be
    /// version-specific.
    pub artifact: super::canonical::Canonical,
    /// The expectation of whether the test must pass for the system to be considered
    /// conformant with the artifact: required - all tests are expected to pass,
    /// optional - all test are expected to pass but non-pass status may be allowed,
    /// strict - all tests are expected to pass and warnings are treated as a failure.
    pub conformance: super::codeable_concept::CodeableConcept,
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
    /// The phase of testing for this artifact: unit - development / implementation
    /// phase, integration - internal system to system phase, production - live system
    /// to system phase (Note, this may involve pii/phi data).
    pub phase: super::codeable_concept::CodeableConcept,
}

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptSetup {
    /// Action would contain either an operation or an assertion.
    pub action: Vec<super::test_script::TestScriptAction>,
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

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptTeardown {
    /// The teardown action will only contain an operation.
    pub action: Vec<super::test_script::TestScriptAction2>,
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

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptTest {
    /// Action would contain either an operation or an assertion.
    pub action: Vec<super::test_script::TestScriptAction1>,
    /// A short description of the test used by test engines for tracking and reporting
    /// purposes.
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
    /// The name of this test used for tracking/logging purposes by test engines.
    pub name: super::string::String,
}

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TestScriptVariable {
    /// A default, hard-coded, or user-defined value for this variable.
    pub default_value: super::string::String,
    /// A free text natural language description of the variable and its purpose.
    pub description: super::string::String,
    /// The FHIRPath expression for a specific value to evaluate against the fixture
    /// body. When variables are defined, only one of either expression, headerField or
    /// path must be specified.
    pub expression: super::string::String,
    /// Will be used to grab the HTTP header field value from the headers that sourceId
    /// is pointing to.
    pub header_field: super::string::String,
    /// Displayable text string with hint help information to the user when entering a
    /// default value.
    pub hint: super::string::String,
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
    /// Descriptive name for this variable.
    pub name: super::string::String,
    /// XPath or JSONPath to evaluate against the fixture body.  When variables are
    /// defined, only one of either expression, headerField or path must be specified.
    pub path: super::string::String,
    /// Fixture to evaluate the XPath/JSONPath expression or the headerField  against
    /// within this variable.
    pub source_id: super::id::Id,
}
