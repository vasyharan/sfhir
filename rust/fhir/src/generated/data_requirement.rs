/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.
#[derive(Debug, Clone, PartialEq)]
pub struct DataRequirement {
    /// Code filters specify additional constraints on the data, specifying the value
    /// set of interest for a particular element of the data. Each code filter defines
    /// an additional constraint on the data, i.e. code filters are AND'ed, not OR'ed.
    pub code_filter: Vec<super::data_requirement::DataRequirementCodeFilter>,
    /// Date filters specify additional constraints on the data in terms of the
    /// applicable date range for specific elements. Each date filter specifies an
    /// additional constraint on the data, i.e. date filters are AND'ed, not OR'ed.
    pub date_filter: Vec<super::data_requirement::DataRequirementDateFilter>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Specifies a maximum number of results that are required (uses the _count search
    /// parameter).
    pub limit: super::positive_int::PositiveInt,
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
    pub must_support: Vec<super::string::String>,
    /// The profile of the required data, specified as the uri of the profile
    /// definition.
    pub profile: Vec<super::canonical::Canonical>,
    /// Specifies the order of the results to be returned.
    pub sort: Vec<super::data_requirement::DataRequirementSort>,
    /// The intended subjects of the data requirement. If this element is not provided,
    /// a Patient subject is assumed.
    pub subject_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The intended subjects of the data requirement. If this element is not provided,
    /// a Patient subject is assumed.
    pub subject_reference: super::reference::Reference,
    /// The type of the required data, specified as the type name of a resource. For
    /// profiles, this value is set to the type of the base resource of the profile.
    pub r#type: super::code::Code,
    /// Value filters specify additional constraints on the data for elements other than
    /// code-valued or date-valued. Each value filter specifies an additional constraint
    /// on the data (i.e. valueFilters are AND'ed, not OR'ed).
    pub value_filter: Vec<super::data_requirement::DataRequirementValueFilter>,
}

/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.
#[derive(Debug, Clone, PartialEq)]
pub struct DataRequirementCodeFilter {
    /// The codes for the code filter. If values are given, the filter will return only
    /// those data items for which the code-valued attribute specified by the path has a
    /// value that is one of the specified codes. If codes are specified in addition to
    /// a value set, the filter returns items matching a code in the value set or one of
    /// the specified codes.
    pub code: Vec<super::coding::Coding>,
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
    /// The code-valued attribute of the filter. The specified path SHALL be a FHIRPath
    /// resolvable on the specified type of the DataRequirement, and SHALL consist
    /// only of identifiers, constant indexers, and .resolve(). The path is allowed to
    /// contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to
    /// traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile]
    /// (fhirpath.html#simple) for full details). Note that the index must be an
    /// integer constant. The path must resolve to an element of type code, Coding,
    /// or CodeableConcept.
    pub path: super::string::String,
    /// A token parameter that refers to a search parameter defined on the specified
    /// type of the DataRequirement, and which searches on elements of type code,
    /// Coding, or CodeableConcept.
    pub search_param: super::string::String,
    /// The valueset for the code filter. The valueSet and code elements are additive.
    /// If valueSet is specified, the filter will return only those data items for which
    /// the value of the code-valued element specified in the path is a member of the
    /// specified valueset.
    pub value_set: super::canonical::Canonical,
}

/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.
#[derive(Debug, Clone, PartialEq)]
pub struct DataRequirementDateFilter {
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
    /// The date-valued attribute of the filter. The specified path SHALL be a FHIRPath
    /// resolvable on the specified type of the DataRequirement, and SHALL consist
    /// only of identifiers, constant indexers, and .resolve(). The path is allowed to
    /// contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to
    /// traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile]
    /// (fhirpath.html#simple) for full details). Note that the index must be an integer
    /// constant. The path must resolve to an element of type date, dateTime, Period,
    /// Schedule, or Timing.
    pub path: super::string::String,
    /// A date parameter that refers to a search parameter defined on the specified type
    /// of the DataRequirement, and which searches on elements of type date, dateTime,
    /// Period, Schedule, or Timing.
    pub search_param: super::string::String,
    /// The value of the filter. If period is specified, the filter will return only
    /// those data items that fall within the bounds determined by the Period, inclusive
    /// of the period boundaries. If dateTime is specified, the filter will return
    /// only those data items that are equal to the specified dateTime. If a Duration
    /// is specified, the filter will return only those data items that fall within
    /// Duration before now.
    pub value_date_time: String,
    /// The value of the filter. If period is specified, the filter will return only
    /// those data items that fall within the bounds determined by the Period, inclusive
    /// of the period boundaries. If dateTime is specified, the filter will return
    /// only those data items that are equal to the specified dateTime. If a Duration
    /// is specified, the filter will return only those data items that fall within
    /// Duration before now.
    pub value_duration: super::duration::Duration,
    /// The value of the filter. If period is specified, the filter will return only
    /// those data items that fall within the bounds determined by the Period, inclusive
    /// of the period boundaries. If dateTime is specified, the filter will return
    /// only those data items that are equal to the specified dateTime. If a Duration
    /// is specified, the filter will return only those data items that fall within
    /// Duration before now.
    pub value_period: super::period::Period,
}

/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.
#[derive(Debug, Clone, PartialEq)]
pub struct DataRequirementSort {
    /// The direction of the sort, ascending or descending.
    pub direction: Direction,
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
    /// The attribute of the sort. The specified path must be resolvable from the type
    /// of the required data. The path is allowed to contain qualifiers (.) to traverse
    /// sub-elements, as well as indexers ([x]) to traverse multiple-cardinality sub-
    /// elements. Note that the index must be an integer constant.
    pub path: super::string::String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Ascending,
    Descending,
}

/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.
#[derive(Debug, Clone, PartialEq)]
pub struct DataRequirementValueFilter {
    /// The comparator to be used to determine whether the value is matching.
    pub comparator: super::code::Code,
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
    /// The attribute of the filter. The specified path SHALL be a FHIRPath resolvable
    /// on the specified type of the DataRequirement, and SHALL consist only of
    /// identifiers, constant indexers, and .resolve(). The path is allowed to
    /// contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to
    /// traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile]
    /// (fhirpath.html#simple) for full details). Note that the index must be an integer
    /// constant. The path must resolve to an element of a type that is comparable to
    /// the valueFilter.value[x] element for the filter.
    pub path: super::string::String,
    /// A search parameter defined on the specified type of the DataRequirement,
    /// and which searches on elements of a type compatible with the type of the
    /// valueFilter.value[x] for the filter.
    pub search_param: super::string::String,
    /// The value of the filter.
    pub value_date_time: String,
    /// The value of the filter.
    pub value_duration: super::duration::Duration,
    /// The value of the filter.
    pub value_period: super::period::Period,
}
