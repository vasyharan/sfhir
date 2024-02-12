/// A medicinal product in the final form which is suitable for administering to
/// a patient (after any mixing of multiple components, dissolution etc. has been
/// performed).
#[derive(Debug, Clone, PartialEq)]
pub struct AdministrableProductDefinition {
/// The dose form of the final product after necessary reconstitution or processing.
/// Contrasts to the manufactured dose form (see ManufacturedItemDefinition). If
/// the manufactured form was 'powder for solution for injection', the administrable
/// dose form could be 'solution for injection' (once mixed with another item having
/// manufactured form 'solvent for solution for injection').
pub administrable_dose_form: super::codeable_concept::CodeableConcept,
/// These resources do not have an independent existence apart from the resource
/// that contains them - they cannot be identified independently, nor can they have
/// their own independent transaction scope. This is allowed to be a Parameters
/// resource if and only if it is referenced by a resource that provides context/
/// meaning.
pub contained: Vec<super::resource_list::ResourceList>,
/// A general description of the product, when in its final form, suitable for
/// administration e.g. effervescent blue liquid, to be swallowed. Intended to be
/// used when the other structured properties of this resource are insufficient or
/// cannot be supported. It is not intended to duplicate information already carried
/// elswehere.
pub description: super::markdown::Markdown,
/// A device that is integral to the medicinal product, in effect being considered
/// as an "ingredient" of the medicinal product. This is not intended for devices
/// that are just co-packaged.
pub device: super::reference::Reference,
/// References a product from which one or more of the constituent parts of that
/// product can be prepared and used as described by this administrable product.  If
/// this administrable product describes the administration of a crushed tablet, the
/// 'formOf' would be the product representing a distribution containing tablets and
/// possibly also a cream.  This is distinct from the 'producedFrom' which refers to
/// the specific components of the product that are used in this preparation, rather
/// than the product as a whole.
pub form_of: Vec<super::reference::Reference>,
/// The logical id of the resource, as used in the URL for the resource. Once
/// assigned, this value never changes.
pub id: super::id::Id,
/// An identifier for the administrable product.
pub identifier: Vec<super::identifier::Identifier>,
/// A reference to a set of rules that were followed when the resource was
/// constructed, and which must be understood when processing the content. Often,
/// this is a reference to an implementation guide that defines the special rules
/// along with other profiles etc.
pub implicit_rules: super::uri::Uri,
/// The ingredients of this administrable medicinal product. This is only needed if
/// the ingredients are not specified either using ManufacturedItemDefiniton (via
/// AdministrableProductDefinition.producedFrom) to state which component items are
/// used to make this, or using by incoming references from the Ingredient resource,
/// to state in detail which substances exist within this. This element allows a
/// basic coded ingredient to be used.
pub ingredient: Vec<super::codeable_concept::CodeableConcept>,
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
/// Indicates the specific manufactured items that are part of the 'formOf' product
/// that are used in the preparation of this specific administrable form.  In some
/// cases, an administrable form might use all of the items from the overall product
/// (or there might only be one item), while in other cases, an administrable form
/// might use only a subset of the items available in the overall product.  For
/// example, an administrable form might involve combining a liquid and a powder
/// available as part of an overall product, but not involve applying the also
/// supplied cream.
pub produced_from: Vec<super::reference::Reference>,
/// Characteristics e.g. a product's onset of action.
pub property: Vec<super::administrable_product_definition::AdministrableProductDefinitionProperty>,
/// This is a AdministrableProductDefinition resource
pub resource_type: String,
/// The path by which the product is taken into or makes contact with the body.
/// In some regions this is referred to as the licenced or approved route.
/// RouteOfAdministration cannot be used when the 'formOf' product already uses
/// MedicinalProductDefinition.route (and vice versa).
pub route_of_administration: Vec<super::administrable_product_definition::AdministrableProductDefinitionRouteOfAdministration>,
/// The status of this administrable product. Enables tracking the life-cycle of
/// the content.
pub status: super::code::Code,
/// A human-readable narrative that contains a summary of the resource and can be
/// used to represent the content of the resource to a human. The narrative need
/// not encode all the structured data, but is required to contain sufficient detail
/// to make it "clinically safe" for a human to just read the narrative. Resource
/// definitions may define what content should be represented in the narrative to
/// ensure clinical safety.
pub text: super::narrative::Narrative,
/// The presentation type in which this item is given to a patient. e.g. for a spray
/// - 'puff' (as in 'contains 100 mcg per puff'), or for a liquid - 'vial' (as in
/// 'contains 5 ml per vial').
pub unit_of_presentation: super::codeable_concept::CodeableConcept,
}

