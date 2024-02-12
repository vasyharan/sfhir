/// Specifies clinical/business/etc. metadata that can be used to retrieve, index
/// and/or categorize an artifact. This metadata can either be specific to the
/// applicable population (e.g., age category, DRG) or the specific context of care
/// (e.g., venue, care setting, provider of care).
#[derive(Debug, Clone, PartialEq)]
pub struct UsageContext {
    /// A code that identifies the type of context being specified by this usage
    /// context.
    pub code: super::coding::Coding,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A value that defines the context specified in this context of use. The
    /// interpretation of the value is defined by the code.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// A value that defines the context specified in this context of use. The
    /// interpretation of the value is defined by the code.
    pub value_quantity: super::quantity::Quantity,
    /// A value that defines the context specified in this context of use. The
    /// interpretation of the value is defined by the code.
    pub value_range: super::range::Range,
    /// A value that defines the context specified in this context of use. The
    /// interpretation of the value is defined by the code.
    pub value_reference: super::reference::Reference,
}
