/// Source material shall capture information on the taxonomic and anatomical
/// origins as well as the fraction of a material that can result in or can
/// be modified to form a substance. This set of data elements shall be used
/// to define polymer substances isolated from biological matrices. Taxonomic
/// and anatomical origins shall be described using a controlled vocabulary as
/// required. This information is captured for naturally derived polymers ( .
/// starch) and structurally diverse substances. For Organisms belonging to the
/// Kingdom Plantae the Substance level defines the fresh material of a single
/// species or infraspecies, the Herbal Drug and the Herbal preparation. For
/// Herbal preparations, the fraction information will be captured at the Substance
/// information level and additional information for herbal extracts will be
/// captured at the Specified Substance Group 1 information level. See for further
/// explanation the Substance Class: Structurally Diverse and the herbal annex.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSourceMaterial {
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The country where the plant material is harvested or the countries where
    /// the plasma is sourced from as laid down in accordance with the Plasma Master
    /// File. For “Plasma-derived substances” the attribute country of origin provides
    /// information about the countries used for the manufacturing of the Cryopoor plama
    /// or Crioprecipitate.
    pub country_of_origin: Vec<super::codeable_concept::CodeableConcept>,
    /// Stage of life for animals, plants, insects and microorganisms. This information
    /// shall be provided only when the substance is significantly different in these
    /// stages (e.g. foetal bovine serum).
    pub development_stage: super::codeable_concept::CodeableConcept,
    /// Many complex materials are fractions of parts of plants, animals, or minerals.
    /// Fraction elements are often necessary to define both Substances and Specified
    /// Group 1 Substances. For substances derived from Plants, fraction information
    /// will be captured at the Substance information level ( . Oils, Juices and
    /// Exudates). Additional information for Extracts, such as extraction solvent
    /// composition, will be captured at the Specified Substance Group 1 information
    /// level. For plasma-derived products fraction information will be captured at the
    /// Substance and the Specified Substance Group 1 levels.
    pub fraction_description:
        Vec<super::substance_source_material::SubstanceSourceMaterialFractionDescription>,
    /// The place/region where the plant is harvested or the places/regions where the
    /// animal source material has its habitat.
    pub geographical_location: Vec<super::string::String>,
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
    /// This subclause describes the organism which the substance is derived from.
    /// For vaccines, the parent organism shall be specified based on these subclause
    /// elements. As an example, full taxonomy will be described for the Substance
    /// Name: ., Leaf.
    pub organism: super::substance_source_material::SubstanceSourceMaterialOrganism,
    /// The unique identifier associated with the source material parent organism shall
    /// be specified.
    pub organism_id: super::identifier::Identifier,
    /// The organism accepted Scientific name shall be provided based on the organism
    /// taxonomy.
    pub organism_name: super::string::String,
    /// The parent of the herbal drug Ginkgo biloba, Leaf is the substance ID of the
    /// substance (fresh) of Ginkgo biloba L. or Ginkgo biloba L. (Whole plant).
    pub parent_substance_id: Vec<super::identifier::Identifier>,
    /// The parent substance of the Herbal Drug, or Herbal preparation.
    pub parent_substance_name: Vec<super::string::String>,
    /// To do.
    pub part_description:
        Vec<super::substance_source_material::SubstanceSourceMaterialPartDescription>,
    /// This is a SubstanceSourceMaterial resource
    pub resource_type: String,
    /// General high level classification of the source material specific to the origin
    /// of the material.
    pub source_material_class: super::codeable_concept::CodeableConcept,
    /// The state of the source material when extracted.
    pub source_material_state: super::codeable_concept::CodeableConcept,
    /// The type of the source material shall be specified based on a controlled
    /// vocabulary. For vaccines, this subclause refers to the class of infectious
    /// agent.
    pub source_material_type: super::codeable_concept::CodeableConcept,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// Source material shall capture information on the taxonomic and anatomical
