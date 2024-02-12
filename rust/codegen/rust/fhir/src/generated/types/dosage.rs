use super::*;
/// Indicates how the medication is/was taken or should be taken by the patient.
#[derive(Debug,Clone,PartialEq)]
pub struct Dosage {
/// Supplemental instructions to the patient on how to take the medication  (e.g.
/// "with meals" or"take half to one hour before food") or warnings for the patient
/// about the medication (e.g. "may cause drowsiness" or "avoid exposure of skin to
/// direct sunlight or sunlamps").
pub additional_instruction: Vec<CodeableConcept>,
/// Indicates whether the Medication is only taken when needed within a specific
/// dosing schedule (Boolean option).
pub as_needed: Boolean,
/// Indicates whether the Medication is only taken based on a precondition for
/// taking the Medication (CodeableConcept).
pub as_needed_for: Vec<CodeableConcept>,
/// Depending on the resource,this is the amount of medication administered, to  be
/// administered or typical amount to be administered.
pub dose_and_rate: Vec<DosageDoseAndRate>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Upper limit on medication per administration.
pub max_dose_per_administration: Quantity,
/// Upper limit on medication per lifetime of the patient.
pub max_dose_per_lifetime: Quantity,
/// Upper limit on medication per unit of time.
pub max_dose_per_period: Vec<Ratio>,
/// Technique for administering medication.
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
/// Instructions in terms that are understood by the patient or consumer.
pub patient_instruction: String,
/// How drug should enter body.
pub route: CodeableConcept,
/// Indicates the order in which the dosage instructions should be applied or
/// interpreted.
pub sequence: Integer,
/// Body site to administer to.
pub site: CodeableConcept,
/// Free text dosage instructions e.g. SIG.
pub text: String,
/// When medication should be administered.
pub timing: Timing,
}
