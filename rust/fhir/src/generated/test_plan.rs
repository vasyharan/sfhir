/// A plan for executing testing on an artifact or specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlan {
    /// The category of the Test Plan - can be acceptance, unit, performance, etc.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the test plan and/or its contents. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// test plan. The short copyright declaration (e.g. (c) '2015+ xyz organization'
    /// should be sent in the copyrightLabel element.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date (and optionally time) when the test plan was last significantly
    /// changed. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the test plan changes.
    pub date: super::date_time::DateTime,
    /// The required criteria to execute the test plan - e.g. preconditions, previous
    /// tests...
    pub dependency: Vec<super::test_plan::TestPlanDependency>,
    /// A free text natural language description of the test plan from a consumer's
    /// perspective.
    pub description: super::markdown::Markdown,
    /// The threshold or criteria for the test plan to be considered successfully
    /// executed - narrative.
    pub exit_criteria: super::markdown::Markdown,
    /// A Boolean value to indicate that this test plan is authored for testing purposes
    /// (or education/evaluation/marketing) and is not intended to be used for genuine
    /// usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this test plan when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the test plan is intended to be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
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
    /// A natural language name identifying the test plan. This name should be usable
    /// as an identifier for the module by machine processing applications such as code
    /// generation.
    pub name: super::string::String,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the test plan.
    pub publisher: super::string::String,
    /// Explanation of why this test plan is needed and why it has been designed as
    /// it has.
    pub purpose: super::markdown::Markdown,
    /// This is a TestPlan resource
    pub resource_type: String,
    /// What is being tested with this Test Plan - a conformance resource, or narrative
    /// criteria, or an external reference...
    pub scope: Vec<super::reference::Reference>,
    /// The status of this test plan. Enables tracking the life-cycle of the content.
    pub status: super::code::Code,
    /// The individual test cases that are part of this plan, when they they are made
    /// explicit.
    pub test_case: Vec<super::test_plan::TestPlanTestCase>,
    /// A description of test tools to be used in the test plan.
    pub test_tools: super::markdown::Markdown,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the test plan.
    pub title: super::string::String,
    /// An absolute URI that is used to identify this test plan when it is referenced
    /// in a specification, model, design or an instance; also called its canonical
    /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
    /// which an authoritative instance of this test plan is (or will be) published.
    /// This URL can be the target of a canonical reference. It SHALL remain the same
    /// when the test plan is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...)
    /// or may be references to specific programs (insurance plans, studies, ...) and
    /// may be used to assist with indexing and searching for appropriate test plan
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the test plan when it is
    /// referenced in a specification, model, design or instance.  This is an arbitrary
    /// value managed by the test plan author and is not expected to be globally unique.
    /// For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is
    /// not available. There is also no expectation that versions can be placed in a
    /// lexicographical sequence.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// A plan for executing testing on an artifact or specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlanAssertion {
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
    /// The focus or object of the assertion i.e. a resource.
    pub object: Vec<super::codeable_reference::CodeableReference>,
    /// The test assertion - the expected outcome from the test case execution.
    pub result: Vec<super::codeable_reference::CodeableReference>,
    /// The test assertion type - this can be used to group assertions as 'required' or
    /// 'optional', or can be used for other classification of the assertion.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
}

/// A plan for executing testing on an artifact or specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlanDependency {
    /// A textual description of the criterium - what is needed for the dependency to be
    /// considered met.
    pub description: super::markdown::Markdown,
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
    /// Predecessor test plans - those that are expected to be successfully performed as
    /// a dependency for the execution of this test plan.
    pub predecessor: super::reference::Reference,
}

/// A plan for executing testing on an artifact or specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlanDependency1 {
    /// Description of the criteria.
    pub description: super::markdown::Markdown,
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
    /// Link to predecessor test plans.
    pub predecessor: super::reference::Reference,
}

/// A plan for executing testing on an artifact or specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlanScript {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The language for the test cases e.g. 'gherkin', 'testscript'.
    pub language: super::codeable_concept::CodeableConcept,
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
    /// The actual content of the cases - references to TestScripts or externally
    /// defined content.
    pub source_reference: super::reference::Reference,
    /// The actual content of the cases - references to TestScripts or externally
    /// defined content.
    pub source_string: String,
}

/// A plan for executing testing on an artifact or specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlanTestCase {
    /// The test assertions - the expectations of test results from the execution of the
    /// test case.
    pub assertion: Vec<super::test_plan::TestPlanAssertion>,
    /// The required criteria to execute the test case - e.g. preconditions, previous
    /// tests.
    pub dependency: Vec<super::test_plan::TestPlanDependency1>,
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
    /// The scope or artifact covered by the case, when the individual test case is
    /// associated with a testable artifact.
    pub scope: Vec<super::reference::Reference>,
    /// Sequence of test case - an ordinal number that indicates the order for the
    /// present test case in the test plan.
    pub sequence: super::integer::Integer,
    /// The test data used in the test case.
    pub test_data: Vec<super::test_plan::TestPlanTestData>,
    /// The actual test to be executed.
    pub test_run: Vec<super::test_plan::TestPlanTestRun>,
}

/// A plan for executing testing on an artifact or specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlanTestData {
    /// The actual test resources when they exist.
    pub content: super::reference::Reference,
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
    /// Pointer to a definition of test resources - narrative or structured e.g.
    /// synthetic data generation, etc.
    pub source_reference: super::reference::Reference,
    /// Pointer to a definition of test resources - narrative or structured e.g.
    /// synthetic data generation, etc.
    pub source_string: String,
    /// The type of test data description, e.g. 'synthea'.
    pub r#type: super::coding::Coding,
}

/// A plan for executing testing on an artifact or specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct TestPlanTestRun {
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
    /// The narrative description of the tests.
    pub narrative: super::markdown::Markdown,
    /// The test cases in a structured language e.g. gherkin, Postman, or FHIR
    /// TestScript.
    pub script: super::test_plan::TestPlanScript,
}
