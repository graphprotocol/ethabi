//! Event param specification.

use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::{fmt, io};
use ParamType;
use {errors, Constructor, ErrorKind, Event, Function};

/// Event param specification.
#[derive(Debug, Clone, PartialEq)]
pub struct EventParam {
	/// Param name.
	pub name: String,
	/// Param type.
	pub kind: ParamType,
	/// Indexed flag. If true, param is used to build block bloom.
	pub indexed: bool,
}

pub struct TupleParams {
    params: Vec<EventParam>,
}

impl<'a> Deserialize<'a> for TupleParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_seq(TupleParamsVisitor)
    }
}

impl<'a> Deserialize<'a> for EventParam {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_any(EventParamVisitor)
    }
}

struct EventParamVisitor;

impl<'a> Visitor<'a> for EventParamVisitor {
    type Value = EventParam;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a valid event parameter spec")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'a>,
    {
        let mut name = None;
        let mut kind = None;
        let mut indexed = None;
        let mut components = None;

        while let Some(ref key) = map.next_key::<String>()? {
            match key.as_ref() {
                "name" => {
                    if name.is_some() {
                        return Err(Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value()?);
                }
                "type" => {
                    if kind.is_some() {
                        return Err(Error::duplicate_field("kind"));
                    }
                    kind = Some(map.next_value()?);
                }
                "components" => {
                    if components.is_some() {
                        return Err(Error::duplicate_field("components"));
                    }
                    let component: TupleParams = map.next_value()?;
                    components = Some(component)
                }
                "indexed" => {
                    if indexed.is_some() {
                        return Err(Error::duplicate_field("indexed"));
                    }
                    indexed = Some(map.next_value()?);
                }
                _ => {}
            }
        }
        let name = name.ok_or_else(|| Error::missing_field("name"))?;
        let kind = kind.ok_or_else(|| Error::missing_field("kind")).map(|t| {
            if let ParamType::Tuple(_) = t {
                components.map(|component| component.params).map(|ps| {
                    ParamType::Tuple(ps.into_iter()
                        .map(|param| Box::new(param.kind))
                        .collect::<Vec<Box<ParamType>>>())
                }).unwrap()
            } else {
                t
            }
        })?;
        let indexed = indexed.unwrap_or(false);
        Ok(EventParam {
            name,
            kind,
            indexed,
        })
    }
}

struct TupleParamsVisitor;
impl<'a> Visitor<'a> for TupleParamsVisitor {
    type Value = TupleParams;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a valid event parameter spec")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'a>,
    {
        let mut params: Vec<EventParam> = Vec::new();

        while let Some(param) = seq.next_element()? {
            let p: EventParam = param;
            params.push(p);
        }

        Ok(TupleParams { params })
    }
}

#[cfg(test)]
mod tests {
	use serde_json;
	use {EventParam, ParamType};

	#[test]
	fn event_param_deserialization() {
		let s = r#"{
			"name": "foo",
			"type": "address",
			"indexed": true
		}"#;

		let deserialized: EventParam = serde_json::from_str(s).unwrap();

		assert_eq!(deserialized, EventParam {
			name: "foo".to_owned(),
			kind: ParamType::Address,
			indexed: true,
		});
	}
    #[test]
    fn event_param_tuple_deserialization() {
        let s = r#"{
            "name": "foo",
            "type": "tuple",
            "indexed": true,
            "components": [
                {
                    "name": "baseToken",
                    "type": "address"
                },
                {
                    "name": "startDate",
                    "type": "uint48"
                }
            ]
        }"#;

        let deserialized: EventParam = serde_json::from_str(s).unwrap();

        assert_eq!(
            deserialized,
            EventParam {
                name: "foo".to_owned(),
                kind: ParamType::Tuple(vec![Box::new(ParamType::Address),Box::new(ParamType::Uint(48))]),
                indexed: true,
            }
        );
    }
}
