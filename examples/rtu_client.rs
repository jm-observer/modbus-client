use futures::{SinkExt, StreamExt};
use tokio_serial::SerialStream;
use tokio_util::codec::Framed;

use modbus_client::{Request, Response};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tty_path = "COM6";
    let rate = 9600;
    let slave = 0x01;

    let serial_builder = tokio_serial::new(tty_path, rate);
    let port = SerialStream::open(&serial_builder).unwrap();

    let rq =
        Request::read_multiple_holding_registers_request(slave, 0, 2);
    let mut transport = Framed::new(port, rq);

    transport.send(()).await?;
    // 01 03 02 27 C5 63 E7
    // 01 03 02 27 C5 63 E7
    while let Some(response) = transport.next().await {
        match response {
            Ok(response) => {
                // println!("Response:\t{:?}", response);
                match response {
                    Response::ReadMultipleHoldingRegisters(
                        _,
                        _,
                        res
                    ) => {
                        let res = res.unwrap();
                        let a = res.get_values();
                        println!(
                            "{:?} {}",
                            a,
                            u16::from_be_bytes([
                                *(a.get(2).unwrap()),
                                *(a.get(3).unwrap())
                            ]) as f32
                                / 10.0
                        );
                        return Ok(());
                    },
                    _ => {
                        println!("unknown")
                    }
                }
            },
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}
