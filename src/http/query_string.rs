use std::collections::{HashMap};
use std::convert::From;

#[derive(Debug)]
pub enum Value<'buffer_lifetime> {
    Single(&'buffer_lifetime str),
    Multiple(Vec<&'buffer_lifetime str>)
}

#[derive(Debug)]
pub struct QueryString<'buffer_lifetime> {
    pub data: HashMap<&'buffer_lifetime str, Value<'buffer_lifetime>>
}



impl<'buffer_lifetime> From<&'buffer_lifetime str> for QueryString<'buffer_lifetime> {
    fn from(s: &'buffer_lifetime str) -> Self {
        let mut data = HashMap::new();

        for sub_string in s.split("&") {
            let mut key = sub_string;
            let mut value = "";

            if let Some(i) = s.find("=") {
                key = &sub_string[..i];
                value = &sub_string[i+1..];
            }

            data.entry(key)
            .and_modify(|existing|match existing {
                Value::Single(prev_val) => {
                    *existing = Value::Multiple(vec![prev_val, value])
                },
                Value::Multiple(prev_val) => prev_val.push(value)
            })
            .or_insert(Value::Single(value));
        }
        QueryString { data }
    }
}