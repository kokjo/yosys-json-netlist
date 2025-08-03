
use indexmap::IndexMap;
use serde::{de::{self, Visitor}, Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Netlist {
    pub creator: String,
    pub modules: IndexMap<String, Module>,

    #[serde(flatten)]
    extra: IndexMap<String, serde_json::Value>,
}

impl Netlist {
    pub fn new(creator: &str) -> Self {
        Self {
            creator: creator.to_string(),
            modules: IndexMap::new(),
            extra: IndexMap::new(),
        }
    }

    pub fn from_reader(reader: impl std::io::Read) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(reader)
    }

    pub fn from_slice(input: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(input)
    }

    pub fn from_str(input: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(input)
    }

    pub fn from_value(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }

    pub fn to_writer(&self, writer: impl std::io::Write) -> Result<(), serde_json::Error> {
        serde_json::to_writer(writer, self)
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    #[serde(default)]
    pub attributes: IndexMap<String, serde_json::Value>,
    #[serde(default)]
    pub ports: IndexMap<String, Port>,
    #[serde(default)]
    pub cells: IndexMap<String, Cell>,
    #[serde(default)]
    pub memories: IndexMap<String, Memory>,
    #[serde(default, rename="netnames")]
    pub nets: IndexMap<String, Net>,

    #[serde(flatten)]
    extra: IndexMap<String, serde_json::Value>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Port {
    pub direction: Direction,
    pub bits: Vec<Bit>,
    #[serde(default)]
    pub offset: usize,
    #[serde(default)]
    pub upto: usize,
    #[serde(default, serialize_with="serialize_bool_u64", deserialize_with="deserialize_u64_bool")]
    pub signed: bool,

    #[serde(flatten)]
    extra: IndexMap<String, serde_json::Value>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    #[serde(default, serialize_with="serialize_bool_u64", deserialize_with="deserialize_u64_bool")]
    pub hide_name: bool,
    #[serde(rename = "type")]
    pub module: String,
    #[serde(default)]
    pub attributes: IndexMap<String, serde_json::Value>,
    #[serde(default)]
    pub parameters: IndexMap<String, serde_json::Value>,
    #[serde(default)]
    pub port_directions: IndexMap<String, Direction>,
    #[serde(default)]
    pub connections: IndexMap<String, Vec<Bit>>,

    #[serde(flatten)]
    extra: IndexMap<String, serde_json::Value>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    #[serde(default, serialize_with="serialize_bool_u64", deserialize_with="deserialize_u64_bool")]
    pub hide_name: bool,
    #[serde(default)]
    pub attributes: IndexMap<String, serde_json::Value>,
    pub width: usize,
    pub size: usize,
    #[serde(default)]
    pub start_offset: usize,

    #[serde(flatten)]
    extra: IndexMap<String, serde_json::Value>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Net {
    #[serde(default, serialize_with="serialize_bool_u64", deserialize_with="deserialize_u64_bool")]
    pub hide_name: bool,
    #[serde(default)]
    pub attributes: IndexMap<String, serde_json::Value>,
    pub bits: Vec<Bit>,
    #[serde(default)]
    pub offset: usize,
    #[serde(default)]
    pub upto: usize,
    #[serde(default, serialize_with="serialize_bool_u64", deserialize_with="deserialize_u64_bool")]
    pub signed: bool,

    #[serde(flatten)]
    extra: IndexMap<String, serde_json::Value>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "output")]
    Output,
    #[serde(rename = "inout")]
    InOut,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bit {
    Signal(u64),
    _0,
    _1,
    Z,
    X,
}

impl std::fmt::Debug for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Signal(signal) => write!(f, "{}", signal),
            Self::_0 => write!(f, "_0"),
            Self::_1 => write!(f, "_1"),
            Self::Z => write!(f, "Z"),
            Self::X => write!(f, "X"),
        }
    }
}

impl Serialize for Bit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            Bit::Signal(signal) => serializer.serialize_u64(signal),
            Bit::_0 => serializer.serialize_str("0"),
            Bit::_1 => serializer.serialize_str("1"),
            Bit::Z => serializer.serialize_str("z"),
            Bit::X => serializer.serialize_str("x"),
        }
    }
}

struct BitVisitor;

