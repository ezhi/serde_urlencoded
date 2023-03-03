use crate::ser::Error;
use serde::ser;
use std::str;

pub struct PartSerializer<'a, S> {
    sink: &'a mut S,
}

impl<'a, S: Sink> PartSerializer<'a, S> {
    pub fn new(sink: &'a mut S) -> Self {
        PartSerializer { sink }
    }
}

pub struct SeqSerializer<'a, S> {
    sink: &'a mut S,
    size: Option<usize>,
}

impl<'a, S: Sink> SeqSerializer<'a, S> {
    fn new(sink: &'a mut S, size: Option<usize>) -> Self {
        Self { sink, size }
    }
}

impl<'a, S: Sink> ser::SerializeSeq for SeqSerializer<'a, S> {
    type Ok = S::Ok;
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Error> {
        self.sink.serialize_some(value).map(|_| ())
    }

    fn end(self) -> Result<Self::Ok, Error> {
        match self.size {
            Some(0) => self.sink.serialize_some(""),
            _       => self.sink.serialize_none(),
        }
    }
}

pub trait Sink: Sized {
    type Ok;

    fn serialize_static_str(
        &mut self,
        value: &'static str,
    ) -> Result<Self::Ok, Error>;

    fn serialize_str(&mut self, value: &str) -> Result<Self::Ok, Error>;
    fn serialize_string(&mut self, value: String) -> Result<Self::Ok, Error>;
    fn serialize_none(&mut self) -> Result<Self::Ok, Error>;

    fn serialize_some<T: ?Sized + ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<Self::Ok, Error>;

    fn unsupported(&mut self) -> Error;
}

impl<'a, S: Sink> ser::Serializer for &'a mut PartSerializer<'a, S> {
    type Ok = S::Ok;
    type Error = Error;
    type SerializeSeq = SeqSerializer<'a, S>;
    type SerializeTuple = ser::Impossible<S::Ok, Error>;
    type SerializeTupleStruct = ser::Impossible<S::Ok, Error>;
    type SerializeTupleVariant = ser::Impossible<S::Ok, Error>;
    type SerializeMap = ser::Impossible<S::Ok, Error>;
    type SerializeStruct = ser::Impossible<S::Ok, Error>;
    type SerializeStructVariant = ser::Impossible<S::Ok, Error>;

    fn serialize_bool(self, v: bool) -> Result<S::Ok, Error> {
        self.sink
            .serialize_static_str(if v { "true" } else { "false" })
    }

    fn serialize_i8(self, v: i8) -> Result<S::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_i16(self, v: i16) -> Result<S::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_i32(self, v: i32) -> Result<S::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_i64(self, v: i64) -> Result<S::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u8(self, v: u8) -> Result<S::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u16(self, v: u16) -> Result<S::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u32(self, v: u32) -> Result<S::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u64(self, v: u64) -> Result<S::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u128(self, v: u128) -> Result<S::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_i128(self, v: i128) -> Result<S::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_f32(self, v: f32) -> Result<S::Ok, Error> {
        self.serialize_floating(v)
    }

    fn serialize_f64(self, v: f64) -> Result<S::Ok, Error> {
        self.serialize_floating(v)
    }

    fn serialize_char(self, v: char) -> Result<S::Ok, Error> {
        self.sink.serialize_string(v.to_string())
    }

    fn serialize_str(self, value: &str) -> Result<S::Ok, Error> {
        self.sink.serialize_str(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<S::Ok, Error> {
        match str::from_utf8(value) {
            Ok(value) => self.sink.serialize_str(value),
            Err(err) => Err(Error::Utf8(err)),
        }
    }

    fn serialize_unit(self) -> Result<S::Ok, Error> {
        Err(self.sink.unsupported())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<S::Ok, Error> {
        self.sink.serialize_static_str(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<S::Ok, Error> {
        self.sink.serialize_static_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<S::Ok, Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<S::Ok, Error> {
        Err(self.sink.unsupported())
    }

    fn serialize_none(self) -> Result<S::Ok, Error> {
        self.sink.serialize_none()
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(
        self,
        value: &T,
    ) -> Result<S::Ok, Error> {
        self.sink.serialize_some(value)
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Error> {
        Ok(Self::SerializeSeq::new(self.sink, len))
    }

    fn serialize_tuple(
        self,
        _len: usize,
    ) -> Result<Self::SerializeTuple, Error> {
        Err(self.sink.unsupported())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTuple, Error> {
        Err(self.sink.unsupported())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Err(self.sink.unsupported())
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeMap, Error> {
        Err(self.sink.unsupported())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        Err(self.sink.unsupported())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Err(self.sink.unsupported())
    }
}

impl<'a, S: Sink> PartSerializer<'a, S> {
    fn serialize_integer<I>(&'a mut self, value: I) -> Result<S::Ok, Error>
    where
        I: itoa::Integer,
    {
        let mut buf = itoa::Buffer::new();
        let part = buf.format(value);
        ser::Serializer::serialize_str(self, part)
    }

    fn serialize_floating<F>(&'a mut self, value: F) -> Result<S::Ok, Error>
    where
        F: ryu::Float,
    {
        let mut buf = ryu::Buffer::new();
        let part = buf.format(value);
        ser::Serializer::serialize_str(self, part)
    }
}
