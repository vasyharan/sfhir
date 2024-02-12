/// This resource is primarily used for the identification and definition of a
/// medication, including ingredients, for the purposes of prescribing, dispensing,
/// and administering a medication as well as for making statements about medication
/// use.
#[derive(Debug, Clone, PartialEq)]
pub struct Medication {
    /// Information that only applies to packages (not products).
    pub batch: super::medication::MedicationBatch,
    /// A code (or set of codes) that specify this medication, or a textual description
    /// if no code is available. Usage note: This could be a standard medication code
    /// such as a code from RxNorm, SNOMED CT, IDMP etc. It could also be a national or
    /// local formulary code, optionally with translations to other code systems.
    pub code: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A reference to a knowledge resource that provides more information about this
    /// medication.
    pub definition: super::reference::Reference,
    /// Describes the form of the item.  Powder; tablets; capsule.
    pub dose_form: super::codeable_concept::CodeableConcept,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifier for this medication.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Identifies a particular constituent of interest in the product.
    pub ingredient: Vec<super::medication::MedicationIngredient>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The company or other legal entity that has authorization, from the appropriate
    /// drug regulatory authority,  to market a medicine in one or more jurisdictions.
    /// Typically abbreviated MAH.Note:  The MAH may manufacture the product and
    /// may also contract the manufacturing of the product to one or more companies
    /// (organizations).
    pub marketing_authorization_holder: super::reference::Reference,
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
    /// This is a Medication resource
    pub resource_type: String,
    /// A code to indicate if the medication is in active use.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// When the specified product code does not infer a package size, this is the
    /// specific amount of drug in the product.  For example, when specifying a product
    /// that has the same strength (For example, Insulin glargine 100 unit per mL
    /// solution for injection), this attribute provides additional clarification of the
    /// package amount (For example, 3 mL, 10mL, etc.).
    pub total_volume: super::quantity::Quantity,
}

/// This resource is primarily used for the identification and definition of a
/// medication, including ingredients, for the purposes of prescribing, dispensing,
/// and administering a medication as well as for making statements about medication
/// use.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationBatch {
    /// When this specific batch of product will expire.
    pub expiration_date: super::date_time::DateTime,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The assigned lot number of a batch of the specified product.
    pub lot_number: super::string::String,
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

/// This resource is primarily used for the identification and definition of a
/// medication, including ingredients, for the purposes of prescribing, dispensing,
/// and administering a medication as well as for making statements about medication
/// use.
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationIngredient {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Indication of whether this ingredient affects the therapeutic action of the
    /// drug.
    pub is_active: super::boolean::Boolean,
    /// The ingredient (substance or medication) that the ingredient.strength relates
    /// to.  This is represented as a concept from a code system or described in another
    /// resource (Substance or Medication).
    pub item: super::codeable_reference::CodeableReference,
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
    /// Specifies how many (or how much) of the items there are in this Medication.  For
    /// example, 250 mg per tablet.  This is expressed as a ratio where the numerator is
    /// 250mg and the denominator is 1 tablet but can also be expressed a quantity when
    /// the denominator is assumed to be 1 tablet.
    pub strength_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Specifies how many (or how much) of the items there are in this Medication.  For
    /// example, 250 mg per tablet.  This is expressed as a ratio where the numerator is
    /// 250mg and the denominator is 1 tablet but can also be expressed a quantity when
    /// the denominator is assumed to be 1 tablet.
    pub strength_quantity: super::quantity::Quantity,
    /// Specifies how many (or how much) of the items there are in this Medication.  For
    /// example, 250 mg per tablet.  This is expressed as a ratio where the numerator is
    /// 250mg and the denominator is 1 tablet but can also be expressed a quantity when
    /// the denominator is assumed to be 1 tablet.
    pub strength_ratio: super::ratio::Ratio,
}
