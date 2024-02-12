use super::*;
/// A sample to be used for analysis.
#[derive(Debug,Clone,PartialEq)]
pub struct SpecimenCollection {
/// Anatomical location from which the specimen was collected (if subject is a
/// patient). This is the target site.  This element is not used for environmental
/// specimens.
pub body_site: CodeableReference,
/// Time when specimen was collected from subject - the physiologically relevant
/// time.
pub collected_date_time: String,
/// Time when specimen was collected from subject - the physiologically relevant
/// time.
pub collected_period: Period,
/// Person who collected the specimen.
pub collector: Reference,
/// A coded value specifying the technique that is used to perform the procedure.
pub device: CodeableReference,
/// The span of time over which the collection of a specimen occurred.
pub duration: Duration,
/// Abstinence or reduction from some or all food, drink, or both, for a period of
/// time prior to sample collection.
pub fasting_status_codeable_concept: CodeableConcept,
/// Abstinence or reduction from some or all food, drink, or both, for a period of
/// time prior to sample collection.
pub fasting_status_duration: Duration,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A coded value specifying the technique that is used to perform the procedure.
pub method: CodeableConcept,
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
/// The procedure event during which the specimen was collected (e.g. the surgery
/// leading to the collection of a pathology sample).
pub procedure: Reference,
/// The quantity of specimen collected; for instance the volume of a blood sample,
/// or the physical measurement of an anatomic pathology sample.
pub quantity: Quantity,
}