/// origins as well as the fraction of a material that can result in or can
/// be modified to form a substance. This set of data elements shall be used
/// to define polymer substances isolated from biological matrices. Taxonomic
/// and anatomical origins shall be described using a controlled vocabulary as
/// required. This information is captured for naturally derived polymers ( .
/// starch) and structurally diverse substances. For Organisms belonging to the
/// Kingdom Plantae the Substance level defines the fresh material of a single
/// species or infraspecies, the Herbal Drug and the Herbal preparation. For
/// Herbal preparations, the fraction information will be captured at the Substance
/// information level and additional information for herbal extracts will be
/// captured at the Specified Substance Group 1 information level. See for further
/// explanation the Substance Class: Structurally Diverse and the herbal annex.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSourceMaterialAuthor {
    /// The author of an organism species shall be specified. The author year of an
    /// organism shall also be specified when applicable; refers to the year in which
    /// the first author(s) published the infraspecific plant/animal name (of any rank).
    pub author_description: super::string::String,
    /// The type of author of an organism species shall be specified. The parenthetical
    /// author of an organism species refers to the first author who published the
    /// plant/animal name (of any rank). The primary author of an organism species
    /// refers to the first author(s), who validly published the plant/animal name.
    pub author_type: super::codeable_concept::CodeableConcept,
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
}

/// Source material shall capture information on the taxonomic and anatomical
/// origins as well as the fraction of a material that can result in or can
/// be modified to form a substance. This set of data elements shall be used
/// to define polymer substances isolated from biological matrices. Taxonomic
/// and anatomical origins shall be described using a controlled vocabulary as
/// required. This information is captured for naturally derived polymers ( .
/// starch) and structurally diverse substances. For Organisms belonging to the
/// Kingdom Plantae the Substance level defines the fresh material of a single
/// species or infraspecies, the Herbal Drug and the Herbal preparation. For
/// Herbal preparations, the fraction information will be captured at the Substance
/// information level and additional information for herbal extracts will be
/// captured at the Specified Substance Group 1 information level. See for further
/// explanation the Substance Class: Structurally Diverse and the herbal annex.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSourceMaterialFractionDescription {
    /// This element is capturing information about the fraction of a plant part, or
    /// human plasma for fractionation.
    pub fraction: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The specific type of the material constituting the component. For Herbal
    /// preparations the particulars of the extracts (liquid/dry) is described in
    /// Specified Substance Group 1.
    pub material_type: super::codeable_concept::CodeableConcept,
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

/// Source material shall capture information on the taxonomic and anatomical
/// origins as well as the fraction of a material that can result in or can
/// be modified to form a substance. This set of data elements shall be used
/// to define polymer substances isolated from biological matrices. Taxonomic
/// and anatomical origins shall be described using a controlled vocabulary as
/// required. This information is captured for naturally derived polymers ( .
/// starch) and structurally diverse substances. For Organisms belonging to the
/// Kingdom Plantae the Substance level defines the fresh material of a single
/// species or infraspecies, the Herbal Drug and the Herbal preparation. For
/// Herbal preparations, the fraction information will be captured at the Substance
/// information level and additional information for herbal extracts will be
/// captured at the Specified Substance Group 1 information level. See for further
/// explanation the Substance Class: Structurally Diverse and the herbal annex.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSourceMaterialHybrid {
    /// The hybrid type of an organism shall be specified.
    pub hybrid_type: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The identifier of the maternal species constituting the hybrid organism shall
    /// be specified based on a controlled vocabulary. For plants, the parents aren’t
    /// always known, and it is unlikely that it will be known which is maternal and
    /// which is paternal.
    pub maternal_organism_id: super::string::String,
    /// The name of the maternal species constituting the hybrid organism shall be
    /// specified. For plants, the parents aren’t always known, and it is unlikely that
    /// it will be known which is maternal and which is paternal.
    pub maternal_organism_name: super::string::String,
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
    /// The identifier of the paternal species constituting the hybrid organism shall be
    /// specified based on a controlled vocabulary.
    pub paternal_organism_id: super::string::String,
    /// The name of the paternal species constituting the hybrid organism shall be
    /// specified.
    pub paternal_organism_name: super::string::String,
}

