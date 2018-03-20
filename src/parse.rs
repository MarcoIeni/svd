use xmltree::Element;
use failure::ResultExt;

use error::*;

// TODO: Should work on &str not Element
// TODO: `parse::u32` should not hide it's errors, see `BitRange::parse`
pub fn u32(tree: &Element) -> Result<u32, SVDError> {
    let text = get_text(tree)?;

    if text.starts_with("0x") || text.starts_with("0X") {
        u32::from_str_radix(&text["0x".len()..], 16).context(SVDErrorKind::Other(format!("{} invalid", text))).map_err(|e| e.into())
    } else if text.starts_with('#') {
        // Handle strings in the binary form of:
        // #01101x1
        // along with don't care character x (replaced with 0)
        u32::from_str_radix(&str::replace(&text.to_lowercase()["#".len()..], "x", "0"), 2).context(SVDErrorKind::Other(format!("{} invalid", text))).map_err(|e| e.into())
    } else if text.starts_with("0b"){
        // Handle strings in the binary form of:
        // 0b01101x1
        // along with don't care character x (replaced with 0)
        u32::from_str_radix(&str::replace(&text["0b".len()..], "x", "0"), 2).context(SVDErrorKind::Other(format!("{} invalid", text))).map_err(|e| e.into())

    } else {
        text.parse::<u32>().context(SVDErrorKind::Other(format!("{} invalid", text))).map_err(|e| e.into())
    }
}

pub fn bool(tree: &Element) -> Option<bool> {
    let text = try!(tree.text.as_ref());
    match text.as_ref() {
        "0" => Some(false),
        "1" => Some(true),
        _ => text.parse::<bool>().ok()
    }
}

pub fn dim_index(text: &str) -> Vec<String> {
    if text.contains('-') {
        let mut parts = text.splitn(2, '-');
        let start = try!(try!(parts.next()).parse::<u32>());
        let end = try!(try!(parts.next()).parse::<u32>()) + 1;

        (start..end).map(|i| i.to_string()).collect()
    } else if text.contains(',') {
        text.split(',').map(|s| s.to_string()).collect()
    } else {
        unreachable!()
    }
}

/// Parses an optional child element with the provided name and Parse function
/// Returns an none if the child doesn't exist, Ok(Some(e)) if parsing succeeds,
/// and Err() if parsing fails.
/// TODO: suspect we should be able to use the Parse trait here
pub fn optional<'a, T, CB>(n: &str, e: &'a Element, f: CB) -> Result<Option<T>, SVDError>
    where CB: 'static + Fn(&Element) -> Result<T, SVDError>
{
     let child = match e.get_child(n) {
        Some(c) => c,
        None => return Ok(None),
    };

    match f(child) {
        Ok(r) => Ok(Some(r)),
        Err(e) => Err(e),
    }
}


/// Get text contained by an XML Element
pub fn get_text<'a>(e: &'a Element) -> Result<&'a str, SVDError> {
    match e.text.as_ref() {
        Some(s) => Ok(s),
        // FIXME: Doesn't look good because SVDErrorKind doesn't format by itself. We already
        // capture the element and this information can be used for getting the name
        // This would fix ParseError
        None => Err(SVDErrorKind::EmptyTag(e.clone(), e.name.clone()).into()),
    }
}

/// Get a named child element from an XML Element
pub fn get_child_elem<'a>(n: &str, e: &'a Element) -> Result<&'a Element, SVDError> {
    match e.get_child(n) {
        Some(s) => Ok(s),
        None => Err(SVDErrorKind::MissingTag(e.clone(), e.name.clone()).into()),
    }
}

/// Get a u32 value from a named child element
pub fn get_child_u32(n: &str, e: &Element) -> Result<u32, SVDError> {
    let s = get_child_elem(n, e)?;
    u32(&s).context(SVDErrorKind::ParseError(e.clone())).map_err(|e| e.into())
}

/// Get a bool value from a named child element
pub fn get_child_bool(n: &str, e: &Element) -> Result<bool, SVDError> {
    let s = get_child_elem(n, e)?;
    match bool(s) {
        Some(u) => Ok(u),
        None => Err(SVDErrorKind::ParseError(e.clone()).into())
    }
}
