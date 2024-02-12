/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMap {
    /// An additionalAttribute defines an additional data element found in the source or
    /// target data model where the data will come from or be mapped to. Some mappings
    /// are based on data in addition to the source data element, where codes in
    /// multiple fields are combined to a single field (or vice versa).
    pub additional_attribute: Vec<super::concept_map::ConceptMapAdditionalAttribute>,
    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// An individiual or organization primarily involved in the creation and
    /// maintenance of the ConceptMap.
    pub author: Vec<super::contact_detail::ContactDetail>,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the concept map and/or its contents. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// concept map.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the concept map was last significantly
    /// changed. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the concept map changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the concept map from a consumer's
    /// perspective.
    pub description: super::markdown::Markdown,
    /// An individual or organization primarily responsible for internal coherence of
    /// the ConceptMap.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the ConceptMap content was or is planned to be in active
    /// use.
    pub effective_period: super::period::Period,
    /// An individual or organization asserted by the publisher to be responsible for
    /// officially endorsing the ConceptMap for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A Boolean value to indicate that this concept map is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// A group of mappings that all have the same source and target system.
    pub group: Vec<super::concept_map::ConceptMapGroup>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this concept map when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the concept map is intended to be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The date on which the resource content was last reviewed. Review happens
    /// periodically after approval but does not change the original approval date.
    pub last_review_date: super::date::Date,
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
    /// A natural language name identifying the concept map. This name should be usable
    /// as an identifier for the module by machine processing applications such as code
    /// generation.
    pub name: super::string::String,
    /// A property defines a slot through which additional information can be provided
    /// about a map from source -> target.
    pub property: Vec<super::concept_map::ConceptMapProperty>,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the concept map.
    pub publisher: super::string::String,
    /// Explanation of why this concept map is needed and why it has been designed as
    /// it has.
    pub purpose: super::markdown::Markdown,
    /// Related artifacts such as additional documentation, justification, dependencies,
    /// bibliographic references, and predecessor and successor artifacts.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a ConceptMap resource
    pub resource_type: String,
    /// An individual or organization asserted by the publisher to be primarily
    /// responsible for review of some aspect of the ConceptMap.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// Identifier for the source value set that contains the concepts that are being
    /// mapped and provides context for the mappings.  Limits the scope of the map to
    /// source codes (ConceptMap.group.element code or valueSet) that are members of
    /// this value set.
    pub source_scope_canonical: String,
    /// Identifier for the source value set that contains the concepts that are being
    /// mapped and provides context for the mappings.  Limits the scope of the map to
    /// source codes (ConceptMap.group.element code or valueSet) that are members of
    /// this value set.
    pub source_scope_uri: String,
    /// The status of this concept map. Enables tracking the life-cycle of the content.
    pub status: super::code::Code,
    /// Identifier for the target value set that provides important context about how
    /// the mapping choices are made.  Limits the scope of the map to target codes
    /// (ConceptMap.group.element.target code or valueSet) that are members of this
    /// value set.
    pub target_scope_canonical: String,
    /// Identifier for the target value set that provides important context about how
    /// the mapping choices are made.  Limits the scope of the map to target codes
    /// (ConceptMap.group.element.target code or valueSet) that are members of this
    /// value set.
    pub target_scope_uri: String,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the concept map.
    pub title: super::string::String,
    /// Descriptions related to the content of the ConceptMap. Topics provide a high-
    /// level categorization as well as keywords for the ConceptMap that can be useful
    /// for filtering and searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// An absolute URI that is used to identify this concept map when it is referenced
    /// in a specification, model, design or an instance; also called its canonical
    /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
    /// which an authoritative instance of this concept map is (or will be) published.
    /// This URL can be the target of a canonical reference. It SHALL remain the same
    /// when the concept map is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...)
    /// or may be references to specific programs (insurance plans, studies, ...) and
    /// may be used to assist with indexing and searching for appropriate concept map
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the concept map when
    /// it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the concept map author and is not expected to be
    /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which ConceptMap
    /// is more current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which ConceptMap
    /// is more current.
    pub version_algorithm_string: String,
}

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapAdditionalAttribute {
    /// A code that is used to identify this additional data attribute. The code is
    /// used internally in ConceptMap.group.element.target.dependsOn.attribute and
    /// ConceptMap.group.element.target.product.attribute.
    pub code: super::code::Code,
    /// A description of the additional attribute and/or the data element it refers to -
    /// why it is defined, and how the value might be used in mappings, and a discussion
    /// of issues associated with the use of the data element.
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
    /// The type of the source data contained in this concept map for this data element.
    pub r#type: super::code::Code,
    /// Reference to the formal definition of the source/target data element. For
    /// elements defined by the FHIR specification, or using a FHIR logical model, the
    /// correct format is {canonical-url}#{element-id}.
    pub uri: super::uri::Uri,
}

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapDependsOn {
    /// A reference to the additional attribute that holds a value the map depends on.
    pub attribute: super::code::Code,
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
    /// Data element value that the map depends on / produces.
    pub value_boolean: bool,
    /// Data element value that the map depends on / produces.
    pub value_code: String,
    /// Data element value that the map depends on / produces.
    pub value_coding: super::coding::Coding,
    /// Data element value that the map depends on / produces.
    pub value_quantity: super::quantity::Quantity,
    /// This mapping applies if the data element value is a code from this value set.
    pub value_set: super::canonical::Canonical,
    /// Data element value that the map depends on / produces.
    pub value_string: String,
}

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapElement {
    /// Identity (code or path) or the element/item being mapped.
    pub code: super::code::Code,
    /// The display for the code. The display is only provided to help editors when
    /// editing the concept map.
    pub display: super::string::String,
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
    /// If noMap = true this indicates that no mapping to a target concept exists for
    /// this source concept.
    pub no_map: super::boolean::Boolean,
    /// A concept from the target value set that this concept maps to.
    pub target: Vec<super::concept_map::ConceptMapTarget>,
    /// The set of concepts from the ConceptMap.group.source code system which are all
    /// being mapped to the target as part of this mapping rule.
    pub value_set: super::canonical::Canonical,
}

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapGroup {
    /// Mappings for an individual concept in the source to one or more concepts in
    /// the target.
    pub element: Vec<super::concept_map::ConceptMapElement>,
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
    /// An absolute URI that identifies the source system where the concepts to be
    /// mapped are defined.
    pub source: super::canonical::Canonical,
    /// An absolute URI that identifies the target system that the concepts will be
    /// mapped to.
    pub target: super::canonical::Canonical,
    /// What to do when there is no mapping to a target concept from the source concept
    /// and ConceptMap.group.element.noMap is not true. This provides the "default" to
    /// be applied when there is no target concept mapping specified or the expansion of
    /// ConceptMap.group.element.target.valueSet is empty.
    pub unmapped: super::concept_map::ConceptMapUnmapped,
}

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapProperty {
    /// A code that is used to identify the property. The code is used internally
    /// (in ConceptMap.group.element.target.property.code) and also in the $translate
    /// operation.
    pub code: super::code::Code,
    /// A description of the property - why it is defined, and how its value might be
    /// used.
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
    /// The CodeSystem that defines the codes from which values of type ```code``` in
    /// property values.
    pub system: super::canonical::Canonical,
    /// The type of the property value.
    pub r#type: super::code::Code,
    /// Reference to the formal meaning of the property.
    pub uri: super::uri::Uri,
}

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapProperty1 {
    /// A reference to a mapping property defined in ConceptMap.property.
    pub code: super::code::Code,
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
    /// The value of this property. If the type chosen for this element is 'code', then
    /// the property SHALL be defined in a ConceptMap.property element.
    pub value_boolean: bool,
    /// The value of this property. If the type chosen for this element is 'code', then
    /// the property SHALL be defined in a ConceptMap.property element.
    pub value_code: String,
    /// The value of this property. If the type chosen for this element is 'code', then
    /// the property SHALL be defined in a ConceptMap.property element.
    pub value_coding: super::coding::Coding,
    /// The value of this property. If the type chosen for this element is 'code', then
    /// the property SHALL be defined in a ConceptMap.property element.
    pub value_date_time: String,
    /// The value of this property. If the type chosen for this element is 'code', then
    /// the property SHALL be defined in a ConceptMap.property element.
    pub value_decimal: f64,
    /// The value of this property. If the type chosen for this element is 'code', then
    /// the property SHALL be defined in a ConceptMap.property element.
    pub value_integer: i64,
    /// The value of this property. If the type chosen for this element is 'code', then
    /// the property SHALL be defined in a ConceptMap.property element.
    pub value_string: String,
}

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapTarget {
    /// Identity (code or path) or the element/item that the map refers to.
    pub code: super::code::Code,
    /// A description of status/issues in mapping that conveys additional information
    /// not represented in  the structured data.
    pub comment: super::string::String,
    /// A set of additional dependencies for this mapping to hold. This mapping is
    /// only applicable if the specified data attribute can be resolved, and it has the
    /// specified value.
    pub depends_on: Vec<super::concept_map::ConceptMapDependsOn>,
    /// The display for the code. The display is only provided to help editors when
    /// editing the concept map.
    pub display: super::string::String,
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
    /// Product is the output of a ConceptMap that provides additional values that go in
    /// other attributes / data elemnts of the target data.
    pub product: Vec<super::concept_map::ConceptMapDependsOn>,
    /// A property value for this source -> target mapping.
    pub property: Vec<super::concept_map::ConceptMapProperty1>,
    /// The relationship between the source and target concepts. The relationship is
    /// read from source to target (e.g. source-is-narrower-than-target).
    pub relationship: super::code::Code,
    /// The set of concepts from the ConceptMap.group.target code system which are all
    /// being mapped to as part of this mapping rule. The effect of using this data
    /// element is the same as having multiple ConceptMap.group.element.target elements
    /// with one for each concept in the ConceptMap.group.element.target.valueSet value
    /// set.
    pub value_set: super::canonical::Canonical,
}

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapUnmapped {
    /// The fixed code to use when the mode = 'fixed'  - all unmapped codes are mapped
    /// to a single fixed code.
    pub code: super::code::Code,
    /// The display for the code. The display is only provided to help editors when
    /// editing the concept map.
    pub display: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Defines which action to take if there is no match for the source concept in
    /// the target system designated for the group. One of 3 actions are possible:
    /// use the unmapped source code (this is useful when doing a mapping between
    /// versions, and only a few codes have changed), use a fixed code (a default code),
    /// or alternatively, a reference to a different concept map can be provided (by
    /// canonical URL).
    pub mode: super::code::Code,
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
    /// The canonical reference to an additional ConceptMap resource instance to use for
    /// mapping if this ConceptMap resource contains no matching mapping for the source
    /// concept.
    pub other_map: super::canonical::Canonical,
    /// The default relationship value to apply between the source and target concepts
    /// when the source code is unmapped and the mode is 'fixed' or 'use-source-code'.
    pub relationship: super::code::Code,
    /// The set of fixed codes to use when the mode = 'fixed'  - all unmapped codes are
    /// mapped to each of the fixed codes.
    pub value_set: super::canonical::Canonical,
}
