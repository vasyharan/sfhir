/// The EvidenceReport Resource is a specialized container for a collection of
/// resources and codeable concepts, adapted to support compositions of Evidence,
/// EvidenceVariable, and Citation resources and related concepts.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceReport {
    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// An individiual, organization, or device primarily involved in the creation and
    /// maintenance of the content.
    pub author: Vec<super::contact_detail::ContactDetail>,
    /// Citation Resource or display of suggested citation for this report.
    pub cite_as_markdown: String,
    /// Citation Resource or display of suggested citation for this report.
    pub cite_as_reference: super::reference::Reference,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the {{title}} and/or its contents. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// {{title}}.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date (and optionally time) when the {{title}} was last significantly
    /// changed. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the {{title}} changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the {{title}} from a consumer's
    /// perspective.
    pub description: super::markdown::Markdown,
    /// An individiual, organization, or device primarily responsible for internal
    /// coherence of the content.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the {{title}} content was or is planned to be in active
    /// use.
    pub effective_period: super::period::Period,
    /// An individiual, organization, or device responsible for officially endorsing the
    /// content for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A Boolean value to indicate that this {{title}} is authored for testing purposes
    /// (or education/evaluation/marketing) and is not intended for genuine usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this EvidenceReport when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the {{title}} is intended to be used.
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
    /// A natural language name identifying the {{title}}. This name should be usable
    /// as an identifier for the resource by machine processing applications such as
    /// code generation.
    pub name: super::string::String,
    /// Used for footnotes and annotations.
    pub note: Vec<super::annotation::Annotation>,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the evidence report.
    pub publisher: super::string::String,
    /// Explanation of why this {{title}} is needed and why it has been designed as
    /// it has.
    pub purpose: super::markdown::Markdown,
    /// Link, description or reference to artifact associated with the report.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// A formal identifier that is used to identify things closely related to this
    /// EvidenceReport.
    pub related_identifier: Vec<super::identifier::Identifier>,
    /// Relationships that this composition has with other compositions or documents
    /// that already exist.
    pub relates_to: Vec<super::evidence_report::EvidenceReportRelatesTo>,
    /// This is a EvidenceReport resource
    pub resource_type: String,
    /// An individiual, organization, or device primarily responsible for review of some
    /// aspect of the content.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// The root of the sections that make up the composition.
    pub section: Vec<super::evidence_report::EvidenceReportSection>,
    /// The status of this summary. Enables tracking the life-cycle of the content.
    pub status: super::code::Code,
    /// Specifies the subject or focus of the report. Answers "What is this report
    /// about?".
    pub subject: super::evidence_report::EvidenceReportSubject,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the {{title}}.
    pub title: super::string::String,
    /// Descriptive topics related to the content of the {{title}}. Topics provide a
    /// high-level categorization as well as keywords for the {{title}} that can be
    /// useful for filtering and searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// Specifies the kind of report, such as grouping of classifiers, search results,
    /// or human-compiled expression.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// An absolute URI that is used to identify this EvidenceReport when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this summary is (or will be)
    /// published. This URL can be the target of a canonical reference. It SHALL remain
    /// the same when the summary is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate evidence report
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the {{title}} when
    /// it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the {{title}} author and is not expected to be
    /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence without additional knowledge.  (See the
    /// versionAlgorithm element.)
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// The EvidenceReport Resource is a specialized container for a collection of
/// resources and codeable concepts, adapted to support compositions of Evidence,
/// EvidenceVariable, and Citation resources and related concepts.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceReportCharacteristic {
    /// Characteristic code.
    pub code: super::codeable_concept::CodeableConcept,
    /// Is used to express not the characteristic.
    pub exclude: super::boolean::Boolean,
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
    /// Timeframe for the characteristic.
    pub period: super::period::Period,
    /// Characteristic value.
    pub value_boolean: bool,
    /// Characteristic value.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Characteristic value.
    pub value_quantity: super::quantity::Quantity,
    /// Characteristic value.
    pub value_range: super::range::Range,
    /// Characteristic value.
    pub value_reference: super::reference::Reference,
}

/// The EvidenceReport Resource is a specialized container for a collection of
/// resources and codeable concepts, adapted to support compositions of Evidence,
/// EvidenceVariable, and Citation resources and related concepts.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceReportRelatesTo {
    /// The type of relationship that this composition has with anther composition or
    /// document.
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
    /// The target composition/document of this relationship.
    pub target: super::evidence_report::EvidenceReportTarget,
}

/// The EvidenceReport Resource is a specialized container for a collection of
/// resources and codeable concepts, adapted to support compositions of Evidence,
/// EvidenceVariable, and Citation resources and related concepts.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceReportSection {
    /// Identifies who is responsible for the information in this section, not
    /// necessarily who typed it in.
    pub author: Vec<super::reference::Reference>,
    /// If the section is empty, why the list is empty. An empty section typically has
    /// some text explaining the empty reason.
    pub empty_reason: super::codeable_concept::CodeableConcept,
    /// Specifies any type of classification of the evidence report.
    pub entry_classifier: Vec<super::codeable_concept::CodeableConcept>,
    /// Quantity as content.
    pub entry_quantity: Vec<super::quantity::Quantity>,
    /// A reference to the actual resource from which the narrative in the section is
    /// derived.
    pub entry_reference: Vec<super::reference::Reference>,
    /// A code identifying the kind of content contained within the section. This should
    /// be consistent with the section title.
    pub focus: super::codeable_concept::CodeableConcept,
    /// A definitional Resource identifying the kind of content contained within the
    /// section. This should be consistent with the section title.
    pub focus_reference: super::reference::Reference,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// How the entry list was prepared - whether it is a working list that is suitable
    /// for being maintained on an ongoing basis, or if it represents a snapshot of a
    /// list of items from another source, or whether it is a prepared list where items
    /// may be marked as added, modified or deleted.
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
    /// Specifies the order applied to the items in the section entries.
    pub ordered_by: super::codeable_concept::CodeableConcept,
    /// A nested sub-section within this section.
    pub section: Vec<super::evidence_report::EvidenceReportSection>,
    /// A human-readable narrative that contains the attested content of the section,
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is peferred to contain sufficient detail to
    /// make it acceptable for a human to just read the narrative.
    pub text: super::narrative::Narrative,
    /// The label for this particular section.  This will be part of the rendered
    /// content for the document, and is often used to build a table of contents.
    pub title: super::string::String,
}

/// The EvidenceReport Resource is a specialized container for a collection of
/// resources and codeable concepts, adapted to support compositions of Evidence,
/// EvidenceVariable, and Citation resources and related concepts.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceReportSubject {
    /// Characteristic.
    pub characteristic: Vec<super::evidence_report::EvidenceReportCharacteristic>,
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
    /// Used for general notes and annotations not coded elsewhere.
    pub note: Vec<super::annotation::Annotation>,
}

/// The EvidenceReport Resource is a specialized container for a collection of
/// resources and codeable concepts, adapted to support compositions of Evidence,
/// EvidenceVariable, and Citation resources and related concepts.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceReportTarget {
    /// Target of the relationship Display.
    pub display: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Target of the relationship Identifier.
    pub identifier: super::identifier::Identifier,
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
    /// Target of the relationship Resource reference.
    pub resource: super::reference::Reference,
    /// Target of the relationship URL.
    pub url: super::uri::Uri,
}
