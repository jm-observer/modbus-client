use std::io::{Error, ErrorKind::InvalidData, Result};

use crate::{Request, Response};
use bytes::{Buf, Bytes, BytesMut};
use easy_modbus::{
    codec::get_function, util::crc, ExceptionResponse, Function,
    ReadCoilsResponse, ReadDiscreteInputsResponse,
    ReadInputRegistersResponse, ReadMultipleHoldingRegistersResponse,
    WriteMultipleCoilsResponse,
    WriteMultipleHoldingRegistersResponse, WriteSingleCoilResponse,
    WriteSingleHoldingRegisterResponse
};
use tokio_util::codec::Decoder;

impl Decoder for Request {
    type Error = Error;
    type Item = Response;

    fn decode(
        &mut self,
        src: &mut BytesMut
    ) -> Result<Option<Response>> {
        let Some(function) = src.get(1) else {
            return Ok(None);
        };
        let (function, is_exception) = get_function(*function)?;
        if !is_exception && function != self.head().function {
            return Err(Error::new(
                InvalidData,
                format!(
                    "Invalid function: {:0>2X} {:0>2X}",
                    function.to_code(),
                    self.head().function.to_code()
                )
            ));
        }

        let data_len: usize = {
            if is_exception {
                1
            } else {
                match function {
                    Function::ReadCoils
                    | Function::ReadDiscreteInputs
                    | Function::ReadMultipleHoldingRegisters
                    | Function::ReadInputRegisters => {
                        src.get(2).map_or(0, |&bytes_num| {
                            bytes_num as usize + 1
                        })
                    },
                    Function::WriteSingleCoil
                    | Function::WriteSingleHoldingRegister
                    | Function::WriteMultipleCoils
                    | Function::WriteMultipleHoldingRegisters => 4
                }
            }
        };
        let frame_len = data_len + 4;
        if src.len() < frame_len {
            return Ok(None);
        }
        // addr + function + data
        let body_bytes = src.copy_to_bytes(data_len + 2);
        let crc = src.get_u16();
        if crc::check(body_bytes.as_ref(), crc) {
            let rs = get_response(
                body_bytes.slice(2..),
                self.clone(),
                is_exception
            );
            return Ok(Some(rs));
        }
        return Err(Error::new(
            InvalidData,
            format!(
                "Invalid crc code: 0x{:0>2X}, {:?}",
                crc, body_bytes
            )
        ));
    }
}
//
// impl Decoder for TcpCodec {
//     type Item = Response;
//     type Error = Error;
//
//     fn decode(&mut self, src: &mut BytesMut) ->
// Result<Option<Response>> {         if src.len() < 4 {
//             return Ok(None);
//         }
//         let head = Head::tcp_try_from(src.copy_to_bytes(8))?;
//         let len = head.length as usize - 2;
//         let response = get_response(src.copy_to_bytes(len), head);
//         Ok(Some(response))
//     }
// }

fn get_response(
    src: Bytes,
    request: Request,
    is_exception: bool
) -> Response {
    match request {
        Request::ReadCoils(head, req) => {
            let rs = if is_exception {
                Err(ExceptionResponse::from(src))
            } else {
                Ok(ReadCoilsResponse::from(src))
            };
            Response::ReadCoils(head, req, rs)
        },
        Request::ReadDiscreteInputs(head, req) => {
            let rs = if is_exception {
                Err(ExceptionResponse::from(src))
            } else {
                Ok(ReadDiscreteInputsResponse::from(src))
            };
            Response::ReadDiscreteInputs(head, req, rs)
        },
        Request::ReadMultipleHoldingRegisters(head, req) => {
            let rs = if is_exception {
                Err(ExceptionResponse::from(src))
            } else {
                Ok(ReadMultipleHoldingRegistersResponse::from(src))
            };
            Response::ReadMultipleHoldingRegisters(head, req, rs)
        },
        Request::ReadInputRegisters(head, req) => {
            let rs = if is_exception {
                Err(ExceptionResponse::from(src))
            } else {
                Ok(ReadInputRegistersResponse::from(src))
            };
            Response::ReadInputRegisters(head, req, rs)
        },
        Request::WriteSingleCoil(head, req) => {
            let rs = if is_exception {
                Err(ExceptionResponse::from(src))
            } else {
                Ok(WriteSingleCoilResponse::from(src))
            };
            Response::WriteSingleCoil(head, req, rs)
        },
        Request::WriteSingleHoldingRegister(head, req) => {
            let rs = if is_exception {
                Err(ExceptionResponse::from(src))
            } else {
                Ok(WriteSingleHoldingRegisterResponse::from(src))
            };
            Response::WriteSingleHoldingRegister(head, req, rs)
        },
        Request::WriteMultipleCoils(head, req) => {
            let rs = if is_exception {
                Err(ExceptionResponse::from(src))
            } else {
                Ok(WriteMultipleCoilsResponse::from(src))
            };
            Response::WriteMultipleCoils(head, req, rs)
        },
        Request::WriteMultipleHoldingRegisters(head, req) => {
            let rs = if is_exception {
                Err(ExceptionResponse::from(src))
            } else {
                Ok(WriteMultipleHoldingRegistersResponse::from(src))
            };
            Response::WriteMultipleHoldingRegisters(head, req, rs)
        }
    }
}
