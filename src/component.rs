use std::str::FromStr;

/// Tells you that the given string is not a valid component
#[derive(Debug, PartialEq)]
pub struct InvalidComponentError(String);

/// Tells you that the given string is not a valid property
#[derive(Debug, PartialEq)]
pub struct InvalidPropertyError(String);

/// Tells you that the given string is not a valid parameter
#[derive(Debug, PartialEq)]
pub struct InvalidParameterError(String);

/// Taken from the RFC5545
#[allow(missing_docs, non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ComponentType {
    VCALENDAR,
    VEVENT,
    VTODO,
    VJOURNAL,
    VFREEBUSY,
    VTIMEZONE,
    VALARM,
    STANDARD,
    DAYLIGHT,
}

impl FromStr for ComponentType {
    type Err = InvalidComponentError;

    fn from_str(s: &str) -> Result<ComponentType, Self::Err> {
        match &s.to_lowercase()[..] {
            "vcalendar" => {
                Ok(ComponentType::VCALENDAR)
            },
            "vevent" => {
                Ok(ComponentType::VEVENT)
            },
            "vtodo" => {
                Ok(ComponentType::VTODO)
            },
            "vjournal" => {
                Ok(ComponentType::VJOURNAL)
            },
            "vfreebusy" => {
                Ok(ComponentType::VFREEBUSY)
            },
            "vtimezone" => {
                Ok(ComponentType::VTIMEZONE)
            },
            "valarm" => {
                Ok(ComponentType::VALARM)
            },
            "standard" => {
                Ok(ComponentType::STANDARD)
            },
            "daylight" => {
                Ok(ComponentType::DAYLIGHT)
            },
            unknown => {
                Err(InvalidComponentError(unknown.to_owned()))
            }
        }
    }
}

#[test]
fn create_component_type() {
    let results_expected = vec![
        ("VCALENDAR",   Ok(ComponentType::VCALENDAR)),
        ("VEVENT",      Ok(ComponentType::VEVENT)),
        ("VTODO",       Ok(ComponentType::VTODO)),
        ("VJOURNAL",    Ok(ComponentType::VJOURNAL)),
        ("VFREEBUSY",   Ok(ComponentType::VFREEBUSY)),
        ("VTIMEZONE",   Ok(ComponentType::VTIMEZONE)),
        ("VALARM",      Ok(ComponentType::VALARM)),
        ("STANDARD",    Ok(ComponentType::STANDARD)),
        ("DAYLIGHT",    Ok(ComponentType::DAYLIGHT)),
        ("dasts",       Err(InvalidComponentError("dasts".into()))),
    ];

    for (q, r) in results_expected {
        assert_eq!(ComponentType::from_str(q), r);
    }
}

/// Taken from the RFC 5545
#[allow(missing_docs, non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PropertyType {
    CALSCALE,
    METHOD,
    PRODID,
    VERSION,
    ATTACH,
    CATEGORIES,
    CLASS,
    COMMENT,
    DESCRIPTION,
    GEO,
    LOCATION,
    PERCENT_COMPLETE,
    PRIORITY,
    RESOURCES,
    STATUS,
    SUMMARY,
    COMPLETED,
    DTEND,
    DUE,
    DTSTART,
    DURATION,
    FREEBUSY,
    TRANSP,
    TZID,
    TZNAME,
    TZOFFSETFROM,
    TZOFFSETTO,
    TZURL,
    ATTENDEE,
    CONTACT,
    ORGANIZER,
    RECURRENCE_ID,
    RELATED_TO,
    URL,
    UID,
    EXDATE,
    EXRULE,
    RDATE,
    RRULE,
    ACTION,
    REPEAT,
    TRIGGER,
    CREATED,
    DTSTAMP,
    LAST_MODIFIED,
    SEQUENCE,
    REQUEST_STATUS,
}

impl FromStr for PropertyType {
    type Err = InvalidPropertyError;

