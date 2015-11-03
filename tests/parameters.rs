extern crate icalendar;
use icalendar::component::{Parameter, ParameterType};
use std::str::FromStr;

#[test]
fn test_altrep() {
    let param = Parameter::from_str("ALTREP=\"http://what!?\"");
    assert!(param.is_ok());

    if let Ok(param) = param {
        assert!(param.is_a(ParameterType::ALTREP));
        assert_eq!(param.get_value(), "http://what!?");
    }
}
