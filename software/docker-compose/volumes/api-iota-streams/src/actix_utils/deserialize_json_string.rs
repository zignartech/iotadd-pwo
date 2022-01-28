use serde::de;

pub fn deserializeJsonString<'de, D, I>(deserializer: D) -> Result<I, D::Error>
where
  D: de::Deserializer<'de>,
  I: de::DeserializeOwned,
{
  struct JsonStringVisitor<I>(std::marker::PhantomData<I>);

  impl<'de, I> de::Visitor<'de> for JsonStringVisitor<I>
  where
    I: de::DeserializeOwned,
  {
    type Value = I;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
      formatter.write_str("a string containing json data")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      serde_json::from_str(v).map_err(E::custom)
    }
  }
  deserializer.deserialize_any(JsonStringVisitor(std::marker::PhantomData::<I>))
}
