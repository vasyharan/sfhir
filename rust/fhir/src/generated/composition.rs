/// A set of healthcare-related information that is assembled together into a
/// single logical package that provides a single coherent statement of meaning,
/// establishes its own context and that has clinical attestation with regard to
/// who is making the statement. A Composition defines the structure and narrative
/// content necessary for a document. However, a Composition alone does not
/// constitute a document. Rather, the Composition must be the first entry in a
/// Bundle where Bundle.type=document, and any other resources referenced from
/// Composition must be included as subsequent entries in the Bundle (for example
/// Patient, Practitioner, Encounter, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct Composition {
    /// A participant who has attested to the accuracy of the composition/document.
    pub attester: Vec<super::composition::CompositionAttester>,
    /// Identifies who is responsible for the information in the composition, not
    /// necessarily who typed it in.
    pub author: Vec<super::reference::Reference>,
    /// A categorization for the type of the composition - helps for indexing and
    /// searching. This may be implied by or derived from the code specified in the
    /// Composition Type.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Identifies the organization or group who is responsible for ongoing maintenance
    /// of and access to the composition/document information.
    pub custodian: super::reference::Reference,
    /// The composition editing time, when the composition was last logically changed by
    /// the author.
    pub date: super::date_time::DateTime,
    /// Describes the clinical encounter or type of care this documentation is
    /// associated with.
    pub encounter: super::reference::Reference,
    /// The clinical service, such as a colonoscopy or an appendectomy, being
    /// documented.
    pub event: Vec<super::composition::CompositionEvent>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A version-independent identifier for the Composition. This identifier stays
    /// constant as the composition is changed over time.
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
    /// A natural language name identifying the {{title}}. This name should be usable
    /// as an identifier for the module by machine processing applications such as code
    /// generation.
    pub name: super::string::String,
    /// For any additional notes.
    pub note: Vec<super::annotation::Annotation>,
    /// Relationships that this composition has with other compositions or documents
    /// that already exist.
    pub relates_to: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a Composition resource
    pub resource_type: String,
    /// The root of the sections that make up the composition.
    pub section: Vec<super::composition::CompositionSection>,
    /// The workflow/clinical status of this composition. The status is a marker for the
    /// clinical standing of the document.
    pub status: super::code::Code,
    /// Who or what the composition is about. The composition can be about a person,
    /// (patient or healthcare practitioner), a device (e.g. a machine) or even a group
    /// of subjects (such as a document about a herd of livestock, or a set of patients
    /// that share a common exposure).
    pub subject: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Official human-readable label for the composition.
    pub title: super::string::String,
    /// Specifies the particular kind of composition (e.g. History and Physical,
    /// Discharge Summary, Progress Note). This usually equates to the purpose of making
    /// the composition.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// An absolute URI that is used to identify this Composition when it is referenced
    /// in a specification, model, design or an instance; also called its canonical
    /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
    /// which an authoritative instance of this Composition is (or will be) published.
    /// This URL can be the target of a canonical reference. It SHALL remain the same
    /// when the Composition is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...)
    /// or may be references to specific programs (insurance plans, studies, ...) and
    /// may be used to assist with indexing and searching for appropriate Composition
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// An explicitly assigned identifer of a variation of the content in the
    /// Composition.
    pub version: super::string::String,
}

/// A set of healthcare-related information that is assembled together into a
/// single logical package that provides a single coherent statement of meaning,
/// establishes its own context and that has clinical attestation with regard to
/// who is making the statement. A Composition defines the structure and narrative
/// content necessary for a document. However, a Composition alone does not
/// constitute a document. Rather, the Composition must be the first entry in a
/// Bundle where Bundle.type=document, and any other resources referenced from
/// Composition must be included as subsequent entries in the Bundle (for example
/// Patient, Practitioner, Encounter, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct CompositionAttester {
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
    /// Who attested the composition in the specified way.
    pub party: super::reference::Reference,
    /// When the composition was attested by the party.
    pub time: super::date_time::DateTime,
}

/// A set of healthcare-related information that is assembled together into a
/// single logical package that provides a single coherent statement of meaning,
/// establishes its own context and that has clinical attestation with regard to
/// who is making the statement. A Composition defines the structure and narrative
/// content necessary for a document. However, a Composition alone does not
/// constitute a document. Rather, the Composition must be the first entry in a
/// Bundle where Bundle.type=document, and any other resources referenced from
/// Composition must be included as subsequent entries in the Bundle (for example
/// Patient, Practitioner, Encounter, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct CompositionEvent {
    /// Represents the main clinical acts, such as a colonoscopy or an appendectomy,
    /// being documented. In some cases, the event is inherent in the typeCode, such as
    /// a "History and Physical Report" in which case the procedure being documented is
    /// necessarily a "History and Physical" act. The events may be included as a code
    /// or as a reference to an other resource.
    pub detail: Vec<super::codeable_reference::CodeableReference>,
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
    /// The period of time covered by the documentation. There is no assertion that
    /// the documentation is a complete representation for this period, only that it
    /// documents events during this time.
    pub period: super::period::Period,
}

/// A set of healthcare-related information that is assembled together into a
/// single logical package that provides a single coherent statement of meaning,
/// establishes its own context and that has clinical attestation with regard to
/// who is making the statement. A Composition defines the structure and narrative
/// content necessary for a document. However, a Composition alone does not
/// constitute a document. Rather, the Composition must be the first entry in a
/// Bundle where Bundle.type=document, and any other resources referenced from
/// Composition must be included as subsequent entries in the Bundle (for example
/// Patient, Practitioner, Encounter, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct CompositionSection {
    /// Identifies who is responsible for the information in this section, not
    /// necessarily who typed it in.
    pub author: Vec<super::reference::Reference>,
    /// A code identifying the kind of content contained within the section. This must
    /// be consistent with the section title.
    pub code: super::codeable_concept::CodeableConcept,
    /// If the section is empty, why the list is empty. An empty section typically has
    /// some text explaining the empty reason.
    pub empty_reason: super::codeable_concept::CodeableConcept,
    /// A reference to the actual resource from which the narrative in the section is
    /// derived.
    pub entry: Vec<super::reference::Reference>,
    /// The actual focus of the section when it is not the subject of the composition,
    /// but instead represents something or someone associated with the subject such
    /// as (for a patient subject) a spouse, parent, fetus, or donor. If not focus is
    /// specified, the focus is assumed to be focus of the parent section, or, for a
    /// section in the Composition itself, the subject of the composition. Sections
    /// with a focus SHALL only include resources where the logical subject (patient,
    /// subject, focus, etc.) matches the section focus, or the resources have no
    /// logical subject (few resources).
    pub focus: super::reference::Reference,
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
    /// Specifies the order applied to the items in the section entries.
    pub ordered_by: super::codeable_concept::CodeableConcept,
    /// A nested sub-section within this section.
    pub section: Vec<super::composition::CompositionSection>,
    /// A human-readable narrative that contains the attested content of the section,
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative.
    pub text: super::narrative::Narrative,
    /// The label for this particular section.  This will be part of the rendered
    /// content for the document, and is often used to build a table of contents.
    pub title: super::string::String,
}
