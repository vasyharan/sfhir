/// Significant health conditions for a person related to the patient relevant in
/// the context of care for the patient.
#[derive(Debug, Clone, PartialEq)]
pub struct FamilyMemberHistory {
    /// The age of the relative at the time the family member history is recorded.
    pub age_age: super::age::Age,
    /// The age of the relative at the time the family member history is recorded.
    pub age_range: super::range::Range,
    /// The age of the relative at the time the family member history is recorded.
    pub age_string: String,
    /// The actual or approximate date of birth of the relative.
    pub born_date: String,
    /// The actual or approximate date of birth of the relative.
    pub born_period: super::period::Period,
    /// The actual or approximate date of birth of the relative.
    pub born_string: String,
    /// The significant Conditions (or condition) that the family member had. This
    /// is a repeating section to allow a system to represent more than one condition
    /// per resource, though there is nothing stopping multiple resources - one per
    /// condition.
    pub condition: Vec<super::family_member_history::FamilyMemberHistoryCondition>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Describes why the family member's history is not available.
    pub data_absent_reason: super::codeable_concept::CodeableConcept,
    /// The date (and possibly time) when the family member history was recorded or
    /// last updated.
    pub date: super::date_time::DateTime,
    /// Deceased flag or the actual or approximate age of the relative at the time of
    /// death for the family member history record.
    pub deceased_age: super::age::Age,
    /// Deceased flag or the actual or approximate age of the relative at the time of
    /// death for the family member history record.
    pub deceased_boolean: bool,
    /// Deceased flag or the actual or approximate age of the relative at the time of
    /// death for the family member history record.
    pub deceased_date: String,
    /// Deceased flag or the actual or approximate age of the relative at the time of
    /// death for the family member history record.
    pub deceased_range: super::range::Range,
    /// Deceased flag or the actual or approximate age of the relative at the time of
    /// death for the family member history record.
    pub deceased_string: String,
    /// If true, indicates that the age value specified is an estimated value.
    pub estimated_age: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this family member history by the performer or
    /// other systems which remain constant as the resource is updated and propagates
    /// from server to server.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this FamilyMemberHistory.
    pub instantiates_canonical: Vec<super::canonical::Canonical>,
    /// The URL pointing to an externally maintained protocol, guideline, orderset
    /// or other definition that is adhered to in whole or in part by this
    /// FamilyMemberHistory.
    pub instantiates_uri: Vec<super::uri::Uri>,
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
    /// This will either be a name or a description; e.g. "Aunt Susan", "my cousin with
    /// the red hair".
    pub name: super::string::String,
    /// This property allows a non condition-specific note to the made about the related
    /// person. Ideally, the note would be in the condition property, but this is not
    /// always possible.
    pub note: Vec<super::annotation::Annotation>,
    /// Indicates who or what participated in the activities related to the family
    /// member history and how they were involved.
    pub participant: Vec<super::family_member_history::FamilyMemberHistoryParticipant>,
    /// The person who this history concerns.
    pub patient: super::reference::Reference,
    /// The significant Procedures (or procedure) that the family member had. This
    /// is a repeating section to allow a system to represent more than one procedure
    /// per resource, though there is nothing stopping multiple resources - one per
    /// procedure.
    pub procedure: Vec<super::family_member_history::FamilyMemberHistoryProcedure>,
    /// Describes why the family member history occurred in coded or textual form, or
    /// Indicates a Condition, Observation, AllergyIntolerance, or QuestionnaireResponse
    /// that justifies this family member history event.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// The type of relationship this person has to the patient (father, mother, brother
    /// etc.).
    pub relationship: super::codeable_concept::CodeableConcept,
    /// This is a FamilyMemberHistory resource
    pub resource_type: String,
    /// The birth sex of the family member.
    pub sex: super::codeable_concept::CodeableConcept,
    /// A code specifying the status of the record of the family history of a specific
    /// family member.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// Significant health conditions for a person related to the patient relevant in
/// the context of care for the patient.
#[derive(Debug, Clone, PartialEq)]
pub struct FamilyMemberHistoryCondition {
    /// The actual condition specified. Could be a coded condition (like MI or Diabetes)
    /// or a less specific string like 'cancer' depending on how much is known about the
    /// condition and the capabilities of the creating system.
    pub code: super::codeable_concept::CodeableConcept,
    /// This condition contributed to the cause of death of the related person. If
    /// contributedToDeath is not populated, then it is unknown.
    pub contributed_to_death: super::boolean::Boolean,
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
    /// An area where general notes can be placed about this specific condition.
    pub note: Vec<super::annotation::Annotation>,
    /// Either the age of onset, range of approximate age or descriptive string can be
    /// recorded.  For conditions with multiple occurrences, this describes the first
    /// known occurrence.
    pub onset_age: super::age::Age,
    /// Either the age of onset, range of approximate age or descriptive string can be
    /// recorded.  For conditions with multiple occurrences, this describes the first
    /// known occurrence.
    pub onset_period: super::period::Period,
    /// Either the age of onset, range of approximate age or descriptive string can be
    /// recorded.  For conditions with multiple occurrences, this describes the first
    /// known occurrence.
    pub onset_range: super::range::Range,
    /// Either the age of onset, range of approximate age or descriptive string can be
    /// recorded.  For conditions with multiple occurrences, this describes the first
    /// known occurrence.
    pub onset_string: String,
    /// Indicates what happened following the condition.  If the condition resulted in
    /// death, deceased date is captured on the relation.
    pub outcome: super::codeable_concept::CodeableConcept,
}

/// Significant health conditions for a person related to the patient relevant in
/// the context of care for the patient.
#[derive(Debug, Clone, PartialEq)]
pub struct FamilyMemberHistoryParticipant {
    /// Indicates who or what participated in the activities related to the family
    /// member history.
    pub actor: super::reference::Reference,
    /// Distinguishes the type of involvement of the actor in the activities related to
    /// the family member history.
    pub function: super::codeable_concept::CodeableConcept,
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

/// Significant health conditions for a person related to the patient relevant in
/// the context of care for the patient.
#[derive(Debug, Clone, PartialEq)]
pub struct FamilyMemberHistoryProcedure {
    /// The actual procedure specified. Could be a coded procedure or a less specific
    /// string depending on how much is known about the procedure and the capabilities
    /// of the creating system.
    pub code: super::codeable_concept::CodeableConcept,
    /// This procedure contributed to the cause of death of the related person. If
    /// contributedToDeath is not populated, then it is unknown.
    pub contributed_to_death: super::boolean::Boolean,
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
    /// An area where general notes can be placed about this specific procedure.
    pub note: Vec<super::annotation::Annotation>,
    /// Indicates what happened following the procedure. If the procedure resulted in
    /// death, deceased date is captured on the relation.
    pub outcome: super::codeable_concept::CodeableConcept,
    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed. Allows a period to support complex procedures that span more than one
    /// date, and also allows for the length of the procedure to be captured.
    pub performed_age: super::age::Age,
    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed. Allows a period to support complex procedures that span more than one
    /// date, and also allows for the length of the procedure to be captured.
    pub performed_date_time: String,
    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed. Allows a period to support complex procedures that span more than one
    /// date, and also allows for the length of the procedure to be captured.
    pub performed_period: super::period::Period,
    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed. Allows a period to support complex procedures that span more than one
    /// date, and also allows for the length of the procedure to be captured.
    pub performed_range: super::range::Range,
    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed. Allows a period to support complex procedures that span more than one
    /// date, and also allows for the length of the procedure to be captured.
    pub performed_string: String,
}
