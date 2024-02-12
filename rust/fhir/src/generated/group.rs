/// Represents a defined collection of entities that may be discussed or acted
/// upon collectively but which are not expected to act collectively, and are not
/// formally or legally recognized; i.e. a collection of entities that isn't an
/// Organization.
#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    /// Indicates whether the record for the group is available for use or is merely
    /// being retained for historical purposes.
    pub active: super::boolean::Boolean,
    /// Identifies traits whose presence r absence is shared by members of the group.
    pub characteristic: Vec<super::group::GroupCharacteristic>,
    /// Provides a specific type of resource the group includes; e.g. "cow", "syringe",
    /// etc.
    pub code: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Explanation of what the group represents and how it is intended to be used.
    pub description: super::markdown::Markdown,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this participant by one of the applications
    /// involved.  These identifiers remain constant as the resource is updated and
    /// propagates from server to server.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Entity responsible for defining and maintaining Group characteristics and/or
    /// registered members.
    pub managing_entity: super::reference::Reference,
    /// Identifies the resource instances that are members of the group.
    pub member: Vec<super::group::GroupMember>,
    /// Basis for membership in the Group:
    ///
    /// * 'definitional': The Group.characteristics specified are both necessary and
    /// sufficient to determine membership. All entities that meet the criteria are
    /// considered to be members of the group, whether referenced by the group or not.
    /// If members are present, they are individuals that happen to be known as meeting
    /// the Group.characteristics. The list cannot be presumed to be complete.
    /// * 'enumerated': The Group.characteristics are necessary but not sufficient to
    /// determine membership. Membership is determined by being listed as one of the
    /// Group.member.
    pub membership: super::code::Code,
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
    /// A label assigned to the group for human identification and communication.
    pub name: super::string::String,
    /// A count of the number of resource instances that are part of the group.
    pub quantity: super::unsigned_int::UnsignedInt,
    /// This is a Group resource
    pub resource_type: String,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Identifies the broad classification of the kind of resources the group includes.
    pub r#type: super::code::Code,
}

/// Represents a defined collection of entities that may be discussed or acted
/// upon collectively but which are not expected to act collectively, and are not
/// formally or legally recognized; i.e. a collection of entities that isn't an
/// Organization.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupCharacteristic {
    /// A code that identifies the kind of trait being asserted.
    pub code: super::codeable_concept::CodeableConcept,
    /// If true, indicates the characteristic is one that is NOT held by members of
    /// the group.
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
    /// The period over which the characteristic is tested; e.g. the patient had an
    /// operation during the month of June.
    pub period: super::period::Period,
    /// The value of the trait that holds (or does not hold - see 'exclude') for members
    /// of the group.
    pub value_boolean: bool,
    /// The value of the trait that holds (or does not hold - see 'exclude') for members
    /// of the group.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The value of the trait that holds (or does not hold - see 'exclude') for members
    /// of the group.
    pub value_quantity: super::quantity::Quantity,
    /// The value of the trait that holds (or does not hold - see 'exclude') for members
    /// of the group.
    pub value_range: super::range::Range,
    /// The value of the trait that holds (or does not hold - see 'exclude') for members
    /// of the group.
    pub value_reference: super::reference::Reference,
}

/// Represents a defined collection of entities that may be discussed or acted
/// upon collectively but which are not expected to act collectively, and are not
/// formally or legally recognized; i.e. a collection of entities that isn't an
/// Organization.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupMember {
    /// A reference to the entity that is a member of the group. Must be consistent with
    /// Group.type. If the entity is another group, then the type must be the same.
    pub entity: super::reference::Reference,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A flag to indicate that the member is no longer in the group, but previously may
    /// have been a member.
    pub inactive: super::boolean::Boolean,
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
    /// The period that the member was in the group, if known.
    pub period: super::period::Period,
}
