use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CFResponse<T> {
    pub data: T,
}

// FFS CF YOUR DOCS DONT MATCH THE ACTUAL RESPONSE BECAUSE FUCK YOU
#[derive(Clone)]
pub struct CFString {
    inner: Option<String>,
}

impl Serialize for CFString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.inner {
            Some(ref value) => serializer.serialize_some(value),
            None => serializer.serialize_none(),
        }
    }
}

impl From<CFString> for String {
    fn from(val: CFString) -> Self {
        val.get()
    }
}

impl From<String> for CFString {
    fn from(val: String) -> Self {
        Self { inner: Some(val) }
    }
}

impl<'de> Deserialize<'de> for CFString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            inner: deserializer.deserialize_option(CFVisitor {
                marker: PhantomData,
            })?,
        })
    }
}

impl Debug for CFString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.get(), f)
    }
}

impl Display for CFString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}

impl CFString {
    pub fn get(&self) -> String {
        match &self.inner {
            Some(s) => s.clone(),
            None => String::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.as_ref().map(|x| x.is_empty()).unwrap_or(true)
    }
}

struct CFVisitor {
    marker: PhantomData<String>,
}

impl<'de> Visitor<'de> for CFVisitor {
    type Value = Option<String>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("wrapper for Option<String>")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Some)
    }
}

#[macro_export]
macro_rules! cf_enum {
    ($name:ident,
        [$($meta:meta),*],
        $($ident:ident=$id:literal),*,
        @$last:ident=$last_id:literal) => {
            cf_enum!{
                $name, [$($meta),*],
                @$last = $last_id,
                $($ident = $id),*
            }
    };

    ($name:ident,
        [$($meta:meta),*],
        $($first_ident:ident=$first_id:literal),*,
        @$middle:ident=$middle_id:literal,
        $($last_ident:ident=$last_id:literal),*) => {
            cf_enum!{$name, [$($meta),*],
                @$middle = $middle_id,
                $($first_ident = $first_id),*,
                $($last_ident = $last_id),*}
    };

    ($name:ident,
    [$($meta:meta),*],
    @$first:ident=$first_id:literal,
    $($ident:ident=$id:literal),*$(,)?) => {
        $(#[$meta])*
        #[derive(Default)]
        #[repr(u8)]
        #[allow(dead_code)]
        pub enum $name {
            #[default]
            $first = $first_id,
            $($ident = $id,)*
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>
            {
                match <Option<u8> as serde::Deserialize>::deserialize(deserializer)? {
                    Some($first_id) => Ok($name::$first),
                    $(Some($id) => Ok($name::$ident),)*
                    Some(_) => Ok($name::$first),
                    None => Ok($name::$first),
                }
            }
        }
    };
}

cf_enum! {FileReleaseType, [],
    @Release = 1,
    Beta = 2,
    Alpha = 3
}
