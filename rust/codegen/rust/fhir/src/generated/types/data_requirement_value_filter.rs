use super::*;
/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.
#[derive(Debug,Clone,PartialEq)]
pub struct DataRequirementValueFilter {
/// The comparator to be used to determine whether the value is matching.
pub comparator: Code,
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
/// The attribute of the filter. The specified path SHALL be a FHIRPath resolvable
/// on the specified type of the DataRequirement, and SHALL consist only of
/// identifiers, constant indexers, and .resolve(). The path is allowed to
/// contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to
/// traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile]
/// (fhirpath.html#simple) for full details). Note that the index must be an integer
/// constant. The path must resolve to an element of a type that is comparable to
/// the valueFilter.value[x] element for the filter.
pub path: String,
/// A search parameter defined on the specified type of the DataRequirement,
/// and which searches on elements of a type compatible with the type of the
/// valueFilter.value[x] for the filter.
pub search_param: String,
/// The value of the filter.
pub value_date_time: String,
/// The value of the filter.
pub value_duration: Duration,
/// The value of the filter.
pub value_period: Period,
}