    fn from_str(s: &str) -> Result<PropertyType, Self::Err> {
        match &s.to_lowercase()[..] {
            "calscale" => {
                Ok(PropertyType::CALSCALE)
            },
            "method" => {
                Ok(PropertyType::METHOD)
            },
            "prodid" => {
                Ok(PropertyType::PRODID)
            },
            "version" => {
                Ok(PropertyType::VERSION)
            },
            "attach" => {
                Ok(PropertyType::ATTACH)
            },
            "categories" => {
                Ok(PropertyType::CATEGORIES)
            },
            "class" => {
                Ok(PropertyType::CLASS)
            },
            "comment" => {
                Ok(PropertyType::COMMENT)
            },
            "description" => {
                Ok(PropertyType::DESCRIPTION)
            },
            "geo" => {
                Ok(PropertyType::GEO)
            },
            "location" => {
                Ok(PropertyType::LOCATION)
            },
            "percent_complete" => {
                Ok(PropertyType::PERCENT_COMPLETE)
            },
            "priority" => {
                Ok(PropertyType::PRIORITY)
            },
            "resources" => {
                Ok(PropertyType::RESOURCES)
            },
            "status" => {
                Ok(PropertyType::STATUS)
            },
            "summary" => {
                Ok(PropertyType::SUMMARY)
            },
            "completed" => {
                Ok(PropertyType::COMPLETED)
            },
            "dtend" => {
                Ok(PropertyType::DTEND)
            },
            "due" => {
                Ok(PropertyType::DUE)
            },
            "dtstart" => {
                Ok(PropertyType::DTSTART)
            },
            "duration" => {
                Ok(PropertyType::DURATION)
            },
            "freebusy" => {
                Ok(PropertyType::FREEBUSY)
            },
            "transp" => {
                Ok(PropertyType::TRANSP)
            },
            "tzid" => {
                Ok(PropertyType::TZID)
            },
            "tzname" => {
                Ok(PropertyType::TZNAME)
            },
            "tzoffsetfrom" => {
                Ok(PropertyType::TZOFFSETFROM)
            },
            "tzoffsetto" => {
                Ok(PropertyType::TZOFFSETTO)
            },
            "tzurl" => {
                Ok(PropertyType::TZURL)
            },
            "attendee" => {
                Ok(PropertyType::ATTENDEE)
            },
            "contact" => {
                Ok(PropertyType::CONTACT)
            },
            "organizer" => {
                Ok(PropertyType::ORGANIZER)
            },
            "recurrence_id" => {
                Ok(PropertyType::RECURRENCE_ID)
            },
            "related_to" => {
                Ok(PropertyType::RELATED_TO)
            },
            "url" => {
                Ok(PropertyType::URL)
            },
            "uid" => {
                Ok(PropertyType::UID)
            },
            "exdate" => {
                Ok(PropertyType::EXDATE)
            },
            "exrule" => {
                Ok(PropertyType::EXRULE)
            },
            "rdate" => {
                Ok(PropertyType::RDATE)
            },
            "rrule" => {
                Ok(PropertyType::RRULE)
            },
            "action" => {
                Ok(PropertyType::ACTION)
            },
            "repeat" => {
                Ok(PropertyType::REPEAT)
            },
            "trigger" => {
                Ok(PropertyType::TRIGGER)
            },
            "created" => {
                Ok(PropertyType::CREATED)
            },
            "dtstamp" => {
                Ok(PropertyType::DTSTAMP)
            },
            "last_modified" => {
                Ok(PropertyType::LAST_MODIFIED)
            },
            "sequence" => {
                Ok(PropertyType::SEQUENCE)
            },
            "request_status" => {
                Ok(PropertyType::REQUEST_STATUS)
            },
            unknown => {
                Err(InvalidPropertyError(unknown.to_owned()))
            }
        }
    }
}

