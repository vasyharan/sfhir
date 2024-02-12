/// Describes the intended objective(s) for a patient, group or organization care,
/// for example, weight loss, restoring an activity of daily living, obtaining herd
/// immunity via immunization, meeting a process improvement objective, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Goal {
    /// Describes the progression, or lack thereof, towards the goal against the target.
    pub achievement_status: super::codeable_concept::CodeableConcept,
    /// The identified conditions and other health record elements that are intended to
    /// be addressed by the goal.
    pub addresses: Vec<super::reference::Reference>,
    /// Indicates a category the goal falls within.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// After meeting the goal, ongoing activity is needed to sustain the goal
    /// objective.
    pub continuous: super::boolean::Boolean,
    /// Human-readable and/or coded description of a specific desired objective of care,
    /// such as "control blood pressure" or "negotiate an obstacle course" or "dance
    /// with child at wedding".
    pub description: super::codeable_concept::CodeableConcept,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this goal by the performer or other systems
    /// which remain constant as the resource is updated and propagates from server
    /// to server.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The state of the goal throughout its lifecycle.
    pub lifecycle_status: super::code::Code,
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
    /// Any comments related to the goal.
    pub note: Vec<super::annotation::Annotation>,
    /// Identifies the change (or lack of change) at the point when the status of the
    /// goal is assessed.
    pub outcome: Vec<super::codeable_reference::CodeableReference>,
    /// Identifies the mutually agreed level of importance associated with reaching/
    /// sustaining the goal.
    pub priority: super::codeable_concept::CodeableConcept,
    /// This is a Goal resource
    pub resource_type: String,
    /// Indicates whose goal this is - patient goal, practitioner goal, etc.
    pub source: super::reference::Reference,
    /// The date or event after which the goal should begin being pursued.
    pub start_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The date or event after which the goal should begin being pursued.
    pub start_date: String,
    /// Identifies when the current status.  I.e. When initially created, when achieved,
    /// when cancelled, etc.
    pub status_date: super::date::Date,
    /// Captures the reason for the current status.
    pub status_reason: super::string::String,
    /// Identifies the patient, group or organization for whom the goal is being
    /// established.
    pub subject: super::reference::Reference,
    /// Indicates what should be done by when.
    pub target: Vec<super::goal::GoalTarget>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// Describes the intended objective(s) for a patient, group or organization care,
/// for example, weight loss, restoring an activity of daily living, obtaining herd
/// immunity via immunization, meeting a process improvement objective, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct GoalTarget {
    /// The target value of the focus to be achieved to signify the fulfillment of the
    /// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
    /// can be specified. When a low value is missing, it indicates that the goal is
    /// achieved at any focus value at or below the high value. Similarly, if the high
    /// value is missing, it indicates that the goal is achieved at any focus value at
    /// or above the low value.
    pub detail_boolean: bool,
    /// The target value of the focus to be achieved to signify the fulfillment of the
    /// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
    /// can be specified. When a low value is missing, it indicates that the goal is
    /// achieved at any focus value at or below the high value. Similarly, if the high
    /// value is missing, it indicates that the goal is achieved at any focus value at
    /// or above the low value.
    pub detail_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The target value of the focus to be achieved to signify the fulfillment of the
    /// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
    /// can be specified. When a low value is missing, it indicates that the goal is
    /// achieved at any focus value at or below the high value. Similarly, if the high
    /// value is missing, it indicates that the goal is achieved at any focus value at
    /// or above the low value.
    pub detail_integer: i64,
    /// The target value of the focus to be achieved to signify the fulfillment of the
    /// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
    /// can be specified. When a low value is missing, it indicates that the goal is
    /// achieved at any focus value at or below the high value. Similarly, if the high
    /// value is missing, it indicates that the goal is achieved at any focus value at
    /// or above the low value.
    pub detail_quantity: super::quantity::Quantity,
    /// The target value of the focus to be achieved to signify the fulfillment of the
    /// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
    /// can be specified. When a low value is missing, it indicates that the goal is
    /// achieved at any focus value at or below the high value. Similarly, if the high
    /// value is missing, it indicates that the goal is achieved at any focus value at
    /// or above the low value.
    pub detail_range: super::range::Range,
    /// The target value of the focus to be achieved to signify the fulfillment of the
    /// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
    /// can be specified. When a low value is missing, it indicates that the goal is
    /// achieved at any focus value at or below the high value. Similarly, if the high
    /// value is missing, it indicates that the goal is achieved at any focus value at
    /// or above the low value.
    pub detail_ratio: super::ratio::Ratio,
    /// The target value of the focus to be achieved to signify the fulfillment of the
    /// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
    /// can be specified. When a low value is missing, it indicates that the goal is
    /// achieved at any focus value at or below the high value. Similarly, if the high
    /// value is missing, it indicates that the goal is achieved at any focus value at
    /// or above the low value.
    pub detail_string: String,
    /// Indicates either the date or the duration after start by which the goal should
    /// be met.
    pub due_date: String,
    /// Indicates either the date or the duration after start by which the goal should
    /// be met.
    pub due_duration: super::duration::Duration,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The parameter whose value is being tracked, e.g. body weight, blood pressure, or
    /// hemoglobin A1c level.
    pub measure: super::codeable_concept::CodeableConcept,
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
