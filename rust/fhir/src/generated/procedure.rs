/// An action that is or was performed on or for a patient, practitioner, device,
/// organization, or location. For example, this can be a physical intervention
/// on a patient like an operation, or less invasive like long term services,
/// counseling, or hypnotherapy.  This can be a quality or safety inspection for a
/// location, organization, or device.  This can be an accreditation procedure on a
/// practitioner for licensing.
#[derive(Debug, Clone, PartialEq)]
pub struct Procedure {
    /// A reference to a resource that contains details of the request for this
    /// procedure.
    pub based_on: Vec<super::reference::Reference>,
    /// Detailed and structured anatomical location information. Multiple locations are
    /// allowed - e.g. multiple punch biopsies of a lesion.
    pub body_site: Vec<super::codeable_concept::CodeableConcept>,
    /// A code that classifies the procedure for searching, sorting and display purposes
    /// (e.g. "Surgical Procedure").
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// The specific procedure that is performed. Use text if the exact nature of the
    /// procedure cannot be coded (e.g. "Laparoscopic Appendectomy").
    pub code: super::codeable_concept::CodeableConcept,
    /// Any complications that occurred during the procedure, or in the immediate post-
    /// performance period. These are generally tracked separately from the notes, which
    /// will typically describe the procedure itself rather than any 'post procedure'
    /// issues.
    pub complication: Vec<super::codeable_reference::CodeableReference>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The Encounter during which this Procedure was created or performed or to which
    /// the creation of this record is tightly associated.
    pub encounter: super::reference::Reference,
    /// A device that is implanted, removed or otherwise manipulated (calibration,
    /// battery replacement, fitting a prosthesis, attaching a wound-vac, etc.) as a
    /// focal portion of the Procedure.
    pub focal_device: Vec<super::procedure::ProcedureFocalDevice>,
    /// Who is the target of the procedure when it is not the subject of record only.
    /// If focus is not present, then subject is the focus.  If focus is present and the
    /// subject is one of the targets of the procedure, include subject as a focus as
    /// well. If focus is present and the subject is not included in focus, it implies
    /// that the procedure was only targeted on the focus. For example, when a caregiver
    /// is given education for a patient, the caregiver would be the focus and the
    /// procedure record is associated with the subject (e.g. patient).  For example,
    /// use focus when recording the target of the education, training, or counseling is
    /// the parent or relative of a patient.
    pub focus: super::reference::Reference,
    /// If the procedure required specific follow up - e.g. removal of sutures. The
    /// follow up may be represented as a simple note or could potentially be more
    /// complex, in which case the CarePlan resource can be used.
    pub follow_up: Vec<super::codeable_concept::CodeableConcept>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this procedure by the performer or other
    /// systems which remain constant as the resource is updated and is propagated from
    /// server to server.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The URL pointing to a FHIR-defined protocol, guideline, order set or other
    /// definition that is adhered to in whole or in part by this Procedure.
    pub instantiates_canonical: Vec<super::canonical::Canonical>,
    /// The URL pointing to an externally maintained protocol, guideline, order set or
    /// other definition that is adhered to in whole or in part by this Procedure.
    pub instantiates_uri: Vec<super::uri::Uri>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The location where the procedure actually happened.  E.g. a newborn at home, a
    /// tracheostomy at a restaurant.
    pub location: super::reference::Reference,
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
    /// Any other notes and comments about the procedure.
    pub note: Vec<super::annotation::Annotation>,
    /// Estimated or actual date, date-time, period, or age when the procedure did occur
    /// or is occurring.  Allows a period to support complex procedures that span more
    /// than one date, and also allows for the length of the procedure to be captured.
    pub occurrence_age: super::age::Age,
    /// Estimated or actual date, date-time, period, or age when the procedure did occur
    /// or is occurring.  Allows a period to support complex procedures that span more
    /// than one date, and also allows for the length of the procedure to be captured.
    pub occurrence_date_time: String,
    /// Estimated or actual date, date-time, period, or age when the procedure did occur
    /// or is occurring.  Allows a period to support complex procedures that span more
    /// than one date, and also allows for the length of the procedure to be captured.
    pub occurrence_period: super::period::Period,
    /// Estimated or actual date, date-time, period, or age when the procedure did occur
    /// or is occurring.  Allows a period to support complex procedures that span more
    /// than one date, and also allows for the length of the procedure to be captured.
    pub occurrence_range: super::range::Range,
    /// Estimated or actual date, date-time, period, or age when the procedure did occur
    /// or is occurring.  Allows a period to support complex procedures that span more
    /// than one date, and also allows for the length of the procedure to be captured.
    pub occurrence_string: String,
    /// Estimated or actual date, date-time, period, or age when the procedure did occur
    /// or is occurring.  Allows a period to support complex procedures that span more
    /// than one date, and also allows for the length of the procedure to be captured.
    pub occurrence_timing: super::timing::Timing,
    /// The outcome of the procedure - did it resolve the reasons for the procedure
    /// being performed?
    pub outcome: super::codeable_concept::CodeableConcept,
    /// A larger event of which this particular procedure is a component or step.
    pub part_of: Vec<super::reference::Reference>,
    /// Indicates who or what performed the procedure and how they were involved.
    pub performer: Vec<super::procedure::ProcedurePerformer>,
    /// The coded reason or reference why the procedure was performed. This may be
    /// a coded entity of some type, be present as text, or be a reference to one of
    /// several resources that justify the procedure.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// The date the occurrence of the procedure was first captured in the record
    /// regardless of Procedure.status (potentially after the occurrence of the event).
    pub recorded: super::date_time::DateTime,
    /// Individual who recorded the record and takes responsibility for its content.
    pub recorder: super::reference::Reference,
    /// This could be a histology result, pathology report, surgical report, etc.
    pub report: Vec<super::reference::Reference>,
    /// Indicates if this record was captured as a secondary 'reported' record rather
    /// than as an original primary source-of-truth record.  It may also indicate the
    /// source of the report.
    pub reported_boolean: bool,
    /// Indicates if this record was captured as a secondary 'reported' record rather
    /// than as an original primary source-of-truth record.  It may also indicate the
    /// source of the report.
    pub reported_reference: super::reference::Reference,
    /// This is a Procedure resource
    pub resource_type: String,
    /// A code specifying the state of the procedure. Generally, this will be the in-
    /// progress or completed state.
    pub status: super::code::Code,
    /// Captures the reason for the current state of the procedure.
    pub status_reason: super::codeable_concept::CodeableConcept,
    /// On whom or on what the procedure was performed. This is usually an individual
    /// human, but can also be performed on animals, groups of humans or animals,
    /// organizations or practitioners (for licensing), locations or devices (for safety
    /// inspections or regulatory authorizations).  If the actual focus of the procedure
    /// is different from the subject, the focus element specifies the actual focus of
    /// the procedure.
    pub subject: super::reference::Reference,
    /// Other resources from the patient record that may be relevant to the procedure.
    /// The information from these resources was either used to create the instance or
    /// is provided to help with its interpretation. This extension should not be used
    /// if more specific inline elements or extensions are available.
    pub supporting_info: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Identifies medications, devices and any other substance used as part of the
    /// procedure.
    pub used: Vec<super::codeable_reference::CodeableReference>,
}

