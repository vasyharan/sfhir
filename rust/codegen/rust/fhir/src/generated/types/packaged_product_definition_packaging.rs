use super::*;
/// A medically related item or items, in a container or package.
#[derive(Debug,Clone,PartialEq)]
pub struct PackagedProductDefinitionPackaging {
/// A possible alternate material for this part of the packaging, that is allowed
/// to be used instead of the usual material (e.g. different types of plastic for a
/// blister sleeve).
pub alternate_material: Vec<CodeableConcept>,
/// Is this a part of the packaging (e.g. a cap or bottle stopper), rather than
/// the packaging itself (e.g. a bottle or vial). The latter type are designed be a
/// container, but the former are not.
pub component_part: Boolean,
/// The item(s) within the packaging.
pub contained_item: Vec<PackagedProductDefinitionContainedItem>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A business identifier that is specific to this particular part of the packaging,
/// often assigned by the manufacturer. Including possibly Data Carrier Identifier
/// (a GS1 barcode).
pub identifier: Vec<Identifier>,
/// Manufacturer of this packaging item. When there are multiple values each one is
/// a potential manufacturer of this packaging item.
pub manufacturer: Vec<Reference>,
/// Material type of the package item.
pub material: Vec<CodeableConcept>,
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
pub modifier_extension: Vec<Extension>,
/// Allows containers (and parts of containers) within containers,
/// still as a part of a single packaged product. See also
/// PackagedProductDefinition.packaging.containedItem.item(PackagedProductDefinition
/// ).
pub packaging: Vec<PackagedProductDefinitionPackaging>,
/// General characteristics of this item.
pub property: Vec<PackagedProductDefinitionProperty>,
/// The quantity of packaging items contained at this layer of the package. This
/// does not relate to the number of contained items but relates solely to the
/// number of packaging items. When looking at the outermost layer it is always 1.
/// If there are two boxes within, at the next layer it would be 2.
pub quantity: Integer,
/// Shelf Life and storage information.
pub shelf_life_storage: Vec<ProductShelfLife>,
/// The physical type of the container of the items.
pub r#type: CodeableConcept,
}
