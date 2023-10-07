use std::io::{Error, ErrorKind::InvalidData, Result};

use crate::Request;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use easy_modbus::codec::get_function;
use easy_modbus::util::crc;
use easy_modbus::*;
use tokio_util::codec::Decoder;

impl Decoder for Request {
    type Item = ();
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<()>> {
        if src.len() < 2 {
            return Ok(None);
        }

        let mut data_bytes = BytesMut::new();
        let (function, is_exception) = get_function(src.get(1).map_or(0, |x| *x))?;

        let len: usize = {
            if is_exception {
                1
            } else {
                if function != self.head().function {
                    return Err(Error::new(
                        InvalidData,
                        format!(
                            "Invalid function: {:0>2X} {:0>2X}",
                            function.to_code(),
                            self.head().function.to_code()
                        ),
                    ));
                }
                match function {
                    Function::ReadCoils
                    | Function::ReadDiscreteInputs
                    | Function::ReadMultipleHoldingRegisters
                    | Function::ReadInputRegisters => {
                        src.get(0).map_or(0, |&bytes_num| bytes_num as usize + 1)
                    }
                    Function::WriteSingleCoil
                    | Function::WriteSingleHoldingRegister
                    | Function::WriteMultipleCoils
                    | Function::WriteMultipleHoldingRegisters => 4,
                }
            }
        };

        if src.len() < len + 2 {
            return Ok(None);
        }
        // let uid = src.get(0).map_or(0, |x| *x);
        // let mut head = Head::init(uid, function, is_exception, self.head().version.clone());
        // head.body_length(len as u16);

        let body_bytes = src.copy_to_bytes(len);
        data_bytes.put_slice(&(body_bytes.to_vec()));
        let crc = src.get_u16();
        if crc::check(&(data_bytes.to_vec()), crc) {
            get_response(body_bytes, self, is_exception);
            return Ok(Some(()));
        }
        return Err(Error::new(
            InvalidData,
            format!("Invalid crc code: 0x{:0>2X}", crc),
        ));
    }
}
//
// impl Decoder for TcpCodec {
//     type Item = Response;
//     type Error = Error;
//
//     fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Response>> {
//         if src.len() < 4 {
//             return Ok(None);
//         }
//         let head = Head::tcp_try_from(src.copy_to_bytes(8))?;
//         let len = head.length as usize - 2;
//         let response = get_response(src.copy_to_bytes(len), head);
//         Ok(Some(response))
//     }
// }

fn get_response(src: Bytes, request: &mut Request, is_exception: bool) {
    match request {
        Request::ReadCoils(_, _, response) => {
            if is_exception {
                response.replace(Err(ExceptionResponse::from(src)));
            } else {
                response.replace(Ok(ReadCoilsResponse::from(src)));
            }
        }
        Request::ReadDiscreteInputs(_, _, response) => {
            if is_exception {
                response.replace(Err(ExceptionResponse::from(src)));
            } else {
                response.replace(Ok(ReadDiscreteInputsResponse::from(src)));
            }
        }
        Request::ReadMultipleHoldingRegisters(_, _, response) => {
            if is_exception {
                response.replace(Err(ExceptionResponse::from(src)));
            } else {
                response.replace(Ok(ReadMultipleHoldingRegistersResponse::from(src)));
            }
        }
        Request::ReadInputRegisters(_, _, response) => {
            if is_exception {
                response.replace(Err(ExceptionResponse::from(src)));
            } else {
                response.replace(Ok(ReadInputRegistersResponse::from(src)));
            }
        }
        Request::WriteSingleCoil(_, _, response) => {
            if is_exception {
                response.replace(Err(ExceptionResponse::from(src)));
            } else {
                response.replace(Ok(WriteSingleCoilResponse::from(src)));
            }
        }
        Request::WriteSingleHoldingRegister(_, _, response) => {
            if is_exception {
                response.replace(Err(ExceptionResponse::from(src)));
            } else {
                response.replace(Ok(WriteSingleHoldingRegisterResponse::from(src)));
            }
        }
        Request::WriteMultipleCoils(_, _, response) => {
            if is_exception {
                response.replace(Err(ExceptionResponse::from(src)));
            } else {
                response.replace(Ok(WriteMultipleCoilsResponse::from(src)));
            }
        }
        Request::WriteMultipleHoldingRegisters(_, _, response) => {
            if is_exception {
                response.replace(Err(ExceptionResponse::from(src)));
            } else {
                response.replace(Ok(WriteMultipleHoldingRegistersResponse::from(src)));
            }
        }
    }
}