/// A medicinal product in the final form which is suitable for administering to
/// a patient (after any mixing of multiple components, dissolution etc. has been
/// performed).
#[derive(Debug, Clone, PartialEq)]
pub struct AdministrableProductDefinitionProperty {
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
    /// The status of characteristic e.g. assigned or pending.
    pub status: super::codeable_concept::CodeableConcept,
    /// A code expressing the type of characteristic.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// A value for the characteristic.
    pub value_attachment: super::attachment::Attachment,
    /// A value for the characteristic.
    pub value_boolean: bool,
    /// A value for the characteristic.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// A value for the characteristic.
    pub value_date: String,
    /// A value for the characteristic.
    pub value_markdown: String,
    /// A value for the characteristic.
    pub value_quantity: super::quantity::Quantity,
    /// A value for the characteristic.
    pub value_reference: super::reference::Reference,
}

/// A medicinal product in the final form which is suitable for administering to
/// a patient (after any mixing of multiple components, dissolution etc. has been
/// performed).
#[derive(Debug, Clone, PartialEq)]
pub struct AdministrableProductDefinitionRouteOfAdministration {
    /// Coded expression for the route.
    pub code: super::codeable_concept::CodeableConcept,
    /// The first dose (dose quantity) administered can be specified for the product,
    /// using a numerical value and its unit of measurement.
    pub first_dose: super::quantity::Quantity,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The maximum dose per day (maximum dose quantity to be administered in any one
    /// 24-h period) that can be administered.
    pub max_dose_per_day: super::quantity::Quantity,
    /// The maximum dose per treatment period that can be administered.
    pub max_dose_per_treatment_period: super::ratio::Ratio,
    /// The maximum single dose that can be administered, specified using a numerical
    /// value and its unit of measurement.
    pub max_single_dose: super::quantity::Quantity,
    /// The maximum treatment period during which the product can be administered.
    pub max_treatment_period: super::duration::Duration,
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
    /// A species for which this route applies.
    pub target_species:
        Vec<super::administrable_product_definition::AdministrableProductDefinitionTargetSpecies>,
}

/// A medicinal product in the final form which is suitable for administering to
/// a patient (after any mixing of multiple components, dissolution etc. has been
/// performed).
#[derive(Debug, Clone, PartialEq)]
pub struct AdministrableProductDefinitionTargetSpecies {
    /// Coded expression for the species.
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
    /// A species specific time during which consumption of animal product is not
    /// appropriate.
    pub withdrawal_period: Vec<
        super::administrable_product_definition::AdministrableProductDefinitionWithdrawalPeriod,
    >,
}

/// A medicinal product in the final form which is suitable for administering to
/// a patient (after any mixing of multiple components, dissolution etc. has been
/// performed).
#[derive(Debug, Clone, PartialEq)]
pub struct AdministrableProductDefinitionWithdrawalPeriod {
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
    /// Extra information about the withdrawal period.
    pub supporting_information: super::string::String,
    /// Coded expression for the type of tissue for which the withdrawal period applies,
    /// e.g. meat, milk.
    pub tissue: super::codeable_concept::CodeableConcept,
    /// A value for the time.
    pub value: super::quantity::Quantity,
}
