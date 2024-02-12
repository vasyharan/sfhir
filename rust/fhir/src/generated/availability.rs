/// Availability data for an {item}.
#[derive(Debug, Clone, PartialEq)]
pub struct Availability {
    /// Times the {item} is available.
    pub available_time: Vec<super::availability::AvailabilityAvailableTime>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Not available during this time due to provided reason.
    pub not_available_time: Vec<super::availability::AvailabilityNotAvailableTime>,
}

/// Availability data for an {item}.
#[derive(Debug, Clone, PartialEq)]
pub struct AvailabilityAvailableTime {
    /// Always available? i.e. 24 hour service.
    pub all_day: super::boolean::Boolean,
    /// Closing time of day (ignored if allDay = true).
    pub available_end_time: super::time::Time,
    /// Opening time of day (ignored if allDay = true).
    pub available_start_time: super::time::Time,
    /// mon | tue | wed | thu | fri | sat | sun.
    pub days_of_week: Vec<super::code::Code>,
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

/// Availability data for an {item}.
#[derive(Debug, Clone, PartialEq)]
pub struct AvailabilityNotAvailableTime {
    /// Reason presented to the user explaining why time not available.
    pub description: super::string::String,
    /// Service not available during this period.
    pub during: super::period::Period,
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
