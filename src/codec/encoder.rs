use std::io::Error;

use bytes::{BufMut, BytesMut};
use easy_modbus::{util::crc, Version::Rtu};
use tokio_util::codec::Encoder;

use crate::Request;

impl Encoder<()> for Request {
    type Error = Error;

    fn encode(
        &mut self,
        _item: (),
        dst: &mut BytesMut
    ) -> std::result::Result<(), Self::Error> {
        request_to_bytesmut(&self, dst);
        Ok(())
    }
}

pub fn request_to_bytesmut(item: &Request, dst: &mut BytesMut) {
    let version;
    match item {
        Request::ReadCoils(head, body) => {
            version = head.version.clone();
            dst.put(BytesMut::from(head.clone()));
            dst.put(BytesMut::from(body.clone()));
        },
        Request::ReadDiscreteInputs(head, body) => {
            version = head.version.clone();
            dst.put(BytesMut::from(head.clone()));
            dst.put(BytesMut::from(body.clone()));
        },
        Request::ReadMultipleHoldingRegisters(head, body) => {
            version = head.version.clone();
            dst.put(BytesMut::from(head.clone()));
            dst.put(BytesMut::from(body.clone()));
        },
        Request::ReadInputRegisters(head, body) => {
            version = head.version.clone();
            dst.put(BytesMut::from(head.clone()));
            dst.put(BytesMut::from(body.clone()));
        },
        Request::WriteSingleCoil(head, body) => {
            version = head.version.clone();
            dst.put(BytesMut::from(head.clone()));
            dst.put(BytesMut::from(body.clone()));
        },
        Request::WriteSingleHoldingRegister(head, body) => {
            version = head.version.clone();
            dst.put(BytesMut::from(head.clone()));
            dst.put(BytesMut::from(body.clone()));
        },
        Request::WriteMultipleCoils(head, body) => {
            version = head.version.clone();
            dst.put(BytesMut::from(head.clone()));
            dst.put(BytesMut::from(body.clone()));
        },
        Request::WriteMultipleHoldingRegisters(head, body) => {
            version = head.version.clone();
            dst.put(BytesMut::from(head.clone()));
            dst.put(BytesMut::from(body.clone()));
        }
    };
    if Rtu == version {
        dst.put_u16(crc::compute(&dst.to_vec()));
    }
}
