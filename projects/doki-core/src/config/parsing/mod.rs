use super::*;

pub fn parse_null_as_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

pub fn parse_url_base<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct Accepted;

    impl<'de> Visitor<'de> for Accepted {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("url_base: Null | String | List<String>")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
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
        where
            S: SeqAccess<'de>,
        {
            Deserialize::deserialize(SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(Accepted)
}

pub fn parse_url_end<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct Accepted;

    impl<'de> Visitor<'de> for Accepted {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("url_end: '/' | '.extension'")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let mut out = String::new();
            match s.trim() {
                "/" => out = "/".to_string(),
                s if s.starts_with('.') => {
                    let mut c = s.chars();
                    c.next(); // pop .
                    if c.all(char::is_alphanumeric) {
                        out = s.to_string()
                    }
                }
                s if s.chars().all(char::is_alphanumeric) => out = format!(".{}", s),
                _ => (),
            };
            match out.is_empty() {
                true => Err(E::custom("url_end: '/' | '.extension'")),
                false => Ok(Some(out)),
            }
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(Accepted)
}