/// An action that is or was performed on or for a patient, practitioner, device,
/// organization, or location. For example, this can be a physical intervention
/// on a patient like an operation, or less invasive like long term services,
/// counseling, or hypnotherapy.  This can be a quality or safety inspection for a
/// location, organization, or device.  This can be an accreditation procedure on a
/// practitioner for licensing.
#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureFocalDevice {
    /// The kind of change that happened to the device during the procedure.
    pub action: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The device that was manipulated (changed) during the procedure.
    pub manipulated: super::reference::Reference,
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

/// An action that is or was performed on or for a patient, practitioner, device,
/// organization, or location. For example, this can be a physical intervention
/// on a patient like an operation, or less invasive like long term services,
/// counseling, or hypnotherapy.  This can be a quality or safety inspection for a
/// location, organization, or device.  This can be an accreditation procedure on a
/// practitioner for licensing.
#[derive(Debug, Clone, PartialEq)]
pub struct ProcedurePerformer {
    /// Indicates who or what performed the procedure.
    pub actor: super::reference::Reference,
    /// Distinguishes the type of involvement of the performer in the procedure. For
    /// example, surgeon, anaesthetist, endoscopist.
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
    /// The Organization the Patient, RelatedPerson, Device, CareTeam, and
    /// HealthcareService was acting on behalf of.
    pub on_behalf_of: super::reference::Reference,
    /// Time period during which the performer performed the procedure.
    pub period: super::period::Period,
}
