/// Creates an named boolean-like enum (set or unset enums).
///
/// Implements utility traits for converting from/to basic types.
#[macro_export]
macro_rules! bool_enum {
    ($name:ident, $doc:tt) => {
        #[doc = $doc]
        #[repr(u8)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum $name {
            /// The field is unset
            Unset = 0b0,
            /// The field is set
            Set = 0b1,
        }

        impl $name {
            /// Creates an item from a byte.
            pub const fn from_u8(b: u8) -> Self {
                Self::from_bool((b & 0b1) != 0)
            }

            /// Creates an item from a bool.
            pub const fn from_bool(b: bool) -> Self {
                match b {
                    false => Self::Unset,
                    true => Self::Set,
                }
            }
        }

        impl From<bool> for $name {
            fn from(b: bool) -> Self {
                Self::from_bool(b)
            }
        }

        impl From<u8> for $name {
            fn from(b: u8) -> Self {
                Self::from_u8(b)
            }
        }

        impl From<$name> for bool {
            fn from(n: $name) -> Self {
                n == $name::Set
            }
        }

        impl From<&$name> for bool {
            fn from(n: &$name) -> Self {
                (*n).into()
            }
        }

        impl From<$name> for u8 {
            fn from(n: $name) -> Self {
                (n == $name::Set) as u8
            }
        }

        impl From<&$name> for u8 {
            fn from(n: &$name) -> Self {
                (*n).into()
            }
        }

        impl From<&$name> for &'static str {
            fn from(name: &$name) -> Self {
                let set: bool = name.into();
                if set {
                    "set"
                } else {
                    "unset"
                }
            }
        }

        impl From<$name> for &'static str {
            fn from(name: $name) -> Self {
                (&name).into()
            }
        }

        impl $crate::std::fmt::Display for $name {
            fn fmt(&self, f: &mut $crate::std::fmt::Formatter<'_>) -> $crate::std::fmt::Result {
                write!(f, "{}", <&'static str>::from(self))
            }
        }

        impl $crate::std::ops::Not for $name {
            type Output = $name;

            fn not(self) -> Self {
                Self::from(!bool::from(self))
            }
        }
    };

    ($name:ident) => {
        bool_enum!($name, "");
    };
}

/// Creates an named anti-boolean-like enum (set(false) or unset(true) enums).
///
/// Implements utility traits for converting from/to basic types.
#[macro_export]
macro_rules! anti_bool_enum {
    ($name:ident, $doc:tt) => {
        #[doc = $doc]
        #[repr(u8)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum $name {
            /// The field is unset
            Unset = 0b1,
            /// The field is set
            Set = 0b0,
        }

        impl $name {
            /// Creates an item from a byte.
            pub const fn from_u8(b: u8) -> Self {
                Self::from_bool((b & 0b1) != 0)
            }

            /// Creates an item from a bool.
            pub const fn from_bool(b: bool) -> Self {
                match b {
                    true => Self::Unset,
                    false => Self::Set,
                }
            }
        }

        impl From<bool> for $name {
            fn from(b: bool) -> Self {
                Self::from_bool(b)
            }
        }

        impl From<u8> for $name {
            fn from(b: u8) -> Self {
                Self::from_u8(b)
            }
        }

        impl From<$name> for bool {
            fn from(n: $name) -> Self {
                n == $name::Unset
            }
        }

        impl From<&$name> for bool {
            fn from(n: &$name) -> Self {
                (*n).into()
            }
        }

        impl From<$name> for u8 {
            fn from(n: $name) -> Self {
                (n == $name::Unset) as u8
            }
        }

        impl From<&$name> for u8 {
            fn from(n: &$name) -> Self {
                (*n).into()
            }
        }

        impl From<&$name> for &'static str {
            fn from(name: &$name) -> Self {
                let unset: bool = name.into();
                if unset {
                    "unset"
                } else {
                    "set"
                }
            }
        }

        impl From<$name> for &'static str {
            fn from(name: $name) -> Self {
                (&name).into()
            }
        }

        impl $crate::std::fmt::Display for $name {
            fn fmt(&self, f: &mut $crate::std::fmt::Formatter<'_>) -> $crate::std::fmt::Result {
                write!(f, "{}", <&'static str>::from(self))
            }
        }

        impl $crate::std::ops::Not for $name {
            type Output = $name;

            fn not(self) -> Self {
                Self::from(!bool::from(self))
            }
        }
    };

    ($name:ident) => {
        bool_enum!($name, "");
    };
}

