use super::*;
/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.
#[derive(Debug,Clone,PartialEq)]
pub struct DataRequirement {
/// Code filters specify additional constraints on the data, specifying the value
/// set of interest for a particular element of the data. Each code filter defines
/// an additional constraint on the data, i.e. code filters are AND'ed, not OR'ed.
pub code_filter: Vec<DataRequirementCodeFilter>,
/// Date filters specify additional constraints on the data in terms of the
/// applicable date range for specific elements. Each date filter specifies an
/// additional constraint on the data, i.e. date filters are AND'ed, not OR'ed.
pub date_filter: Vec<DataRequirementDateFilter>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Specifies a maximum number of results that are required (uses the _count search
/// parameter).
pub limit: PositiveInt,
/// Indicates that specific elements of the type are referenced by the knowledge
/// module and must be supported by the consumer in order to obtain an effective
/// evaluation. This does not mean that a value is required for this element, only
/// that the consuming system must understand the element and be able to provide
/// values for it if they are available.
///
/// The value of mustSupport SHALL be a FHIRPath resolvable on the type of the
/// DataRequirement. The path SHALL consist only of identifiers, constant indexers,
/// and .resolve() (see the [Simple FHIRPath Profile](fhirpath.html#simple) for
/// full details).
pub must_support: Vec<String>,
/// The profile of the required data, specified as the uri of the profile
/// definition.
pub profile: Vec<Canonical>,
/// Specifies the order of the results to be returned.
pub sort: Vec<DataRequirementSort>,
/// The intended subjects of the data requirement. If this element is not provided,
/// a Patient subject is assumed.
pub subject_codeable_concept: CodeableConcept,
/// The intended subjects of the data requirement. If this element is not provided,
/// a Patient subject is assumed.
pub subject_reference: Reference,
/// The type of the required data, specified as the type name of a resource. For
/// profiles, this value is set to the type of the base resource of the profile.
pub r#type: Code,
/// Value filters specify additional constraints on the data for elements other than
/// code-valued or date-valued. Each value filter specifies an additional constraint
/// on the data (i.e. valueFilters are AND'ed, not OR'ed).
pub value_filter: Vec<DataRequirementValueFilter>,
}
