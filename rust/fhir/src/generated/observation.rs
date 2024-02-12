/// Measurements and simple assertions made about a patient, device or other
/// subject.
#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    /// A plan, proposal or order that is fulfilled in whole or in part by this event.
    /// For example, a MedicationRequest may require a patient to have laboratory test
    /// performed before  it is dispensed.
    pub based_on: Vec<super::reference::Reference>,
    /// Indicates the site on the subject's body where the observation was made (i.e.
    /// the target site).
    pub body_site: super::codeable_concept::CodeableConcept,
    /// Indicates the body structure on the subject's body where the observation was
    /// made (i.e. the target site).
    pub body_structure: super::reference::Reference,
    /// A code that classifies the general type of observation being made.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// Describes what was observed. Sometimes this is called the observation "name".
    pub code: super::codeable_concept::CodeableConcept,
    /// Some observations have multiple component observations.  These component
    /// observations are expressed as separate code value pairs that share the same
    /// attributes.  Examples include systolic and diastolic component observations
    /// for blood pressure measurement and multiple component observations for genetics
    /// observations.
    pub component: Vec<super::observation::ObservationComponent>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Provides a reason why the expected value in the element Observation.value[x]
    /// is missing.
    pub data_absent_reason: super::codeable_concept::CodeableConcept,
    /// The target resource that represents a measurement from which this observation
    /// value is derived. For example, a calculated anion gap or a fetal measurement
    /// based on an ultrasound image.
    pub derived_from: Vec<super::reference::Reference>,
    /// A reference to the device that generates the measurements or the device settings
    /// for the device.
    pub device: super::reference::Reference,
    /// The time or time-period the observed value is asserted as being true. For
    /// biological subjects - e.g. human patients - this is usually called the
    /// "physiologically relevant time". This is usually either the time of the
    /// procedure or of specimen collection, but very often the source of the date/time
    /// is not known, only the date/time itself.
    pub effective_date_time: String,
    /// The time or time-period the observed value is asserted as being true. For
    /// biological subjects - e.g. human patients - this is usually called the
    /// "physiologically relevant time". This is usually either the time of the
    /// procedure or of specimen collection, but very often the source of the date/time
    /// is not known, only the date/time itself.
    pub effective_instant: String,
    /// The time or time-period the observed value is asserted as being true. For
    /// biological subjects - e.g. human patients - this is usually called the
    /// "physiologically relevant time". This is usually either the time of the
    /// procedure or of specimen collection, but very often the source of the date/time
    /// is not known, only the date/time itself.
    pub effective_period: super::period::Period,
    /// The time or time-period the observed value is asserted as being true. For
    /// biological subjects - e.g. human patients - this is usually called the
    /// "physiologically relevant time". This is usually either the time of the
    /// procedure or of specimen collection, but very often the source of the date/time
    /// is not known, only the date/time itself.
    pub effective_timing: super::timing::Timing,
    /// The healthcare event  (e.g. a patient and healthcare provider interaction)
    /// during which this observation is made.
    pub encounter: super::reference::Reference,
    /// The actual focus of an observation when it is not the patient of record
    /// representing something or someone associated with the patient such as a
    /// spouse, parent, fetus, or donor. For example, fetus observations in a mother's
    /// record.  The focus of an observation could also be an existing condition,  an
    /// intervention, the subject's diet,  another observation of the subject,  or a
    /// body structure such as tumor or implanted device.   An example use case would
    /// be using the Observation resource to capture whether the mother is trained to
    /// change her child's tracheostomy tube. In this example, the child is the patient
    /// of record and the mother is the focus.
    pub focus: Vec<super::reference::Reference>,
    /// This observation is a group observation (e.g. a battery, a panel of tests, a set
    /// of vital sign measurements) that includes the target as a member of the group.
    pub has_member: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A unique identifier assigned to this observation.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The reference to a FHIR ObservationDefinition resource that provides the
    /// definition that is adhered to in whole or in part by this Observation instance.
    pub instantiates_canonical: String,
    /// The reference to a FHIR ObservationDefinition resource that provides the
    /// definition that is adhered to in whole or in part by this Observation instance.
    pub instantiates_reference: super::reference::Reference,
    /// A categorical assessment of an observation value.  For example, high, low,
    /// normal.
    pub interpretation: Vec<super::codeable_concept::CodeableConcept>,
    /// The date and time this version of the observation was made available to
    /// providers, typically after the results have been reviewed and verified.
    pub issued: super::instant::Instant,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// Indicates the mechanism used to perform the observation.
    pub method: super::codeable_concept::CodeableConcept,
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
    /// Comments about the observation or the results.
    pub note: Vec<super::annotation::Annotation>,
    /// A larger event of which this particular Observation is a component or step.  For
    /// example,  an observation as part of a procedure.
    pub part_of: Vec<super::reference::Reference>,
    /// Who was responsible for asserting the observed value as "true".
    pub performer: Vec<super::reference::Reference>,
    /// Guidance on how to interpret the value by comparison to a normal or recommended
    /// range.  Multiple reference ranges are interpreted as an "OR".   In other words,
    /// to represent two distinct target populations, two `referenceRange` elements
    /// would be used.
    pub reference_range: Vec<super::observation::ObservationReferenceRange>,
    /// This is a Observation resource
    pub resource_type: String,
    /// The specimen that was used when this observation was made.
    pub specimen: super::reference::Reference,
    /// The status of the result value.
    pub status: super::code::Code,
    /// The patient, or group of patients, location, device, organization, procedure
    /// or practitioner this observation is about and into whose or what record the
    /// observation is placed. If the actual focus of the observation is different
    /// from the subject (or a sample of, part, or region of the subject), the `focus`
    /// element or the `code` itself specifies the actual focus of the observation.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Identifies the observation(s) that triggered the performance of this
    /// observation.
    pub triggered_by: Vec<super::observation::ObservationTriggeredBy>,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_attachment: super::attachment::Attachment,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_boolean: bool,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_date_time: String,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_integer: i64,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_period: super::period::Period,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_quantity: super::quantity::Quantity,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_range: super::range::Range,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_ratio: super::ratio::Ratio,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_reference: super::reference::Reference,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_sampled_data: super::sampled_data::SampledData,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_string: String,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_time: String,
}

