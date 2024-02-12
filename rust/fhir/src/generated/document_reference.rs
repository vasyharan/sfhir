/// A reference to a document of any kind for any purpose. While the term “document”
/// implies a more narrow focus, for this resource this "document" encompasses *any*
/// serialized object with a mime-type, it includes formal patient-centric documents
/// (CDA), clinical notes, scanned paper, non-patient specific documents like
/// policy text, as well as a photo, video, or audio recording acquired or used in
/// healthcare.  The DocumentReference resource provides metadata about the document
/// so that the document can be discovered and managed.  The actual content may be
/// inline base64 encoded data or provided by direct reference.
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentReference {
    /// A participant who has authenticated the accuracy of the document.
    pub attester: Vec<super::document_reference::DocumentReferenceAttester>,
    /// Identifies who is responsible for adding the information to the document.
    pub author: Vec<super::reference::Reference>,
    /// A procedure that is fulfilled in whole or in part by the creation of this media.
    pub based_on: Vec<super::reference::Reference>,
    /// The anatomic structures included in the document.
    pub body_site: Vec<super::codeable_reference::CodeableReference>,
    /// A categorization for the type of document referenced - helps for indexing and
    /// searching. This may be implied by or derived from the code specified in the
    /// DocumentReference.type.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The document and format referenced.  If there are multiple content element
    /// repetitions, these must all represent the same document in different format, or
    /// attachment metadata.
    pub content: Vec<super::document_reference::DocumentReferenceContent>,
    /// Describes the clinical encounter or type of care that the document content is
    /// associated with.
    pub context: Vec<super::reference::Reference>,
    /// Identifies the organization or group who is responsible for ongoing maintenance
    /// of and access to the document.
    pub custodian: super::reference::Reference,
    /// When the document reference was created.
    pub date: super::instant::Instant,
    /// Human-readable description of the source document.
    pub description: super::markdown::Markdown,
    /// The status of the underlying document.
    pub doc_status: super::code::Code,
    /// This list of codes represents the main clinical acts, such as a colonoscopy or
    /// an appendectomy, being documented. In some cases, the event is inherent in the
    /// type Code, such as a "History and Physical Report" in which the procedure being
    /// documented is necessarily a "History and Physical" act.
    pub event: Vec<super::codeable_reference::CodeableReference>,
    /// The kind of facility where the patient was seen.
    pub facility_type: super::codeable_concept::CodeableConcept,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Other business identifiers associated with the document, including version
    /// independent identifiers.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// Imaging modality used. This may include both acquisition and non-acquisition
    /// modalities.
    pub modality: Vec<super::codeable_concept::CodeableConcept>,
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
    /// The time period over which the service that is described by the document was
    /// provided.
    pub period: super::period::Period,
    /// This property may convey specifics about the practice setting where the content
    /// was created, often reflecting the clinical specialty.
    pub practice_setting: super::codeable_concept::CodeableConcept,
    /// Relationships that this document has with other document references that already
    /// exist.
    pub relates_to: Vec<super::document_reference::DocumentReferenceRelatesTo>,
    /// This is a DocumentReference resource
    pub resource_type: String,
    /// A set of Security-Tag codes specifying the level of privacy/security of
    /// the Document found at DocumentReference.content.attachment.url. Note that
    /// DocumentReference.meta.security contains the security labels of the data
    /// elements in DocumentReference, while DocumentReference.securityLabel contains
    /// the security labels for the document the reference refers to. The distinction
    /// recognizes that the document may contain sensitive information, while the
    /// DocumentReference is metadata about the document and thus might not be as
    /// sensitive as the document. For example: a psychotherapy episode may contain
    /// highly sensitive information, while the metadata may simply indicate that some
    /// episode happened.
    pub security_label: Vec<super::codeable_concept::CodeableConcept>,
    /// The status of this document reference.
    pub status: super::code::Code,
    /// Who or what the document is about. The document can be about a person, (patient
    /// or healthcare practitioner), a device (e.g. a machine) or even a group of
    /// subjects (such as a document about a herd of farm animals, or a set of patients
    /// that share a common exposure).
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Specifies the particular kind of document referenced  (e.g. History and
    /// Physical, Discharge Summary, Progress Note). This usually equates to the purpose
    /// of making the document referenced.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// An explicitly assigned identifer of a variation of the content in the
    /// DocumentReference.
    pub version: super::string::String,
}

/// A reference to a document of any kind for any purpose. While the term “document”
/// implies a more narrow focus, for this resource this "document" encompasses *any*
/// serialized object with a mime-type, it includes formal patient-centric documents
/// (CDA), clinical notes, scanned paper, non-patient specific documents like
/// policy text, as well as a photo, video, or audio recording acquired or used in
/// healthcare.  The DocumentReference resource provides metadata about the document
/// so that the document can be discovered and managed.  The actual content may be
/// inline base64 encoded data or provided by direct reference.
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentReferenceAttester {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The type of attestation the authenticator offers.
    pub mode: super::codeable_concept::CodeableConcept,
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
    /// Who attested the document in the specified way.
    pub party: super::reference::Reference,
    /// When the document was attested by the party.
    pub time: super::date_time::DateTime,
}

/// A reference to a document of any kind for any purpose. While the term “document”
/// implies a more narrow focus, for this resource this "document" encompasses *any*
/// serialized object with a mime-type, it includes formal patient-centric documents
/// (CDA), clinical notes, scanned paper, non-patient specific documents like
/// policy text, as well as a photo, video, or audio recording acquired or used in
/// healthcare.  The DocumentReference resource provides metadata about the document
/// so that the document can be discovered and managed.  The actual content may be
/// inline base64 encoded data or provided by direct reference.
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentReferenceContent {
    /// The document or URL of the document along with critical metadata to prove
    /// content has integrity.
    pub attachment: super::attachment::Attachment,
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
    /// An identifier of the document constraints, encoding, structure, and template
    /// that the document conforms to beyond the base format indicated in the mimeType.
    pub profile: Vec<super::document_reference::DocumentReferenceProfile>,
}

/// A reference to a document of any kind for any purpose. While the term “document”
/// implies a more narrow focus, for this resource this "document" encompasses *any*
/// serialized object with a mime-type, it includes formal patient-centric documents
/// (CDA), clinical notes, scanned paper, non-patient specific documents like
/// policy text, as well as a photo, video, or audio recording acquired or used in
/// healthcare.  The DocumentReference resource provides metadata about the document
/// so that the document can be discovered and managed.  The actual content may be
/// inline base64 encoded data or provided by direct reference.
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentReferenceProfile {
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
    /// Code|uri|canonical.
    pub value_canonical: String,
    /// Code|uri|canonical.
    pub value_coding: super::coding::Coding,
    /// Code|uri|canonical.
    pub value_uri: String,
}

/// A reference to a document of any kind for any purpose. While the term “document”
/// implies a more narrow focus, for this resource this "document" encompasses *any*
/// serialized object with a mime-type, it includes formal patient-centric documents
/// (CDA), clinical notes, scanned paper, non-patient specific documents like
/// policy text, as well as a photo, video, or audio recording acquired or used in
/// healthcare.  The DocumentReference resource provides metadata about the document
/// so that the document can be discovered and managed.  The actual content may be
/// inline base64 encoded data or provided by direct reference.
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentReferenceRelatesTo {
    /// The type of relationship that this document has with anther document.
    pub code: super::codeable_concept::CodeableConcept,
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
    /// The target document of this relationship.
    pub target: super::reference::Reference,
}