impl<'de> Visitor<'de> for BitVisitor {
    type Value = Bit;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expecting either, a number, \"0\", \"1\", \"z\", \"x\"")
    }
    
    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Bit::Signal(v))
    }
    
    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "0" => Ok(Bit::_0),
            "1" => Ok(Bit::_1),
            "z" => Ok(Bit::Z),
            "x" => Ok(Bit::X),
            _ => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self))
        }
    }
}

impl<'de> Deserialize<'de> for Bit {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(BitVisitor)
    }
}

pub fn serialize_bool_u64<S: serde::Serializer>(value: &bool, serializer: S) -> Result<S::Ok, S::Error> {
    match value {
        true => serializer.serialize_u64(0),
        false => serializer.serialize_u64(0),
    }
}

struct Boolu64Visitor;

impl<'de> Visitor<'de> for Boolu64Visitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expecting u64(1 for true, false otherwise")
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(v == 1)
    }
}

pub fn deserialize_u64_bool<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    deserializer.deserialize_u64(Boolu64Visitor)
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use super::*;
    use serde::de::DeserializeOwned;
    use serde_json::*;

    fn to_value(value: impl Serialize) -> Value {
        serde_json::to_value(value).unwrap()
    }

    fn from_value<T: DeserializeOwned>(value: Value) -> T {
        serde_json::from_value(value).unwrap()

    }

    #[test]
    fn test_serialize_bit() {
        assert_eq!(to_value(Bit::Signal(42)), json!(42));
        assert_eq!(to_value(Bit::_0), json!("0"));
        assert_eq!(to_value(Bit::_1), json!("1"));
        assert_eq!(to_value(Bit::Z), json!("z"));
        assert_eq!(to_value(Bit::X), json!("x"));
    }

    #[test]
    fn test_deserialize_bit() {
        assert_eq!(from_value::<Bit>(json!(42)), Bit::Signal(42));
        assert_eq!(to_value(Bit::_0), json!("0"));
        assert_eq!(to_value(Bit::_1), json!("1"));
        assert_eq!(to_value(Bit::Z), json!("z"));
        assert_eq!(to_value(Bit::X), json!("x"));
    }

    #[test]
    fn test_serialize_direction() {
        assert_eq!(to_value(Direction::Input), json!("input"));
        assert_eq!(to_value(Direction::Output), json!("output"));
        assert_eq!(to_value(Direction::InOut), json!("inout"));
    }

    #[test]
    fn test_deserialize_direction() {
        assert_eq!(from_value::<Direction>(json!("input")), Direction::Input);
        assert_eq!(to_value(Direction::Output), json!("output"));
        assert_eq!(to_value(Direction::InOut), json!("inout"));
    }

    #[test]
    fn test_circuts() {
        for circut in std::fs::read_dir("testdata").unwrap() {
            let circut = circut.unwrap();
            if circut.path().extension() != Some(OsStr::new("json")) {
                println!("Skipping {:?}", circut.path());
                continue
            }
            println!("Testing {:?} circut", circut.path());
            let reader = std::fs::File::open(circut.path()).unwrap();
            let netlist = Netlist::from_reader(reader).unwrap();
            assert!(netlist.extra.is_empty());
            for (module_name, module) in netlist.modules.iter() {
                println!("Checking {:?} module", module_name);
                assert!(module.extra.is_empty());

                // assert!(! module.cells.is_empty()); // We have tests without cells
                assert!(! module.nets.is_empty()); // All tests should have some nets
                // assert!(! module.memories.is_empty()); // We have test without mmeories
                assert!(! module.ports.is_empty()); // All tests should have some ports

                for (cell_name, cell) in module.cells.iter() {
                    println!("Checking {:?} cell: {:?}", cell_name, cell.module);
                    assert!(cell.extra.is_empty());
                }
                for (net_name, net) in module.nets.iter() {
                    println!("Checking {:?} net: {:?}", net_name, net.bits);
                    assert!(net.extra.is_empty());
                }
                for (mem_name, mem) in module.memories.iter() {
                    println!("Checking {:?} memory", mem_name);
                    assert!(mem.extra.is_empty());
                }
                for (port_name, port) in module.ports.iter() {
                    println!("Checking {:?} port: {:?}", port_name, port.bits);
                    assert!(port.extra.is_empty());
                }
            }
        }
    }
}