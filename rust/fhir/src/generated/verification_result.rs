/// Describes validation requirements, source(s), status and dates for one or more
/// elements.
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationResult {
    /// Information about the entity attesting to information.
    pub attestation: super::verification_result::VerificationResultAttestation,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The result if validation fails (fatal; warning; record only; none).
    pub failure_action: super::codeable_concept::CodeableConcept,
    /// Frequency of revalidation.
    pub frequency: super::timing::Timing,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The date/time validation was last completed (including failed validations).
    pub last_performed: super::date_time::DateTime,
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
    /// The frequency with which the target must be validated (none; initial; periodic).
    pub need: super::codeable_concept::CodeableConcept,
    /// The date when target is next validated, if appropriate.
    pub next_scheduled: super::date::Date,
    /// Information about the primary source(s) involved in validation.
    pub primary_source: Vec<super::verification_result::VerificationResultPrimarySource>,
    /// This is a VerificationResult resource
    pub resource_type: String,
    /// The validation status of the target (attested; validated; in process; requires
    /// revalidation; validation failed; revalidation failed).
    pub status: super::code::Code,
    /// When the validation status was updated.
    pub status_date: super::date_time::DateTime,
    /// A resource that was validated.
    pub target: Vec<super::reference::Reference>,
    /// The fhirpath location(s) within the resource that was validated.
    pub target_location: Vec<super::string::String>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// The primary process by which the target is validated (edit check; value set;
    /// primary source; multiple sources; standalone; in context).
    pub validation_process: Vec<super::codeable_concept::CodeableConcept>,
    /// What the target is validated against (nothing; primary source; multiple
    /// sources).
    pub validation_type: super::codeable_concept::CodeableConcept,
    /// Information about the entity validating information.
    pub validator: Vec<super::verification_result::VerificationResultValidator>,
}

/// Describes validation requirements, source(s), status and dates for one or more
/// elements.
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationResultAttestation {
    /// The method by which attested information was submitted/retrieved (manual; API;
    /// Push).
    pub communication_method: super::codeable_concept::CodeableConcept,
    /// The date the information was attested to.
    pub date: super::date::Date,
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
    /// When the who is asserting on behalf of another (organization or individual).
    pub on_behalf_of: super::reference::Reference,
    /// A digital identity certificate associated with the proxy entity submitting
    /// attested information on behalf of the attestation source.
    pub proxy_identity_certificate: super::string::String,
    /// Signed assertion by the proxy entity indicating that they have the right to
    /// submit attested information on behalf of the attestation source.
    pub proxy_signature: super::signature::Signature,
    /// A digital identity certificate associated with the attestation source.
    pub source_identity_certificate: super::string::String,
    /// Signed assertion by the attestation source that they have attested to the
    /// information.
    pub source_signature: super::signature::Signature,
    /// The individual or organization attesting to information.
    pub who: super::reference::Reference,
}

/// Describes validation requirements, source(s), status and dates for one or more
/// elements.
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationResultPrimarySource {
    /// Ability of the primary source to push updates/alerts (yes; no; undetermined).
    pub can_push_updates: super::codeable_concept::CodeableConcept,
    /// Method for communicating with the primary source (manual; API; Push).
    pub communication_method: Vec<super::codeable_concept::CodeableConcept>,
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
    /// Type of alerts/updates the primary source can send (specific requested changes;
    /// any changes; as defined by source).
    pub push_type_available: Vec<super::codeable_concept::CodeableConcept>,
    /// Type of primary source (License Board; Primary Education; Continuing Education;
    /// Postal Service; Relationship owner; Registration Authority; legal source;
    /// issuing source; authoritative source).
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
    /// When the target was validated against the primary source.
    pub validation_date: super::date_time::DateTime,
    /// Status of the validation of the target against the primary source (successful;
    /// failed; unknown).
    pub validation_status: super::codeable_concept::CodeableConcept,
    /// Reference to the primary source.
    pub who: super::reference::Reference,
}

/// Describes validation requirements, source(s), status and dates for one or more
/// elements.
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationResultValidator {
    /// Signed assertion by the validator that they have validated the information.
    pub attestation_signature: super::signature::Signature,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A digital identity certificate associated with the validator.
    pub identity_certificate: super::string::String,
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
    /// Reference to the organization validating information.
    pub organization: super::reference::Reference,
}
