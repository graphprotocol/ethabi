use serde::de::{Error, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::{fmt};
use ParamType;

/// Tuple params specification
#[derive(Debug, Clone, PartialEq)]
pub struct TupleParams(pub Vec<Box<ParamType>>);

impl<'a> Deserialize<'a> for TupleParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'a>,
    {
        deserializer.deserialize_seq(TupleParamsVisitor)
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
        let mut params: Vec<Box<ParamType>> = Vec::new();

        while let Some(param) = seq.next_element()? {
            let p: Value = param;
            let kind: &Value = p.get("type")
                .ok_or_else(|| Error::custom("Invalid tuple param type"))?;
            params.push(Box::new(ParamType::deserialize(kind).unwrap()));
        }

        Ok(TupleParams(params))
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use ParamType;
    use TupleParams;

    #[test]
    fn event_param_deserialization() {
        let s = r#"[{
			"name": "foo",
			"type": "address"
			},{
			"name": "foo",
			"type": "address"
			},{
			"name": "foo",
			"type": "address"
			},{
			"name": "foo",
			"type": "bool"
			}
		]"#;

        let deserialized: TupleParams = serde_json::from_str(s).unwrap();

        assert_eq!(deserialized, TupleParams (vec![Box::new(ParamType::Address),Box::new(ParamType::Address),Box::new(ParamType::Address),Box::new(ParamType::Bool)]));
    }
}