/// Source material shall capture information on the taxonomic and anatomical
/// origins as well as the fraction of a material that can result in or can
/// be modified to form a substance. This set of data elements shall be used
/// to define polymer substances isolated from biological matrices. Taxonomic
/// and anatomical origins shall be described using a controlled vocabulary as
/// required. This information is captured for naturally derived polymers ( .
/// starch) and structurally diverse substances. For Organisms belonging to the
/// Kingdom Plantae the Substance level defines the fresh material of a single
/// species or infraspecies, the Herbal Drug and the Herbal preparation. For
/// Herbal preparations, the fraction information will be captured at the Substance
/// information level and additional information for herbal extracts will be
/// captured at the Specified Substance Group 1 information level. See for further
/// explanation the Substance Class: Structurally Diverse and the herbal annex.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSourceMaterialOrganism {
    /// 4.9.13.6.1 Author type (Conditional).
    pub author: Vec<super::substance_source_material::SubstanceSourceMaterialAuthor>,
    /// The family of an organism shall be specified.
    pub family: super::codeable_concept::CodeableConcept,
    /// The genus of an organism shall be specified; refers to the Latin epithet of the
    /// genus element of the plant/animal scientific name; it is present in names for
    /// genera, species and infraspecies.
    pub genus: super::codeable_concept::CodeableConcept,
    /// 4.9.13.8.1 Hybrid species maternal organism ID (Optional).
    pub hybrid: super::substance_source_material::SubstanceSourceMaterialHybrid,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The intraspecific description of an organism shall be specified based on a
    /// controlled vocabulary. For Influenza Vaccine, the intraspecific description
    /// shall contain the syntax of the antigen in line with the WHO convention.
    pub intraspecific_description: super::string::String,
    /// The Intraspecific type of an organism shall be specified.
    pub intraspecific_type: super::codeable_concept::CodeableConcept,
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
    /// 4.9.13.7.1 Kingdom (Conditional).
    pub organism_general: super::substance_source_material::SubstanceSourceMaterialOrganismGeneral,
    /// The species of an organism shall be specified; refers to the Latin epithet
    /// of the species of the plant/animal; it is present in names for species and
    /// infraspecies.
    pub species: super::codeable_concept::CodeableConcept,
}

/// Source material shall capture information on the taxonomic and anatomical
/// origins as well as the fraction of a material that can result in or can
/// be modified to form a substance. This set of data elements shall be used
/// to define polymer substances isolated from biological matrices. Taxonomic
/// and anatomical origins shall be described using a controlled vocabulary as
/// required. This information is captured for naturally derived polymers ( .
/// starch) and structurally diverse substances. For Organisms belonging to the
/// Kingdom Plantae the Substance level defines the fresh material of a single
/// species or infraspecies, the Herbal Drug and the Herbal preparation. For
/// Herbal preparations, the fraction information will be captured at the Substance
/// information level and additional information for herbal extracts will be
/// captured at the Specified Substance Group 1 information level. See for further
/// explanation the Substance Class: Structurally Diverse and the herbal annex.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSourceMaterialOrganismGeneral {
    /// The class of an organism shall be specified.
    pub class: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The kingdom of an organism shall be specified.
    pub kingdom: super::codeable_concept::CodeableConcept,
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
    /// The order of an organism shall be specified,.
    pub order: super::codeable_concept::CodeableConcept,
    /// The phylum of an organism shall be specified.
    pub phylum: super::codeable_concept::CodeableConcept,
}

/// Source material shall capture information on the taxonomic and anatomical
/// origins as well as the fraction of a material that can result in or can
/// be modified to form a substance. This set of data elements shall be used
/// to define polymer substances isolated from biological matrices. Taxonomic
/// and anatomical origins shall be described using a controlled vocabulary as
/// required. This information is captured for naturally derived polymers ( .
/// starch) and structurally diverse substances. For Organisms belonging to the
/// Kingdom Plantae the Substance level defines the fresh material of a single
/// species or infraspecies, the Herbal Drug and the Herbal preparation. For
/// Herbal preparations, the fraction information will be captured at the Substance
/// information level and additional information for herbal extracts will be
/// captured at the Specified Substance Group 1 information level. See for further
/// explanation the Substance Class: Structurally Diverse and the herbal annex.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSourceMaterialPartDescription {
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
    /// Entity of anatomical origin of source material within an organism.
    pub part: super::codeable_concept::CodeableConcept,
    /// The detailed anatomic location when the part can be extracted from different
    /// anatomical locations of the organism. Multiple alternative locations may apply.
    pub part_location: super::codeable_concept::CodeableConcept,
}
