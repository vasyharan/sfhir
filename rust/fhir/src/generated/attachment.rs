/// For referring to data content defined in other formats.
#[derive(Debug, Clone, PartialEq)]
pub struct Attachment {
    /// Identifies the type of the data in the attachment and allows a method to be
    /// chosen to interpret or render the data. Includes mime type parameters such as
    /// charset where appropriate.
    pub content_type: super::code::Code,
    /// The date that the attachment was first created.
    pub creation: super::date_time::DateTime,
    /// The actual data of the attachment - a sequence of bytes, base64 encoded.
    pub data: super::base_64_binary::Base64Binary,
    /// The duration of the recording in seconds - for audio and video.
    pub duration: super::decimal::Decimal,
    /// The number of frames in a photo. This is used with a multi-page fax, or an
    /// imaging acquisition context that takes multiple slices in a single image, or
    /// an animated gif. If there is more than one frame, this SHALL have a value in
    /// order to alert interface software that a multi-frame capable rendering widget
    /// is required.
    pub frames: super::positive_int::PositiveInt,
    /// The calculated hash of the data using SHA-1. Represented using base64.
    pub hash: super::base_64_binary::Base64Binary,
    /// Height of the image in pixels (photo/video).
    pub height: super::positive_int::PositiveInt,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The human language of the content. The value can be any valid value according to
    /// BCP 47.
    pub language: super::code::Code,
    /// The number of pages when printed.
    pub pages: super::positive_int::PositiveInt,
    /// The number of bytes of data that make up this attachment (before base64
    /// encoding, if that is done).
    pub size: super::integer_64::Integer64,
    /// A label or set of text to display in place of the data.
    pub title: super::string::String,
    /// A location where the data can be accessed.
    pub url: super::url::Url,
    /// Width of the image in pixels (photo/video).
    pub width: super::positive_int::PositiveInt,
}