#[test]
fn create_property_type() {
    let results_expected = vec![
        ("CALSCALE", Ok(PropertyType::CALSCALE)),
        ("METHOD", Ok(PropertyType::METHOD)),
        ("PRODID", Ok(PropertyType::PRODID)),
        ("VERSION", Ok(PropertyType::VERSION)),
        ("ATTACH", Ok(PropertyType::ATTACH)),
        ("CATEGORIES", Ok(PropertyType::CATEGORIES)),
        ("CLASS", Ok(PropertyType::CLASS)),
        ("COMMENT", Ok(PropertyType::COMMENT)),
        ("DESCRIPTION", Ok(PropertyType::DESCRIPTION)),
        ("GEO", Ok(PropertyType::GEO)),
        ("LOCATION", Ok(PropertyType::LOCATION)),
        ("PERCENT_COMPLETE", Ok(PropertyType::PERCENT_COMPLETE)),
        ("PRIORITY", Ok(PropertyType::PRIORITY)),
        ("RESOURCES", Ok(PropertyType::RESOURCES)),
        ("STATUS", Ok(PropertyType::STATUS)),
        ("SUMMARY", Ok(PropertyType::SUMMARY)),
        ("COMPLETED", Ok(PropertyType::COMPLETED)),
        ("DTEND", Ok(PropertyType::DTEND)),
        ("DUE", Ok(PropertyType::DUE)),
        ("DTSTART", Ok(PropertyType::DTSTART)),
        ("DURATION", Ok(PropertyType::DURATION)),
        ("FREEBUSY", Ok(PropertyType::FREEBUSY)),
        ("TRANSP", Ok(PropertyType::TRANSP)),
        ("TZID", Ok(PropertyType::TZID)),
        ("TZNAME", Ok(PropertyType::TZNAME)),
        ("TZOFFSETFROM", Ok(PropertyType::TZOFFSETFROM)),
        ("TZOFFSETTO", Ok(PropertyType::TZOFFSETTO)),
        ("TZURL", Ok(PropertyType::TZURL)),
        ("ATTENDEE", Ok(PropertyType::ATTENDEE)),
        ("CONTACT", Ok(PropertyType::CONTACT)),
        ("ORGANIZER", Ok(PropertyType::ORGANIZER)),
        ("RECURRENCE_ID", Ok(PropertyType::RECURRENCE_ID)),
        ("RELATED_TO", Ok(PropertyType::RELATED_TO)),
        ("URL", Ok(PropertyType::URL)),
        ("UID", Ok(PropertyType::UID)),
        ("EXDATE", Ok(PropertyType::EXDATE)),
        ("EXRULE", Ok(PropertyType::EXRULE)),
        ("RDATE", Ok(PropertyType::RDATE)),
        ("RRULE", Ok(PropertyType::RRULE)),
        ("ACTION", Ok(PropertyType::ACTION)),
        ("REPEAT", Ok(PropertyType::REPEAT)),
        ("TRIGGER", Ok(PropertyType::TRIGGER)),
        ("CREATED", Ok(PropertyType::CREATED)),
        ("DTSTAMP", Ok(PropertyType::DTSTAMP)),
        ("LAST_MODIFIED", Ok(PropertyType::LAST_MODIFIED)),
        ("SEQUENCE", Ok(PropertyType::SEQUENCE)),
        ("REQUEST_STATUS", Ok(PropertyType::REQUEST_STATUS)),
        ("asdfasdf", Err(InvalidPropertyError("asdfasdf".into()))),
        ];

    for (q, r) in results_expected {
        assert_eq!(PropertyType::from_str(q), r);
    }
}


/// Taken from RFC 5545
#[allow(missing_docs, non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ParameterType {
    ALTREP,
    CN,
    CUTYPE,
    DELEGATED_FROM,
    DELEGATED_TO,
    DIR,
    ENCODING,
    FMTTYPE,
    FBTYPE,
    LANGUAGE,
    MEMBER,
    PARTSTAT,
    RANGE,
    RELATED,
    RELTYPE,
    ROLE,
    RSVP,
    SENT_BY,
    TZID,
    VALUE,
}

