// Automatically generated rust module for 'esp_local_ctrl.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use core::convert::{TryFrom, TryInto};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LocalCtrlMsgType {
    TypeCmdGetPropertyCount = 0,
    TypeRespGetPropertyCount = 1,
    TypeCmdGetPropertyValues = 4,
    TypeRespGetPropertyValues = 5,
    TypeCmdSetPropertyValues = 6,
    TypeRespSetPropertyValues = 7,
}

impl Default for LocalCtrlMsgType {
    fn default() -> Self {
        LocalCtrlMsgType::TypeCmdGetPropertyCount
    }
}

impl From<i32> for LocalCtrlMsgType {
    fn from(i: i32) -> Self {
        match i {
            0 => LocalCtrlMsgType::TypeCmdGetPropertyCount,
            1 => LocalCtrlMsgType::TypeRespGetPropertyCount,
            4 => LocalCtrlMsgType::TypeCmdGetPropertyValues,
            5 => LocalCtrlMsgType::TypeRespGetPropertyValues,
            6 => LocalCtrlMsgType::TypeCmdSetPropertyValues,
            7 => LocalCtrlMsgType::TypeRespSetPropertyValues,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for LocalCtrlMsgType {
    fn from(s: &'a str) -> Self {
        match s {
            "TypeCmdGetPropertyCount" => LocalCtrlMsgType::TypeCmdGetPropertyCount,
            "TypeRespGetPropertyCount" => LocalCtrlMsgType::TypeRespGetPropertyCount,
            "TypeCmdGetPropertyValues" => LocalCtrlMsgType::TypeCmdGetPropertyValues,
            "TypeRespGetPropertyValues" => LocalCtrlMsgType::TypeRespGetPropertyValues,
            "TypeCmdSetPropertyValues" => LocalCtrlMsgType::TypeCmdSetPropertyValues,
            "TypeRespSetPropertyValues" => LocalCtrlMsgType::TypeRespSetPropertyValues,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CmdGetPropertyCount { }

impl<'a> MessageRead<'a> for CmdGetPropertyCount {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for CmdGetPropertyCount { }


            impl TryFrom<&[u8]> for CmdGetPropertyCount {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(CmdGetPropertyCount::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RespGetPropertyCount {
    pub status: constants::Status,
    pub count: u32,
}

impl<'a> MessageRead<'a> for RespGetPropertyCount {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status = r.read_enum(bytes)?,
                Ok(16) => msg.count = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RespGetPropertyCount {
    fn get_size(&self) -> usize {
        0
        + if self.status == constants::Status::Success { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.count) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status != constants::Status::Success { w.write_with_tag(8, |w| w.write_enum(*&self.status as i32))?; }
        if self.count != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.count))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for RespGetPropertyCount {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(RespGetPropertyCount::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PropertyInfo {
    pub status: constants::Status,
    pub name: String,
    pub type_pb: u32,
    pub flags: u32,
    pub value: Vec<u8>,
}

impl<'a> MessageRead<'a> for PropertyInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status = r.read_enum(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.type_pb = r.read_uint32(bytes)?,
                Ok(32) => msg.flags = r.read_uint32(bytes)?,
                Ok(42) => msg.value = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PropertyInfo {
    fn get_size(&self) -> usize {
        0
        + if self.status == constants::Status::Success { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.type_pb == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.flags == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.flags) as u64) }
        + if self.value.is_empty() { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status != constants::Status::Success { w.write_with_tag(8, |w| w.write_enum(*&self.status as i32))?; }
        if self.name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.type_pb != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.type_pb))?; }
        if self.flags != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.flags))?; }
        if !self.value.is_empty() { w.write_with_tag(42, |w| w.write_bytes(&**&self.value))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for PropertyInfo {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(PropertyInfo::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CmdGetPropertyValues {
    pub indices: Vec<u32>,
}

impl<'a> MessageRead<'a> for CmdGetPropertyValues {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.indices = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CmdGetPropertyValues {
    fn get_size(&self) -> usize {
        0
        + if self.indices.is_empty() { 0 } else { 1 + sizeof_len(self.indices.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.indices, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for CmdGetPropertyValues {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(CmdGetPropertyValues::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RespGetPropertyValues {
    pub status: constants::Status,
    pub props: Vec<PropertyInfo>,
}

impl<'a> MessageRead<'a> for RespGetPropertyValues {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status = r.read_enum(bytes)?,
                Ok(18) => msg.props.push(r.read_message::<PropertyInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RespGetPropertyValues {
    fn get_size(&self) -> usize {
        0
        + if self.status == constants::Status::Success { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + self.props.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status != constants::Status::Success { w.write_with_tag(8, |w| w.write_enum(*&self.status as i32))?; }
        for s in &self.props { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for RespGetPropertyValues {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(RespGetPropertyValues::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PropertyValue {
    pub index: u32,
    pub value: Vec<u8>,
}

impl<'a> MessageRead<'a> for PropertyValue {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.index = r.read_uint32(bytes)?,
                Ok(18) => msg.value = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PropertyValue {
    fn get_size(&self) -> usize {
        0
        + if self.index == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.index) as u64) }
        + if self.value.is_empty() { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.index != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.index))?; }
        if !self.value.is_empty() { w.write_with_tag(18, |w| w.write_bytes(&**&self.value))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for PropertyValue {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(PropertyValue::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CmdSetPropertyValues {
    pub props: Vec<PropertyValue>,
}

impl<'a> MessageRead<'a> for CmdSetPropertyValues {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.props.push(r.read_message::<PropertyValue>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CmdSetPropertyValues {
    fn get_size(&self) -> usize {
        0
        + self.props.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.props { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for CmdSetPropertyValues {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(CmdSetPropertyValues::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RespSetPropertyValues {
    pub status: constants::Status,
}

impl<'a> MessageRead<'a> for RespSetPropertyValues {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RespSetPropertyValues {
    fn get_size(&self) -> usize {
        0
        + if self.status == constants::Status::Success { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status != constants::Status::Success { w.write_with_tag(8, |w| w.write_enum(*&self.status as i32))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for RespSetPropertyValues {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(RespSetPropertyValues::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LocalCtrlMessage {
    pub msg: LocalCtrlMsgType,
    pub payload: mod_LocalCtrlMessage::OneOfpayload,
}

impl<'a> MessageRead<'a> for LocalCtrlMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msg = r.read_enum(bytes)?,
                Ok(82) => msg.payload = mod_LocalCtrlMessage::OneOfpayload::cmd_get_prop_count(r.read_message::<CmdGetPropertyCount>(bytes)?),
                Ok(90) => msg.payload = mod_LocalCtrlMessage::OneOfpayload::resp_get_prop_count(r.read_message::<RespGetPropertyCount>(bytes)?),
                Ok(98) => msg.payload = mod_LocalCtrlMessage::OneOfpayload::cmd_get_prop_vals(r.read_message::<CmdGetPropertyValues>(bytes)?),
                Ok(106) => msg.payload = mod_LocalCtrlMessage::OneOfpayload::resp_get_prop_vals(r.read_message::<RespGetPropertyValues>(bytes)?),
                Ok(114) => msg.payload = mod_LocalCtrlMessage::OneOfpayload::cmd_set_prop_vals(r.read_message::<CmdSetPropertyValues>(bytes)?),
                Ok(122) => msg.payload = mod_LocalCtrlMessage::OneOfpayload::resp_set_prop_vals(r.read_message::<RespSetPropertyValues>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LocalCtrlMessage {
    fn get_size(&self) -> usize {
        0
        + if self.msg == LocalCtrlMsgType::TypeCmdGetPropertyCount { 0 } else { 1 + sizeof_varint(*(&self.msg) as u64) }
        + match self.payload {
            mod_LocalCtrlMessage::OneOfpayload::cmd_get_prop_count(ref m) => 1 + sizeof_len((m).get_size()),
            mod_LocalCtrlMessage::OneOfpayload::resp_get_prop_count(ref m) => 1 + sizeof_len((m).get_size()),
            mod_LocalCtrlMessage::OneOfpayload::cmd_get_prop_vals(ref m) => 1 + sizeof_len((m).get_size()),
            mod_LocalCtrlMessage::OneOfpayload::resp_get_prop_vals(ref m) => 1 + sizeof_len((m).get_size()),
            mod_LocalCtrlMessage::OneOfpayload::cmd_set_prop_vals(ref m) => 1 + sizeof_len((m).get_size()),
            mod_LocalCtrlMessage::OneOfpayload::resp_set_prop_vals(ref m) => 1 + sizeof_len((m).get_size()),
            mod_LocalCtrlMessage::OneOfpayload::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msg != LocalCtrlMsgType::TypeCmdGetPropertyCount { w.write_with_tag(8, |w| w.write_enum(*&self.msg as i32))?; }
        match self.payload {            mod_LocalCtrlMessage::OneOfpayload::cmd_get_prop_count(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            mod_LocalCtrlMessage::OneOfpayload::resp_get_prop_count(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            mod_LocalCtrlMessage::OneOfpayload::cmd_get_prop_vals(ref m) => { w.write_with_tag(98, |w| w.write_message(m))? },
            mod_LocalCtrlMessage::OneOfpayload::resp_get_prop_vals(ref m) => { w.write_with_tag(106, |w| w.write_message(m))? },
            mod_LocalCtrlMessage::OneOfpayload::cmd_set_prop_vals(ref m) => { w.write_with_tag(114, |w| w.write_message(m))? },
            mod_LocalCtrlMessage::OneOfpayload::resp_set_prop_vals(ref m) => { w.write_with_tag(122, |w| w.write_message(m))? },
            mod_LocalCtrlMessage::OneOfpayload::None => {},
    }        Ok(())
    }
}


            impl TryFrom<&[u8]> for LocalCtrlMessage {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(LocalCtrlMessage::from_reader(&mut reader, &buf)?)
                }
            }
            
pub mod mod_LocalCtrlMessage {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfpayload {
    cmd_get_prop_count(CmdGetPropertyCount),
    resp_get_prop_count(RespGetPropertyCount),
    cmd_get_prop_vals(CmdGetPropertyValues),
    resp_get_prop_vals(RespGetPropertyValues),
    cmd_set_prop_vals(CmdSetPropertyValues),
    resp_set_prop_vals(RespSetPropertyValues),
    None,
}

impl Default for OneOfpayload {
    fn default() -> Self {
        OneOfpayload::None
    }
}

}

