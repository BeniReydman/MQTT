use custom_error::custom_error;

custom_error!{pub MyError
    TomlParseError      = "Received Empty Config!",
    SerializeError      = "Couldn't serialize"
}