/// Measurements and simple assertions made about a patient, device or other
/// subject.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationComponent {
    /// Describes what was observed. Sometimes this is called the observation "code".
    pub code: super::codeable_concept::CodeableConcept,
    /// Provides a reason why the expected value in the element
    /// Observation.component.value[x] is missing.
    pub data_absent_reason: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A categorical assessment of an observation value.  For example, high, low,
    /// normal.
    pub interpretation: Vec<super::codeable_concept::CodeableConcept>,
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
    /// Guidance on how to interpret the value by comparison to a normal or recommended
    /// range.
    pub reference_range: Vec<super::observation::ObservationReferenceRange>,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_attachment: super::attachment::Attachment,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_boolean: bool,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_date_time: String,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_integer: i64,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_period: super::period::Period,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_quantity: super::quantity::Quantity,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_range: super::range::Range,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_ratio: super::ratio::Ratio,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_reference: super::reference::Reference,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_sampled_data: super::sampled_data::SampledData,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_string: String,
    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub value_time: String,
}

/// Measurements and simple assertions made about a patient, device or other
/// subject.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationReferenceRange {
    /// The age at which this reference range is applicable. This is a neonatal age
    /// (e.g. number of weeks at term) if the meaning says so.
    pub age: super::range::Range,
    /// Codes to indicate the target population this reference range applies to.  For
    /// example, a reference range may be based on the normal population or a particular
    /// sex or race.  Multiple `appliesTo`  are interpreted as an "AND" of the target
    /// populations.  For example, to represent a target population of African American
    /// females, both a code of female and a code for African American would be used.
    pub applies_to: Vec<super::codeable_concept::CodeableConcept>,
    /// The value of the high bound of the reference range.  The high bound of the
    /// reference range endpoint is inclusive of the value (e.g.  reference range is
    /// >=5 - <=9). If the high bound is omitted,  it is assumed to be meaningless (e.g.
    /// reference range is >= 2.3).
    pub high: super::quantity::Quantity,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The value of the low bound of the reference range.  The low bound of the
    /// reference range endpoint is inclusive of the value (e.g.  reference range is
    /// >=5 - <=9). If the low bound is omitted,  it is assumed to be meaningless (e.g.
    /// reference range is <=2.3).
    pub low: super::quantity::Quantity,
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
    /// The value of the normal value of the reference range.
    pub normal_value: super::codeable_concept::CodeableConcept,
    /// Text based reference range in an observation which may be used when a
    /// quantitative range is not appropriate for an observation.  An example would be a
    /// reference value of "Negative" or a list or table of "normals".
    pub text: super::markdown::Markdown,
    /// Codes to indicate the what part of the targeted reference population it applies
    /// to. For example, the normal or therapeutic range.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// Measurements and simple assertions made about a patient, device or other
/// subject.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationTriggeredBy {
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
    /// Reference to the triggering observation.
    pub observation: super::reference::Reference,
    /// Provides the reason why this observation was performed as a result of the
    /// observation(s) referenced.
    pub reason: super::string::String,
    /// The type of trigger.
    /// Reflex | Repeat | Re-run.
    pub r#type: super::code::Code,
}
