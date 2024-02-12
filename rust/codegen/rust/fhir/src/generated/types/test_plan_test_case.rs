use super::*;
/// A plan for executing testing on an artifact or specifications.
#[derive(Debug,Clone,PartialEq)]
pub struct TestPlanTestCase {
/// The test assertions - the expectations of test results from the execution of the
/// test case.
pub assertion: Vec<TestPlanAssertion>,
/// The required criteria to execute the test case - e.g. preconditions, previous
/// tests.
pub dependency: Vec<TestPlanDependency1>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
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
/// The scope or artifact covered by the case, when the individual test case is
/// associated with a testable artifact.
pub scope: Vec<Reference>,
/// Sequence of test case - an ordinal number that indicates the order for the
/// present test case in the test plan.
pub sequence: Integer,
/// The test data used in the test case.
pub test_data: Vec<TestPlanTestData>,
/// The actual test to be executed.
pub test_run: Vec<TestPlanTestRun>,
}
