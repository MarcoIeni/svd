//! Shared primitive types for use in SVD objects.

use failure::ResultExt;
use xmltree::Element;

#[cfg(feature = "unproven")]
pub use crate::encode::Encode;
pub use crate::parse::optional as parse_optional;
pub use crate::parse::Parse;

use crate::elementext::ElementExt;
use crate::error::{SVDError, SVDErrorKind};

macro_rules! unwrap {
    ($e:expr) => {
        $e.expect(concat!(
            file!(),
            ":",
            line!(),
            " ",
            stringify!($e)
        ))
    };
}

impl Parse for u32 {
    type Object = u32;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<u32, SVDError> {
        let text = tree.get_text()?;

        if text.starts_with("0x") || text.starts_with("0X") {
            u32::from_str_radix(&text["0x".len()..], 16)
                .context(SVDErrorKind::Other(format!("{} invalid", text)))
                .map_err(|e| e.into())
        } else if text.starts_with('#') {
            // Handle strings in the binary form of:
            // #01101x1
            // along with don't care character x (replaced with 0)
            u32::from_str_radix(
                &str::replace(&text.to_lowercase()["#".len()..], "x", "0"),
                2,
            ).context(SVDErrorKind::Other(format!("{} invalid", text)))
                .map_err(|e| e.into())
        } else if text.starts_with("0b") {
            // Handle strings in the binary form of:
            // 0b01101x1
            // along with don't care character x (replaced with 0)
            u32::from_str_radix(&str::replace(&text["0b".len()..], "x", "0"), 2)
                .context(SVDErrorKind::Other(format!("{} invalid", text)))
                .map_err(|e| e.into())
        } else {
            text.parse::<u32>()
                .context(SVDErrorKind::Other(format!("{} invalid", text)))
                .map_err(|e| e.into())
        }
    }
}

pub struct BoolParse;

impl Parse for BoolParse {
    type Object = bool;
    type Error = SVDError;
    fn parse(tree: &Element) -> Result<bool, SVDError> {
        let text = unwrap!(tree.text.as_ref());
        Ok(match text.as_ref() {
            "0" => false,
            "1" => true,
            _ => match text.parse() {
                Ok(b) => b,
                Err(e) => {
                    return Err(SVDErrorKind::InvalidBooleanValue(
                        tree.clone(),
                        text.clone(),
                        e,
                    ).into())
                }
            },
        })
    }
}

pub struct DimIndex;

impl Parse for DimIndex {
    type Object = Vec<String>;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<Vec<String>, SVDError> {
        let text = tree.get_text()?;
        if text.contains('-') {
            let mut parts = text.splitn(2, '-');
            let start = unwrap!(unwrap!(parts.next()).parse::<u32>());
            let end = unwrap!(unwrap!(parts.next()).parse::<u32>()) + 1;

            Ok((start..end).map(|i| i.to_string()).collect())
        } else if text.contains(',') {
            Ok(text.split(',').map(|s| s.to_string()).collect())
        } else {
            unreachable!()
        }
    }
}

//TODO: encode for DimIndex