impl FromStr for ParameterType {
    type Err = InvalidParameterError;

    fn from_str(s: &str) -> Result<ParameterType, Self::Err> {
        match &s.to_lowercase()[..] {
            "altrep" => {
                Ok(ParameterType::ALTREP)
            },
            "cn" => {
                Ok(ParameterType::CN)
            },
            "cutype" => {
                Ok(ParameterType::CUTYPE)
            },
            "delegated_from" => {
                Ok(ParameterType::DELEGATED_FROM)
            },
            "delegated_to" => {
                Ok(ParameterType::DELEGATED_TO)
            },
            "dir" => {
                Ok(ParameterType::DIR)
            },
            "encoding" => {
                Ok(ParameterType::ENCODING)
            },
            "fmttype" => {
                Ok(ParameterType::FMTTYPE)
            },
            "fbtype" => {
                Ok(ParameterType::FBTYPE)
            },
            "language" => {
                Ok(ParameterType::LANGUAGE)
            },
            "member" => {
                Ok(ParameterType::MEMBER)
            },
            "partstat" => {
                Ok(ParameterType::PARTSTAT)
            },
            "range" => {
                Ok(ParameterType::RANGE)
            },
            "related" => {
                Ok(ParameterType::RELATED)
            },
            "reltype" => {
                Ok(ParameterType::RELTYPE)
            },
            "role" => {
                Ok(ParameterType::ROLE)
            },
            "rsvp" => {
                Ok(ParameterType::RSVP)
            },
            "sent_by" => {
                Ok(ParameterType::SENT_BY)
            },
            "tzid" => {
                Ok(ParameterType::TZID)
            },
            "value" => {
                Ok(ParameterType::VALUE)
            },
            unknown => {
                Err(InvalidParameterError(unknown.to_owned()))
            }
        }
    }
}

#[test]
fn create_parameter_type() {
    let results_expected = vec![
        ("ALTREP", Ok(ParameterType::ALTREP)),
        ("CN", Ok(ParameterType::CN)),
        ("CUTYPE", Ok(ParameterType::CUTYPE)),
        ("DELEGATED_FROM", Ok(ParameterType::DELEGATED_FROM)),
        ("DELEGATED_TO", Ok(ParameterType::DELEGATED_TO)),
        ("DIR", Ok(ParameterType::DIR)),
        ("ENCODING", Ok(ParameterType::ENCODING)),
        ("FMTTYPE", Ok(ParameterType::FMTTYPE)),
        ("FBTYPE", Ok(ParameterType::FBTYPE)),
        ("LANGUAGE", Ok(ParameterType::LANGUAGE)),
        ("MEMBER", Ok(ParameterType::MEMBER)),
        ("PARTSTAT", Ok(ParameterType::PARTSTAT)),
        ("RANGE", Ok(ParameterType::RANGE)),
        ("RELATED", Ok(ParameterType::RELATED)),
        ("RELTYPE", Ok(ParameterType::RELTYPE)),
        ("ROLE", Ok(ParameterType::ROLE)),
        ("RSVP", Ok(ParameterType::RSVP)),
        ("SENT_BY", Ok(ParameterType::SENT_BY)),
        ("TZID", Ok(ParameterType::TZID)),
        ("VALUE", Ok(ParameterType::VALUE)),
        ("asdf", Err(InvalidParameterError("asdf".into()))),
    ];

    for (q, r) in results_expected {
        assert_eq!(ParameterType::from_str(q), r);
    }
}

/// A valid property for it's parent Component
#[derive(Debug)]
pub struct Property {
    /// The type of the property
    pub kind: PropertyType,

    allowed_parameters: Vec<ParameterType>,
}

/// A fully featured and complete Component with only valid attributes
#[derive(Debug)]
pub struct Component {
    /// The type of iCalendar component type we have
    pub kind: ComponentType,

    /// The list of allowed properties
    allowed_properties: Vec<PropertyType>,
}
