/// A series of measurements taken by a device, with upper and lower limits. There
/// may be more than one dimension in the data.
#[derive(Debug, Clone, PartialEq)]
pub struct SampledData {
    /// Reference to ConceptMap that defines the codes used in the data.
    pub code_map: super::canonical::Canonical,
    /// A series of data points which are decimal values or codes separated by a single
    /// space (character u20). The special codes "E" (error), "L" (below detection
    /// limit) and "U" (above detection limit) are also defined for used in place of
    /// decimal values.
    pub data: super::string::String,
    /// The number of sample points at each time point. If this value is greater than
    /// one, then the dimensions will be interlaced - all the sample points for a point
    /// in time will be recorded at once.
    pub dimensions: super::positive_int::PositiveInt,
    /// A correction factor that is applied to the sampled data points before they are
    /// added to the origin.
    pub factor: super::decimal::Decimal,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Amount of intervalUnits between samples, e.g. milliseconds for time-based
    /// sampling.
    pub interval: super::decimal::Decimal,
    /// The measurement unit in which the sample interval is expressed.
    pub interval_unit: super::code::Code,
    /// The lower limit of detection of the measured points. This is needed if any of
    /// the data points have the value "L" (lower than detection limit).
    pub lower_limit: super::decimal::Decimal,
    /// A series of data points which are decimal values separated by a single space
    /// (character u20).  The units in which the offsets are expressed are found in
    /// intervalUnit.  The absolute point at which the measurements begin SHALL be
    /// conveyed outside the scope of this datatype, e.g. Observation.effectiveDateTime
    /// for a timing offset.
    pub offsets: super::string::String,
    /// The base quantity that a measured value of zero represents. In addition, this
    /// provides the units of the entire measurement series.
    pub origin: super::quantity::Quantity,
    /// The upper limit of detection of the measured points. This is needed if any of
    /// the data points have the value "U" (higher than detection limit).
    pub upper_limit: super::decimal::Decimal,
}
