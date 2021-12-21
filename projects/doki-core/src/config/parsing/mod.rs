use super::*;

pub fn parse_base_url<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where D: Deserializer<'de>
{
    struct Accepted;

    impl<'de> Visitor<'de> for Accepted {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("base_url: Null | String | List<String>")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where E: de::Error
        {
            Ok(s.trim_matches('/').split('/').map(|s| s.to_string()).collect())
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
        {
            Ok(vec![])
        }

        fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
            where S: SeqAccess<'de>
        {
            Deserialize::deserialize(SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(Accepted)
}