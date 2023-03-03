use crate::ser::part::{PartSerializer, Sink};
use crate::ser::Error;
use form_urlencoded::Serializer as UrlEncodedSerializer;
use form_urlencoded::Target as UrlEncodedTarget;
use serde::ser::Serialize;
use std::str;

pub struct ValueSink<'input, 'key, 'target, Target>
where
    Target: UrlEncodedTarget,
{
    urlencoder: &'target mut UrlEncodedSerializer<'input, Target>,
    key: &'key str,
}

impl<'input, 'key, 'target, Target> ValueSink<'input, 'key, 'target, Target>
where
    Target: 'target + UrlEncodedTarget,
{
    pub fn new(
        urlencoder: &'target mut UrlEncodedSerializer<'input, Target>,
        key: &'key str,
    ) -> Self {
        ValueSink { urlencoder, key }
    }
}

impl<'input, 'key, 'target, Target> Sink
    for ValueSink<'input, 'key, 'target, Target>
where
    Target: 'target + UrlEncodedTarget,
{
    type Ok = ();

    fn serialize_str(&mut self, value: &str) -> Result<(), Error> {
        self.urlencoder.append_pair(self.key, value);
        Ok(())
    }

    fn serialize_static_str(&mut self, value: &'static str) -> Result<(), Error> {
        self.serialize_str(value)
    }

    fn serialize_string(&mut self, value: String) -> Result<(), Error> {
        self.serialize_str(&value)
    }

    fn serialize_none(&mut self) -> Result<Self::Ok, Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(
        &mut self,
        value: &T,
    ) -> Result<Self::Ok, Error> {
        value.serialize(&mut PartSerializer::new(self))
    }

    fn unsupported(&mut self) -> Error {
        Error::Custom("unsupported value".into())
    }
}
