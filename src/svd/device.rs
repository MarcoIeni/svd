use crate::elementext::ElementExt;
#[cfg(feature = "unproven")]
use std::collections::HashMap;
use xmltree::Element;

use crate::parse;
use crate::types::Parse;

#[cfg(feature = "unproven")]
use crate::encode::{Encode, EncodeChildren};
use crate::error::SVDError;
#[cfg(feature = "unproven")]
use crate::new_element;
use crate::svd::{
    cpu::Cpu,
    registerproperties::RegisterProperties,
    peripheral::Peripheral,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct Device {
    pub name: String,
    schema_version: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub address_unit_bits: Option<u32>,
    pub width: Option<u32>,
    pub cpu: Option<Cpu>,
    pub peripherals: Vec<Peripheral>,
    pub default_register_properties: RegisterProperties,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Parse for Device {
    type Object = Device;
    type Error = SVDError;

    /// Parses a SVD file
    fn parse(tree: &Element) -> Result<Device, SVDError> {
        Ok(Device {
            name: tree.get_child_text("name")?,
            schema_version: tree.attributes
                .get("schemaVersion")
                .map(|s| s.clone()),
            cpu: parse::optional::<Cpu>("cpu", tree)?,
            version: tree.get_child_text_opt("version")?,
            description: tree.get_child_text_opt("description")?,
            address_unit_bits: parse::optional::<u32>("addressUnitBits", tree)?,
            width: None,
            peripherals: {
                let ps: Result<Vec<_>, _> = tree.get_child_elem("peripherals")?
                    .children
                    .iter()
                    .map(Peripheral::parse)
                    .collect();
                ps?
            },
            default_register_properties: RegisterProperties::parse(tree)?,
            _extensible: (),
        })
    }
}

#[cfg(feature = "unproven")]
impl Encode for Device {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        let mut elem = Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("device"),
            attributes: HashMap::new(),
            children: vec![new_element("name", Some(self.name.clone()))],
            text: None,
        };

        elem.attributes.insert(
            String::from("xmlns:xs"),
            String::from("http://www.w3.org/2001/XMLSchema-instance"),
        );
        match self.schema_version {
            Some(ref schema_version) => { 
                elem.attributes.insert(
                    String::from("schemaVersion"),
                    format!("{}", schema_version));
                },
            None => (),
        }
        match self.schema_version {
            Some(ref schema_version) => { 
                elem.attributes.insert(
                    String::from("xs:noNamespaceSchemaLocation"),
                    format!("CMSIS-SVD_Schema_{}.xsd", schema_version));
                },
            None => (),
        }


        match self.version {
            Some(ref v) => elem.children
                .push(new_element("version", Some(v.clone()))),
            None => (),
        }

        match self.description {
            Some(ref v) => elem.children
                .push(new_element("description", Some(v.clone()))),
            None => (),
        }

        match self.description {
            Some(ref v) => elem.children.push(new_element(
                "addressUnitBits",
                Some(format!("{}", v)),
            )),
            None => (),
        }

        match self.width {
            Some(ref v) => elem.children
                .push(new_element("width", Some(format!("{}", v)))),
            None => (),
        }

        elem.children.extend(self.default_register_properties.encode()?);

        match self.cpu {
            Some(ref v) => {
                elem.children.push(v.encode()?);
            }
            None => (),
        }

        let peripherals: Result<Vec<_>, _> = self.peripherals
            .iter()
            .map(Peripheral::encode)
            .collect();
        elem.children.push(Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("peripherals"),
            attributes: HashMap::new(),
            children: peripherals?,
            text: None,
        });

        Ok(elem)
    }
}

// TODO: test device encoding and decoding