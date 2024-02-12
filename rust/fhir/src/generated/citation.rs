/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct Citation {
    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// Who authored or created the citation record.
    pub author: Vec<super::contact_detail::ContactDetail>,
    /// The article or artifact being described.
    pub cited_artifact: super::citation::CitationCitedArtifact,
    /// The assignment to an organizing scheme.
    pub classification: Vec<super::citation::CitationClassification>,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Use and/or publishing restrictions for the citation record, not for the cited
    /// artifact.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The status of the citation record.
    pub current_state: Vec<super::codeable_concept::CodeableConcept>,
    /// The date (and optionally time) when the citation record was last significantly
    /// changed. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the citation record changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the citation from a consumer's
    /// perspective.
    pub description: super::markdown::Markdown,
    /// Who edited or revised the citation record.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the citation record content was or is planned to be in
    /// active use.
    pub effective_period: super::period::Period,
    /// Who endorsed the citation record.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A Boolean value to indicate that this citation record is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this citation record when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the citation record is intended to be
    /// used.
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
    /// A natural language name identifying the citation record. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub name: super::string::String,
    /// Used for general notes and annotations not coded elsewhere.
    pub note: Vec<super::annotation::Annotation>,
    /// The name of the organization or individual that published the citation record.
    pub publisher: super::string::String,
    /// Explanation of why this citation is needed and why it has been designed as it
    /// has.
    pub purpose: super::markdown::Markdown,
    /// Artifact related to the citation record.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a Citation resource
    pub resource_type: String,
    /// Who reviewed the citation record.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// The status of this summary. Enables tracking the life-cycle of the content.
    pub status: super::code::Code,
    /// The state or status of the citation record paired with an effective date or
    /// period for that state.
    pub status_date: Vec<super::citation::CitationStatusDate>,
    /// A human-readable display of key concepts to represent the citation.
    pub summary: Vec<super::citation::CitationSummary>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the citation record.
    pub title: super::string::String,
    /// Descriptive topics related to the content of the {{title}}. Topics provide a
    /// high-level categorization as well as keywords for the {{title}} that can be
    /// useful for filtering and searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// An absolute URI that is used to identify this citation record when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this summary is (or will be)
    /// published. This URL can be the target of a canonical reference. It SHALL remain
    /// the same when the summary is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate citation record
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the citation record
    /// when it is referenced in a specification, model, design or instance. This is
    /// an arbitrary value managed by the citation record author and is not expected to
    /// be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationAbstract {
    /// Copyright notice for the abstract.
    pub copyright: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Used to express the specific language of the abstract.
    pub language: super::codeable_concept::CodeableConcept,
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
    /// Abstract content.
    pub text: super::markdown::Markdown,
    /// Used to express the reason for or classification of the abstract.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifact {
    /// The abstract may be used to convey article-contained abstracts, externally-
    /// created abstracts, or other descriptive summaries.
    pub r#abstract: Vec<super::citation::CitationAbstract>,
    /// The assignment to an organizing scheme.
    pub classification: Vec<super::citation::CitationClassification1>,
    /// This element is used to list authors and other contributors, their contact
    /// information, specific contributions, and summary statements.
    pub contributorship: super::citation::CitationContributorship,
    /// The status of the cited artifact.
    pub current_state: Vec<super::codeable_concept::CodeableConcept>,
    /// When the cited artifact was accessed.
    pub date_accessed: super::date_time::DateTime,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A formal identifier that is used to identify the cited artifact when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
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
    /// Any additional information or content for the article or artifact.
    pub note: Vec<super::annotation::Annotation>,
    /// The component of the article or artifact.
    pub part: super::citation::CitationPart,
    /// If multiple, used to represent alternative forms of the article that are not
    /// separate citations.
    pub publication_form: Vec<super::citation::CitationPublicationForm>,
    /// A formal identifier that is used to identify things closely related to the cited
    /// artifact.
    pub related_identifier: Vec<super::identifier::Identifier>,
    /// The artifact related to the cited artifact.
    pub relates_to: Vec<super::citation::CitationRelatesTo>,
    /// An effective date or period, historical or future, actual or expected, for a
    /// status of the cited artifact.
    pub status_date: Vec<super::citation::CitationStatusDate1>,
    /// The title details of the article or artifact.
    pub title: Vec<super::citation::CitationTitle>,
    /// The defined version of the cited artifact.
    pub version: super::citation::CitationVersion,
    /// Used for any URL for the article or artifact cited.
    pub web_location: Vec<super::citation::CitationWebLocation>,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationClassification {
    /// The specific classification value.
    pub classifier: Vec<super::codeable_concept::CodeableConcept>,
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
    /// The kind of classifier (e.g. publication type, keyword).
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationClassification1 {
    /// Complex or externally created classification.
    pub artifact_assessment: Vec<super::reference::Reference>,
    /// The specific classification value.
    pub classifier: Vec<super::codeable_concept::CodeableConcept>,
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
    /// The kind of classifier (e.g. publication type, keyword).
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationContributionInstance {
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
    /// The time that the contribution was made.
    pub time: super::date_time::DateTime,
    /// The specific contribution.
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationContributorship {
    /// Indicates if the list includes all authors and/or contributors.
    pub complete: super::boolean::Boolean,
    /// An individual entity named as a contributor, for example in the author list or
    /// contributor list.
    pub entry: Vec<super::citation::CitationEntry>,
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
    /// Used to record a display of the author/contributor list without separate data
    /// element for each list member.
    pub summary: Vec<super::citation::CitationSummary1>,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationEntry {
    /// Organization affiliated with the contributor.
    pub affiliation: Vec<super::reference::Reference>,
    /// Contributions with accounting for time or number.
    pub contribution_instance: Vec<super::citation::CitationContributionInstance>,
    /// This element identifies the specific nature of an individual’s contribution with
    /// respect to the cited work.
    pub contribution_type: Vec<super::codeable_concept::CodeableConcept>,
    /// The identity of the individual contributor.
    pub contributor: super::reference::Reference,
    /// Whether the contributor is the corresponding contributor for the role.
    pub corresponding_contact: super::boolean::Boolean,
    /// For citation styles that use initials.
    pub forename_initials: super::string::String,
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
    /// Provides a numerical ranking to represent the degree of contributorship relative
    /// to other contributors, such as 1 for first author and 2 for second author.
    pub ranking_order: super::positive_int::PositiveInt,
    /// The role of the contributor (e.g. author, editor, reviewer, funder).
    pub role: super::codeable_concept::CodeableConcept,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationPart {
    /// The citation for the full article or artifact.
    pub base_citation: super::reference::Reference,
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
    /// The kind of component.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The specification of the component.
    pub value: super::string::String,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationPublicationForm {
    /// Entry number or identifier for inclusion in a database.
    pub accession_number: super::string::String,
    /// The date the article was added to the database, or the date the article was
    /// released.
    pub article_date: super::date_time::DateTime,
    /// Describes the form of the medium cited. Common codes are "Internet" or "Print".
    /// The CitedMedium value set has 6 codes. The codes internet, print, and offline-
    /// digital-storage are the common codes for a typical publication form, though
    /// internet and print are more common for study citations. Three additional codes
    /// (each appending one of the primary codes with "-without-issue" are used for
    /// situations when a study is published both within an issue (of a periodical
    /// release as commonly done for journals) AND is published separately from the
    /// issue (as commonly done with early online publication), to represent specific
    /// identification of the publication form not associated with the issue.
    pub cited_medium: super::codeable_concept::CodeableConcept,
    /// Copyright notice for the full article or artifact.
    pub copyright: super::markdown::Markdown,
    /// Used for isolated representation of first page.
    pub first_page: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Issue, part or supplement of journal or other collection in which the article
    /// is published.
    pub issue: super::string::String,
    /// The language or languages in which this form of the article is published.
    pub language: Vec<super::codeable_concept::CodeableConcept>,
    /// Used for isolated representation of last page.
    pub last_page: super::string::String,
    /// The date the article was last revised or updated in the database.
    pub last_revision_date: super::date_time::DateTime,
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
    /// Actual or approximate number of pages or screens. Distinct from reporting the
    /// page numbers.
    pub page_count: super::string::String,
    /// Used for full display of pagination.
    pub page_string: super::string::String,
    /// Spring, Summer, Fall/Autumn, Winter.
    pub publication_date_season: super::string::String,
    /// Text representation of the date on which the issue of the cited artifact was
    /// published.
    pub publication_date_text: super::string::String,
    /// The collection the cited article or artifact is published in.
    pub published_in: super::citation::CitationPublishedIn,
    /// Volume number of journal or other collection in which the article is published.
    pub volume: super::string::String,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationPublishedIn {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Journal identifiers include ISSN, ISO Abbreviation and NLMuniqueID; Book
    /// identifiers include ISBN.
    pub identifier: Vec<super::identifier::Identifier>,
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
    /// Name of or resource describing the publisher.
    pub publisher: super::reference::Reference,
    /// Geographic location of the publisher.
    pub publisher_location: super::string::String,
    /// Name of the database or title of the book or journal.
    pub title: super::string::String,
    /// Kind of container (e.g. Periodical, database, or book).
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationRelatesTo {
    /// A bibliographic citation for the related artifact. This text SHOULD be formatted
    /// according to an accepted citation format.
    pub citation: super::markdown::Markdown,
    /// Provides additional classifiers of the related artifact.
    pub classifier: Vec<super::codeable_concept::CodeableConcept>,
    /// A brief description of the document or knowledge resource being referenced,
    /// suitable for display to a consumer.
    pub display: super::string::String,
    /// The document being referenced, represented as an attachment. Do not use this
    /// element if using the resource element to provide the canonical to the related
    /// artifact.
    pub document: super::attachment::Attachment,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A short label that can be used to reference the related artifact from elsewhere
    /// in the containing artifact, such as a footnote index.
    pub label: super::string::String,
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
    /// The related artifact, such as a library, value set, profile, or other knowledge
    /// resource.
    pub resource: super::canonical::Canonical,
    /// The related artifact, if the artifact is not a canonical resource, or a resource
    /// reference to a canonical resource.
    pub resource_reference: super::reference::Reference,
    /// The type of relationship to the related artifact.
    pub r#type: super::code::Code,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationStatusDate {
    /// The state or status of the citation record (that will be paired with the
    /// period).
    pub activity: super::codeable_concept::CodeableConcept,
    /// Whether the status date is actual (has occurred) or expected (estimated or
    /// anticipated).
    pub actual: super::boolean::Boolean,
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
    /// When the status started and/or ended.
    pub period: super::period::Period,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationStatusDate1 {
    /// A definition of the status associated with a date or period.
    pub activity: super::codeable_concept::CodeableConcept,
    /// Either occurred or expected.
    pub actual: super::boolean::Boolean,
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
    /// When the status started and/or ended.
    pub period: super::period::Period,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationSummary {
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
    /// Format for display of the citation summary.
    pub style: super::codeable_concept::CodeableConcept,
    /// The human-readable display of the citation summary.
    pub text: super::markdown::Markdown,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationSummary1 {
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
    /// Used to code the producer or rule for creating the display string.
    pub source: super::codeable_concept::CodeableConcept,
    /// The format for the display string, such as author last name with first letter
    /// capitalized followed by forename initials.
    pub style: super::codeable_concept::CodeableConcept,
    /// Used most commonly to express an author list or a contributorship statement.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The display string for the author list, contributor list, or contributorship
    /// statement.
    pub value: super::markdown::Markdown,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationTitle {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Used to express the specific language of the title.
    pub language: super::codeable_concept::CodeableConcept,
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
    /// The title of the article or artifact.
    pub text: super::markdown::Markdown,
    /// Used to express the reason for or classification of the title.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationVersion {
    /// Citation for the main version of the cited artifact.
    pub base_citation: super::reference::Reference,
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
    /// The version number or other version identifier.
    pub value: super::string::String,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes
/// of identification and attribution. The Citation Resource supports existing
/// reference structures and developing publication practices such as versioning,
/// expressing complex contributorship roles, and referencing computable resources.
#[derive(Debug, Clone, PartialEq)]
pub struct CitationWebLocation {
    /// A characterization of the object expected at the web location.
    pub classifier: Vec<super::codeable_concept::CodeableConcept>,
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
    /// The specific URL.
    pub url: super::uri::Uri,
}
