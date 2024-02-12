/// A signature along with supporting context. The signature may be a digital
/// signature that is cryptographic in nature, or some other signature acceptable
/// to the domain. This other signature may be as simple as a graphical image
/// representing a hand-written signature, or a signature ceremony Different
/// signature approaches have different utilities.
#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    /// The base64 encoding of the Signature content. When signature is not recorded
    /// electronically this element would be empty.
    pub data: super::base_64_binary::Base64Binary,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A reference to an application-usable description of the identity that is
    /// represented by the signature.
    pub on_behalf_of: super::reference::Reference,
    /// A mime type that indicates the technical format of the signature. Important mime
    /// types are application/signature+xml for X ML DigSig, application/jose for JWS,
    /// and image/* for a graphical image of a signature, etc.
    pub sig_format: super::code::Code,
    /// A mime type that indicates the technical format of the target resources signed
    /// by the signature.
    pub target_format: super::code::Code,
    /// An indication of the reason that the entity signed this document. This may be
    /// explicitly included as part of the signature information and can be used when
    /// determining accountability for various actions concerning the document.
    pub r#type: Vec<super::coding::Coding>,
    /// When the digital signature was signed.
    pub when: super::instant::Instant,
    /// A reference to an application-usable description of the identity that signed
    /// (e.g. the signature used their private key).
    pub who: super::reference::Reference,
}
