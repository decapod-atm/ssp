//! Work-around for const-generic arrays in serde from MikailBag:
//!
//! <https://github.com/serde-rs/serde/issues/1937#issuecomment-812137971>

use heapless::Vec;
use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeTuple,
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::std::{fmt, marker::PhantomData};

/// Serializes a fixed-length array.
pub fn serialize_array<S: Serializer, T: Serialize, const N: usize>(
    data: &[T; N],
    ser: S,
) -> Result<S::Ok, S::Error> {
    let mut s = ser.serialize_tuple(N)?;
    for item in data.iter() {
        s.serialize_element(item)?;
    }
    s.end()
}

/// Serializes a fixed-length [Vec].
pub fn serialize_vec<S: Serializer, T: Serialize, const N: usize>(
    data: &Vec<T, N>,
    ser: S,
) -> Result<S::Ok, S::Error> {
    let mut s = ser.serialize_tuple(data.len())?;
    for item in data.iter() {
        s.serialize_element(item)?;
    }
    s.end()
}

struct ArrayVisitor<T, const N: usize>(PhantomData<T>);
struct VecVisitor<T, const N: usize>(PhantomData<T>);

impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
where
    T: Deserialize<'de>,
{
    type Value = [T; N];

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!("an array of length {N}"))
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        // can be optimized using MaybeUninit
        let mut data = Vec::<T, N>::new();
        for _ in 0..N {
            match (seq.next_element())? {
                Some(val) => {
                    let _ = data.push(val).ok();
                }
                None => return Err(serde::de::Error::invalid_length(N, &self)),
            }
        }
        match data.into_array() {
            Ok(arr) => Ok(arr),
            Err(_) => Err(serde::de::Error::invalid_length(N, &self)),
        }
    }
}

impl<'de, T, const N: usize> Visitor<'de> for VecVisitor<T, N>
where
    T: Deserialize<'de>,
{
    type Value = Vec<T, N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!("an array of length {N}"))
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        // can be optimized using MaybeUninit
        let mut data = Vec::<T, N>::new();
        while let Some(val) = seq.next_element()? {
            if data.push(val).is_err() {
                break;
            }
        }
        Ok(data)
    }
}

/// Deserializer method for fixed-length arrays.
pub fn deserialize_array<'de, D, T, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    deserializer.deserialize_tuple(N, ArrayVisitor::<T, N>(PhantomData))
}

/// Deserializer method for fixed-length vectors.
pub fn deserialize_vec<'de, D, T, const N: usize>(deserializer: D) -> Result<Vec<T, N>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    deserializer.deserialize_tuple(N, VecVisitor::<T, N>(PhantomData))
}