/// Creates a tuple struct with a given name, base type, and documentation.
#[macro_export]
macro_rules! tuple_struct {
    ($name:ident, $base:ident, $doc:tt) => {
        #[doc = $doc]
        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name(pub $base);

        impl $name {
            /// Creates a new tuple struct type.
            pub fn new() -> Self {
                Self($base::default())
            }

            /// Converts an inner type into a new tuple struct.
            pub const fn from_inner(b: $base) -> Self {
                Self(b)
            }

            /// Converts a tuple struct into its inner type.
            pub const fn as_inner(&self) -> $base {
                self.0
            }
        }

        impl From<$base> for $name {
            fn from(val: $base) -> Self {
                Self::from_inner(val)
            }
        }

        impl From<$name> for $base {
            fn from(val: $name) -> Self {
                val.as_inner()
            }
        }

        impl From<&$name> for $base {
            fn from(val: &$name) -> Self {
                val.as_inner()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

/// Creates a serializable tuple struct with a given name, base type, and documentation.
#[macro_export]
macro_rules! tuple_struct_ser {
    ($name:ident, $base:ident, $doc:tt) => {
        #[doc = $doc]
        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct $name(pub $base);

        impl $name {
            /// Creates a new tuple struct type.
            pub fn new() -> Self {
                Self($base::default())
            }

            /// Converts an inner type into a new tuple struct.
            pub const fn from_inner(b: $base) -> Self {
                Self(b)
            }

            /// Converts a tuple struct into its inner type.
            pub const fn as_inner(&self) -> $base {
                self.0
            }
        }

        impl From<$base> for $name {
            fn from(val: $base) -> Self {
                Self::from_inner(val)
            }
        }

        impl From<$name> for $base {
            fn from(val: $name) -> Self {
                val.as_inner()
            }
        }

        impl From<&$name> for $base {
            fn from(val: &$name) -> Self {
                val.as_inner()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

/// Creates a named type to represent a key struct used in encryption.
#[macro_export]
macro_rules! make_key {
    ($name:ident, $base:ident, $doc:tt) => {
        #[doc = $doc]
        #[repr(C)]
        #[derive(Clone, Debug, PartialEq, zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
        pub struct $name(pub $base);

        impl $name {
            /// Converts an inner type into a new tuple struct.
            pub const fn from_inner(b: $base) -> Self {
                Self(b)
            }

            /// Converts a tuple struct into its inner type.
            pub const fn as_inner(&self) -> $base {
                self.0
            }
        }

        impl From<$base> for $name {
            fn from(val: $base) -> Self {
                Self::from_inner(val)
            }
        }

        impl From<$name> for $base {
            fn from(val: $name) -> Self {
                val.as_inner()
            }
        }

        impl From<&$name> for $base {
            fn from(val: &$name) -> Self {
                val.as_inner()
            }
        }
    };
}

/// Implements the [Default] trait for named types with `Self::new()` function.
#[macro_export]
macro_rules! impl_default {
    ($name:ident) => {
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

/// Implements the defaults of the [MessageOps](crate::MessageOps) trait for a named message type.
///
/// The type must have a `buf` data member that can be converted to a slice.
#[macro_export]
macro_rules! impl_message_ops {
    ($name:ident) => {
        impl $crate::MessageOps for $name {
            fn buf(&self) -> &[u8] {
                self.buf.as_ref()
            }

            fn buf_mut(&mut self) -> &mut [u8] {
                self.buf.as_mut()
            }

            fn message_type(&self) -> $crate::MessageType {
                self.command()
            }

            fn is_command(&self) -> bool {
                true
            }
        }
    };

    ($name:ident, $msg_type:ident::$variant:tt) => {
        impl $crate::MessageOps for $name {
            fn buf(&self) -> &[u8] {
                self.buf.as_ref()
            }

            fn buf_mut(&mut self) -> &mut [u8] {
                self.buf.as_mut()
            }

            fn message_type(&self) -> $msg_type {
                $msg_type::$variant
            }

            fn is_command(&self) -> bool {
                false
            }
        }
    };
}

/// Implement the defaults for the [MessageOps](crate::MessageOps) trait for a named message type
/// with variable length.
#[macro_export]
macro_rules! impl_var_message_ops {
    ($name:ident) => {
        impl $crate::MessageOps for $name {
            fn buf(&self) -> &[u8] {
                self.buf.as_ref()
            }

            fn buf_mut(&mut self) -> &mut [u8] {
                self.buf.as_mut()
            }

            fn message_type(&self) -> $crate::MessageType {
                self.buf[$crate::message::index::COMMAND].into()
            }

            fn is_command(&self) -> bool {
                true
            }

            fn is_variable(&self) -> bool {
                true
            }

            fn len(&self) -> usize {
                // Read the LEN message field, because the actual length is variable.
                (self.buf[$crate::message::index::LEN] as usize) + $crate::len::METADATA
            }
        }
    };

    ($name:ident, $msg_type:ident::$variant:tt) => {
        impl MessageOps for $name {
            fn buf(&self) -> &[u8] {
                self.buf.as_ref()
            }

            fn buf_mut(&mut self) -> &mut [u8] {
                self.buf.as_mut()
            }

            fn message_type(&self) -> $msg_type {
                $msg_type::$variant
            }

            fn is_command(&self) -> bool {
                false
            }

            fn is_variable(&self) -> bool {
                true
            }

            fn len(&self) -> usize {
                // Read the LEN message field, because the actual length is variable.
                (self.buf[$crate::message::index::LEN] as usize) + $crate::len::METADATA
            }
        }
    };
}

/// Implement the defaults for the [MessageOps](crate::MessageOps) trait for an encrypted message type
/// with variable length.
#[macro_export]
macro_rules! impl_encrypted_message_ops {
    ($name:ident) => {
        impl $crate::MessageOps for $name {
            fn buf(&self) -> &[u8] {
                self.buf.as_ref()
            }

            fn buf_mut(&mut self) -> &mut [u8] {
                self.buf.as_mut()
            }

            fn message_type(&self) -> $crate::MessageType {
                self.buf[$crate::encrypted::index::COMMAND].into()
            }

            fn is_command(&self) -> bool {
                true
            }

            fn is_variable(&self) -> bool {
                true
            }

            fn init(&mut self) {
                let buf = self.buf_mut();

                buf[$crate::encrypted::index::STEX] = $crate::encrypted::STEX;

                let count_start = $crate::encrypted::index::COUNT;
                let count_end = $crate::encrypted::index::COUNT_END;
                let count = $crate::encrypted::sequence_count().as_inner().to_be_bytes();

                buf[count_start..count_end].copy_from_slice(count.as_ref());
            }

            fn len(&self) -> usize {
                // Read the LEN message field, because the actual length is variable.
                let meta = self.metadata_len();
                let data_len = self.data_len();
                let packing_len = $crate::len::aes_packing_len(data_len + meta - 1);

                meta + data_len + packing_len
            }

            fn data_len(&self) -> usize {
                let buf = self.buf();
                let len_idx = $crate::encrypted::index::LEN;

                buf[len_idx] as usize
            }

            fn set_data_len(&mut self, len: u8) {
                self.buf_mut()[$crate::encrypted::index::LEN] = len;
            }

            fn metadata_len(&self) -> usize {
                $crate::len::ENCRYPTED_METADATA
            }

            fn calculate_checksum(&mut self) -> u16 {
                use $crate::encrypted::index;

                let len = self.len();
                let buf = self.buf_mut();
                let crc_start = len - 2;

                let crc = $crate::crc::crc16(buf[index::LEN..crc_start].as_ref());
                buf[crc_start..len].copy_from_slice(crc.to_le_bytes().as_ref());

                crc
            }

            fn verify_checksum(&self) -> Result<()> {
                use $crate::encrypted::index;

                let buf = self.buf();
                let len = self.len();

                let crc = self.checksum();
                let exp_crc = $crate::crc::crc16(buf[index::LEN..len - 2].as_ref());

                if crc == exp_crc {
                    Ok(())
                } else {
                    Err(Error::Crc((crc, exp_crc)))
                }
            }
        }
    };

    ($name:ident, $msg_type:ident::$variant:tt) => {
        impl MessageOps for $name {
            fn buf(&self) -> &[u8] {
                self.buf.as_ref()
            }

            fn buf_mut(&mut self) -> &mut [u8] {
                self.buf.as_mut()
            }

            fn message_type(&self) -> $msg_type {
                $msg_type::$variant
            }

            fn is_command(&self) -> bool {
                false
            }

            fn is_variable(&self) -> bool {
                true
            }

            fn init(&mut self) {
                let buf = self.buf_mut();

                buf[$crate::encrypted::index::STEX] = $crate::encrypted::STEX;

                let count_start = $crate::encrypted::index::COUNT;
                let count_end = $crate::encrypted::index::COUNT_END;
                let count = $crate::encrypted::sequence_count().as_inner().to_be_bytes();

                buf[count_start..count_end].copy_from_slice(count.as_ref());
            }

            fn len(&self) -> usize {
                // Read the LEN message field, because the actual length is variable.
                let meta = self.metadata_len();
                let data_len = self.data_len();
                let packing_len = $crate::len::aes_packing_len(data_len + meta - 1);

                meta + data_len + packing_len
            }

            fn data_len(&self) -> usize {
                let buf = self.buf();
                let len_idx = ;
                let inited = buf[len_idx] != 0;

                buf[len_idx] as usize
            }

            fn metadata_len(&self) -> usize {
                $crate::len::ENCRYPTED_METADATA
            }

            fn calculate_checksum(&mut self) -> u16 {
                use $crate::encrypted::index;

                let len = self.len();
                let buf = self.buf_mut();
                let crc_start = len - 2;

                let crc = $crate::crc::crc16(buf[index::LEN..crc_start].as_ref());
                buf[crc_start..len].copy_from_slice(crc.to_le_bytes().as_ref());

                crc
            }

            fn verify_checksum(&self) -> Result<()> {
                use $crate::encrypted::index;

                let buf = self.buf();
                let len = self.len();

                let crc = self.checksum();
                let exp_crc = $crate::crc::crc16(buf[index::LEN..len - 2].as_ref());

                if crc == exp_crc {
                    Ok(())
                } else {
                    Err(Error::Crc((crc, exp_crc)))
                }
            }
        }
    };
}

/// Implement the defaults for the [MessageOps](crate::MessageOps) trait for a named message type
/// with variable length.
#[macro_export]
macro_rules! impl_wrapped_message_ops {
    ($name:ident) => {
        impl $crate::MessageOps for $name {
            fn buf(&self) -> &[u8] {
                self.buf.as_ref()
            }

            fn buf_mut(&mut self) -> &mut [u8] {
                self.buf.as_mut()
            }

            fn message_type(&self) -> $crate::MessageType {
                $crate::MessageType::Encrypted
            }

            fn is_command(&self) -> bool {
                true
            }

            fn is_response(&self) -> bool {
                true
            }

            fn is_variable(&self) -> bool {
                true
            }

            fn len(&self) -> usize {
                self.data_len() + $crate::len::METADATA
            }
        }
    };
}

/// Implements the defaults for the [CommandOps](crate::CommandOps) trait for a named message type.
#[macro_export]
macro_rules! impl_command_ops {
    ($name:ident) => {
        impl $crate::CommandOps for $name {}
    };
}

/// Implements the defaults for the [ResponseOps](crate::ResponseOps) trait for a named message type.
#[macro_export]
macro_rules! impl_response_ops {
    ($name:ident) => {
        impl $crate::ResponseOps for $name {}
    };
}

/// Implements the default formatter for [MessageOps](crate::MessageOps) named types.
#[macro_export]
macro_rules! impl_message_display {
    ($name:ident) => {
        impl $crate::std::fmt::Display for $name {
            fn fmt(&self, f: &mut $crate::std::fmt::Formatter<'_>) -> $crate::std::fmt::Result {
                let msg: &dyn $crate::MessageOps = self;
                write!(f, "{msg}")
            }
        }
    };
}

/// Implements the default formatter for [CommandOps](crate::CommandOps) named types.
#[macro_export]
macro_rules! impl_command_display {
    ($name:ident) => {
        impl $crate::std::fmt::Display for $name {
            fn fmt(&self, f: &mut $crate::std::fmt::Formatter<'_>) -> $crate::std::fmt::Result {
                let cmd: &dyn $crate::CommandOps = self;
                write!(f, "{cmd}")
            }
        }
    };
}

/// Implements the default formatter for [ResponseOps](crate::ResponseOps) named types.
#[macro_export]
macro_rules! impl_response_display {
    ($name:ident) => {
        impl $crate::std::fmt::Display for $name {
            fn fmt(&self, f: &mut $crate::std::fmt::Formatter<'_>) -> $crate::std::fmt::Result {
                let res: &dyn $crate::ResponseOps = self;
                write!(f, "{res}")
            }
        }
    };
}

/// Implement `From<_>` traits to convert buffer types into a named message type.
#[macro_export]
macro_rules! impl_message_from_buf {
    ($name:ident) => {
        impl TryFrom<&[u8]> for $name {
            type Error = $crate::Error;

            fn try_from(val: &[u8]) -> $crate::Result<Self> {
                let mut msg = Self::new();

                msg.from_buf(val)?;

                Ok(msg)
            }
        }

        impl<const N: usize> TryFrom<[u8; N]> for $name {
            type Error = $crate::Error;

            fn try_from(val: [u8; N]) -> $crate::Result<Self> {
                Self::try_from(val.as_ref())
            }
        }
    };
}

/// Creates a list container type for a named type.
///
/// Useful for formatting, and other trait implementations over user-defined types.
#[macro_export]
macro_rules! make_list {
    ($list_name:ident, $name:ident, $doc:tt) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq)]
        pub struct $list_name($crate::Vec<$name>);

        impl $list_name {
            /// Creates a new empty list.
            pub fn new() -> Self {
                Self($crate::Vec::new())
            }

            /// Gets an iterator over the list.
            pub fn iter(&self) -> $crate::std::slice::Iter<$name> {
                self.0.iter()
            }

            /// Gets a mutable iterator over the list.
            pub fn iter_mut(&mut self) -> $crate::std::slice::IterMut<$name> {
                self.0.iter_mut()
            }

            /// Gets the list length.
            pub fn len(&self) -> usize {
                self.0.len()
            }

            /// Gets the list capacity.
            pub fn capacity(&self) -> usize {
                self.0.capacity()
            }

            /// Gets whether the list is empty.
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            /// Get the list as a reference to its inner container type.
            pub fn as_inner(&self) -> &$crate::Vec<$name> {
                &self.0
            }

            /// Get the list as a mutable reference to its inner container type.
            pub fn as_inner_mut(&mut self) -> &mut $crate::Vec<$name> {
                &mut self.0
            }

            /// Converts the list into its inner container type.
            pub fn into_inner(self) -> $crate::Vec<$name> {
                self.0
            }
        }

        impl AsRef<[$name]> for $list_name {
            fn as_ref(&self) -> &[$name] {
                self.0.as_slice()
            }
        }

        impl AsMut<[$name]> for $list_name {
            fn as_mut(&mut self) -> &mut [$name] {
                self.0.as_mut()
            }
        }

        impl From<$crate::Vec<$name>> for $list_name {
            fn from(val: $crate::Vec<$name>) -> Self {
                Self(val)
            }
        }

        impl From<&[$name]> for $list_name {
            fn from(val: &[$name]) -> Self {
                let max = $crate::std::cmp::min(val.len(), $crate::len::MAX_DATA);
                let mut ret = Self::new();
                let list = ret.as_inner_mut();

                for &v in val[..max].iter() {
                    // range is guaranteed valid by the min call above
                    // so, this unwrap call should never panic.
                    list.push(v).unwrap();
                }

                ret
            }
        }

        impl<const N: usize> From<[$name; N]> for $list_name {
            fn from(val: [$name; N]) -> Self {
                val.as_ref().into()
            }
        }

        impl<const N: usize> From<&[$name; N]> for $list_name {
            fn from(val: &[$name; N]) -> Self {
                val.as_ref().into()
            }
        }

        impl $crate::std::fmt::Display for $list_name {
            fn fmt(&self, f: &mut $crate::std::fmt::Formatter<'_>) -> $crate::std::fmt::Result {
                write!(f, "[")?;

                for (i, code) in self.iter().enumerate() {
                    write!(f, "{code}")?;

                    if i < self.len() - 1 {
                        write!(f, ", ")?;
                    }
                }

                write!(f, "]")
            }
        }

        impl Default for $list_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
