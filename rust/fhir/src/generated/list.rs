/// A List is a curated collection of resources, for things such as problem lists,
/// allergy lists, facility list, organization list, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct List {
    /// This code defines the purpose of the list - why it was created.
    pub code: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Date list was last reviewed/revised and determined to be 'current'.
    pub date: super::date_time::DateTime,
    /// If the list is empty, why the list is empty.
    pub empty_reason: super::codeable_concept::CodeableConcept,
    /// The encounter that is the context in which this list was created.
    pub encounter: super::reference::Reference,
    /// Entries in this list.
    pub entry: Vec<super::list::ListEntry>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifier for the List assigned for business purposes outside the context of
    /// FHIR.
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
    /// How this list was prepared - whether it is a working list that is suitable for
    /// being maintained on an ongoing basis, or if it represents a snapshot of a list
    /// of items from another source, or whether it is a prepared list where items may
    /// be marked as added, modified or deleted.
    pub mode: super::code::Code,
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
    /// Comments that apply to the overall list.
    pub note: Vec<super::annotation::Annotation>,
    /// What order applies to the items in the list.
    pub ordered_by: super::codeable_concept::CodeableConcept,
    /// This is a List resource
    pub resource_type: String,
    /// The entity responsible for deciding what the contents of the list were. Where
    /// the list was created by a human, this is the same as the author of the list.
    pub source: super::reference::Reference,
    /// Indicates the current state of this list.
    pub status: super::code::Code,
    /// The common subject(s) (or patient(s)) of the resources that are in the list if
    /// there is one (or a set of subjects).
    pub subject: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A label for the list assigned by the author.
    pub title: super::string::String,
}

/// A List is a curated collection of resources, for things such as problem lists,
/// allergy lists, facility list, organization list, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct ListEntry {
    /// When this item was added to the list.
    pub date: super::date_time::DateTime,
    /// True if this item is marked as deleted in the list.
    pub deleted: super::boolean::Boolean,
    /// The flag allows the system constructing the list to indicate the role and
    /// significance of the item in the list.
    pub flag: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A reference to the actual resource from which data was derived.
    pub item: super::reference::Reference,
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