//! # JSON serialize for hex encoded byte arrays (without '0x' prefix)

/// Macro to generate hex serialazable byte arrays
macro_rules! byte_array_struct {
    ($name: ident, $num: expr) => (
        #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name([u8; $num]);

        impl ::std::ops::Deref for $name {
            type Target = [u8];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$name> for [u8; $num] {
            fn from(arr: $name) -> Self {
                arr.0
            }
        }

        impl From<[u8; $num]> for $name {
            fn from(bytes: [u8; $num]) -> Self {
                $name(bytes)
            }
        }

        impl ::rustc_serialize::Decodable for $name {
            fn decode<D: ::rustc_serialize::Decoder>(d: &mut D) -> Result<$name, D::Error> {
                let v =
                    (d.read_str().and_then(|s| s.from_hex().map_err(|e| d.error(&e.to_string()))))?;

                if v.len() != $num {
                    return Err(d.error(&format!("Byte array invalid length: {}", v.len())));
                }

                let mut bytes = [0u8; $num];

                bytes.copy_from_slice(&v);

                Ok($name(bytes))
            }
        }

        impl ::rustc_serialize::Encodable for $name {
            fn encode<S: ::rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
                s.emit_str(&self.0.to_hex())
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use rustc_serialize::hex::{FromHex, ToHex};
    use rustc_serialize::json;

    byte_array_struct!(Hex8, 8);

    #[test]
    fn should_encode_default_byte_array() {
        assert_eq!(json::encode(&Hex8::default()).unwrap(),
                   "\"0000000000000000\"");
    }

    #[test]
    fn should_decode_zero_byte_array() {
        assert_eq!(json::decode::<Hex8>("\"0000000000000000\"").unwrap(),
                   Hex8::default());
    }

    #[test]
    fn should_encode_byte_array() {
        let hex = Hex8::from([0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);

        assert_eq!(json::encode(&hex).unwrap(), "\"0123456789abcdef\"");
    }

    #[test]
    fn should_decode_byte_array() {
        let hex = Hex8::from([0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);

        assert_eq!(json::decode::<Hex8>("\"0123456789abcdef\"").unwrap(), hex);
    }

    #[test]
    fn should_not_decode_invalid_byte_array() {
        assert!(json::decode::<Hex8>("\"__23456789abcdef\"").is_err());
    }

    #[test]
    fn should_not_decode_insufficient_byte_array() {
        assert!(json::decode::<Hex8>("1234567890").is_err());
    }

    #[test]
    fn should_not_decode_empty_text() {
        assert!(json::decode::<Hex8>("\"\"").is_err());
    }

    #[test]
    fn should_not_decode_absent_text() {
        assert!(json::decode::<Hex8>("").is_err());
    }
}
