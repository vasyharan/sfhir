/// A reference to a code defined by a terminology system.
#[derive(Debug, Clone, PartialEq)]
pub struct Coding {
    /// A symbol in syntax defined by the system. The symbol may be a predefined code or
    /// an expression in a syntax defined by the coding system (e.g. post-coordination).
    pub code: super::code::Code,
    /// A representation of the meaning of the code in the system, following the rules
    /// of the system.
    pub display: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The identification of the code system that defines the meaning of the symbol in
    /// the code.
    pub system: super::uri::Uri,
    /// Indicates that this coding was chosen by a user directly - e.g. off a pick list
    /// of available items (codes or displays).
    pub user_selected: super::boolean::Boolean,
    /// The version of the code system which was used when choosing this code. Note that
    /// a well-maintained code system does not need the version reported, because the
    /// meaning of codes is consistent across versions. However this cannot consistently
    /// be assured, and when the meaning is not guaranteed to be consistent, the version
    /// SHOULD be exchanged.
    pub version: super::string::String,
}
