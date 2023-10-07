//! Codec based [tokio-util](https://docs.rs/tokio-util/latest/tokio_util/codec/index.html)

mod decoder;
mod encoder;

/// Mutual convert TCP Client frames and buffers.
#[derive(Debug, Default)]
pub struct TcpCodec;

/// Mutual convert RTU Client frames and buffers.
#[derive(Debug, Default)]
pub struct RtuCodec;
