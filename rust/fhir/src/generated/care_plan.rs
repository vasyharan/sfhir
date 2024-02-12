/// Describes the intention of how one or more practitioners intend to deliver care
/// for a particular patient, group or community for a period of time, possibly
/// limited to care for a specific condition or set of conditions.
#[derive(Debug, Clone, PartialEq)]
pub struct CarePlan {
    /// Identifies an action that has occurred or is a planned action to occur as part
    /// of the plan. For example, a medication to be used, lab tests to perform, self-
    /// monitoring that has occurred, education etc.
    pub activity: Vec<super::care_plan::CarePlanActivity>,
    /// Identifies the conditions/problems/concerns/diagnoses/etc. whose management and/
    /// or mitigation are handled by this plan.
    pub addresses: Vec<super::codeable_reference::CodeableReference>,
    /// A higher-level request resource (i.e. a plan, proposal or order) that is
    /// fulfilled in whole or in part by this care plan.
    pub based_on: Vec<super::reference::Reference>,
    /// Identifies all people and organizations who are expected to be involved in the
    /// care envisioned by this plan.
    pub care_team: Vec<super::reference::Reference>,
    /// Identifies what "kind" of plan this is to support differentiation between
    /// multiple co-existing plans; e.g. "Home health", "psychiatric", "asthma",
    /// "disease management", "wellness plan", etc.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Identifies the individual(s), organization or device who provided the contents
    /// of the care plan.
    pub contributor: Vec<super::reference::Reference>,
    /// Represents when this particular CarePlan record was created in the system, which
    /// is often a system-generated date.
    pub created: super::date_time::DateTime,
    /// When populated, the custodian is responsible for the care plan. The care plan is
    /// attributed to the custodian.
    pub custodian: super::reference::Reference,
    /// A description of the scope and nature of the plan.
    pub description: super::string::String,
    /// The Encounter during which this CarePlan was created or to which the creation of
    /// this record is tightly associated.
    pub encounter: super::reference::Reference,
    /// Describes the intended objective(s) of carrying out the care plan.
    pub goal: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this care plan by the performer or other
    /// systems which remain constant as the resource is updated and propagates from
    /// server to server.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The URL pointing to a FHIR-defined protocol, guideline, questionnaire or other
    /// definition that is adhered to in whole or in part by this CarePlan.
    pub instantiates_canonical: Vec<super::canonical::Canonical>,
    /// The URL pointing to an externally maintained protocol, guideline, questionnaire
    /// or other definition that is adhered to in whole or in part by this CarePlan.
    pub instantiates_uri: Vec<super::uri::Uri>,
    /// Indicates the level of authority/intentionality associated with the care plan
    /// and where the care plan fits into the workflow chain.
    pub intent: super::code::Code,
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
    /// General notes about the care plan not covered elsewhere.
    pub note: Vec<super::annotation::Annotation>,
    /// A larger care plan of which this particular care plan is a component or step.
    pub part_of: Vec<super::reference::Reference>,
    /// Indicates when the plan did (or is intended to) come into effect and end.
    pub period: super::period::Period,
    /// Completed or terminated care plan whose function is taken by this new care plan.
    pub replaces: Vec<super::reference::Reference>,
    /// This is a CarePlan resource
    pub resource_type: String,
    /// Indicates whether the plan is currently being acted upon, represents future
    /// intentions or is now a historical record.
    pub status: super::code::Code,
    /// Identifies the patient or group whose intended care is described by the plan.
    pub subject: super::reference::Reference,
    /// Identifies portions of the patient's record that specifically influenced the
    /// formation of the plan.  These might include comorbidities, recent procedures,
    /// limitations, recent assessments, etc.
    pub supporting_info: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Human-friendly name for the care plan.
    pub title: super::string::String,
}

/// Describes the intention of how one or more practitioners intend to deliver care
/// for a particular patient, group or community for a period of time, possibly
/// limited to care for a specific condition or set of conditions.
#[derive(Debug, Clone, PartialEq)]
pub struct CarePlanActivity {
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
    /// Identifies the activity that was performed. For example, an activity could be
    /// patient education, exercise, or a medication administration. The reference to
    /// an "event" resource, such as Procedure or Encounter or Observation, represents
    /// the activity that was performed. The requested activity can be conveyed using
    /// the CarePlan.activity.plannedActivityReference (a reference to a “request”
    /// resource).
    pub performed_activity: Vec<super::codeable_reference::CodeableReference>,
    /// The details of the proposed activity represented in a specific resource.
    pub planned_activity_reference: super::reference::Reference,
    /// Notes about the adherence/status/progress of the activity.
    pub progress: Vec<super::annotation::Annotation>,
}
