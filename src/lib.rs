use crate::codec::request_to_bytesmut;
use bytes::BytesMut;
use easy_modbus::*;

mod codec;

#[derive(Clone)]
pub enum Request {
    ReadCoils(Head, ReadCoilsRequest),
    ReadDiscreteInputs(Head, ReadDiscreteInputsRequest),
    ReadMultipleHoldingRegisters(
        Head,
        ReadMultipleHoldingRegistersRequest
    ),
    ReadInputRegisters(Head, ReadInputRegistersRequest),
    WriteSingleCoil(Head, WriteSingleCoilRequest),
    WriteSingleHoldingRegister(
        Head,
        WriteSingleHoldingRegisterRequest
    ),
    WriteMultipleCoils(Head, WriteMultipleCoilsRequest),
    WriteMultipleHoldingRegisters(
        Head,
        WriteMultipleHoldingRegistersRequest
    )
}

impl Request {
    pub fn head(&self) -> &Head {
        match self {
            Request::ReadCoils(head, _) => head,
            Request::ReadDiscreteInputs(head, _) => head,
            Request::ReadMultipleHoldingRegisters(head, _) => head,
            Request::ReadInputRegisters(head, _) => head,
            Request::WriteSingleCoil(head, _) => head,
            Request::WriteSingleHoldingRegister(head, _) => head,
            Request::WriteMultipleCoils(head, _) => head,
            Request::WriteMultipleHoldingRegisters(head, _) => head
        }
    }

    pub fn to_bytes(&self, data: &mut BytesMut) {
        request_to_bytesmut(self, data)
    }

    /// Create a TCP frame
    ///
    /// A Modbus variant used for communications over TCP/IP networks.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let tcp = Frame::tcp();
    /// ```
    // pub fn tcp() -> Frame {
    //     Frame {
    //         version: Version::Tcp,
    //         tid_map: Mutex::new(HashMap::new()),
    //     }
    // }

    /// Create a RTU frame
    ///
    /// Used in serial communication, and is the most common
    /// implementation available for Modbus.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let rut = Frame::rtu();
    /// ```
    // pub fn rtu() -> Frame {
    //     Frame {
    //         version: Version::Rtu,
    //         tid_map: Mutex::new(HashMap::new()),
    //     }
    // }

    /// Create a read coils request (Function Code: 0x01)
    ///
    /// * `unit_id` -  Server address
    /// * `first_address` - Address of first coil to read
    /// * `number` - Number of coils to read
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().read_coils_request(0x01, 0x02, 0x08);
    /// ```
    pub fn read_coils_request(
        unit_id: u8,
        first_address: u16,
        number: u16
    ) -> Request {
        let function = Function::ReadCoils;
        let request_body =
            ReadCoilsRequest::new(first_address, number);
        let head = Self::init_head(
            unit_id,
            function,
            request_body.len(),
            false
        );
        Request::ReadCoils(head, request_body)
    }

    /// Create a read discrete Request (Function Code: 0x02)
    ///
    /// * `unit_id` -  Server address
    /// * `first_address` - Address of first discrete input to read
    /// * `number` - Number of discrete input to read
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().read_discrete_request(0x0B, 0x007A, 0x001C);
    /// ```
    pub fn read_discrete_request(
        &self,
        unit_id: u8,
        first_address: u16,
        number: u16
    ) -> Request {
        let function = Function::ReadDiscreteInputs;
        let request_body =
            ReadDiscreteInputsRequest::new(first_address, number);
        let head = Self::init_head(
            unit_id,
            function,
            request_body.len(),
            false
        );
        Request::ReadDiscreteInputs(head, request_body)
    }

    /// Create a read multiple holding registers request (Function
    /// Code: 0x03)
    ///
    /// * `unit_id` -  Server address
    /// * `first_address` - Address of first register to read
    /// * `number` - Number of discrete input to read
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().read_multiple_holding_registers_request(0x0B, 0x006F, 0x0003);
    /// ```
    pub fn read_multiple_holding_registers_request(
        unit_id: u8,
        first_address: u16,
        number: u16
    ) -> Request {
        let function = Function::ReadMultipleHoldingRegisters;
        let request_body = ReadMultipleHoldingRegistersRequest::new(
            first_address,
            number
        );
        let head = Self::init_head(
            unit_id,
            function,
            request_body.len(),
            false
        );
        Request::ReadMultipleHoldingRegisters(head, request_body)
    }

    /// Create a read input registers request (Function Code: 0x04)
    ///
    /// * `unit_id` -  Server address
    /// * `first_address` - Address of first register to read
    /// * `number` - Number of registers to read
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().read_input_registers_request(0x0B, 0x000A, 0x0001);
    /// ```
    pub fn read_input_registers_request(
        &self,
        unit_id: u8,
        first_address: u16,
        number: u16
    ) -> Request {
        let function = Function::ReadInputRegisters;
        let request_body =
            ReadInputRegistersRequest::new(first_address, number);
        let head = Self::init_head(
            unit_id,
            function,
            request_body.len(),
            false
        );
        Request::ReadInputRegisters(head, request_body)
    }

    /// Create a write single coil request (Function Code: 0x05)
    ///
    /// * `unit_id` -  Server address
    /// * `address` - Address of coil to write
    /// * `value` - Value to write. 0 (0x0000) for off, 65,280
    ///   (0xFF00) for on.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().write_single_coil_request(0x0B, 0x00BF, 0x0000);
    /// ```
    pub fn write_single_coil_request(
        &self,
        unit_id: u8,
        address: u16,
        value: u16
    ) -> Request {
        let function = Function::WriteSingleCoil;
        let request_body =
            WriteSingleCoilRequest::new(address, value);
        let head = Self::init_head(
            unit_id,
            function,
            request_body.len(),
            false
        );
        Request::WriteSingleCoil(head, request_body)
    }

    /// Create a write single holding register request (Function Code:
    /// 0x06)
    ///
    /// * `unit_id` -  Server address
    /// * `address` - Address of Holding Register to write
    /// * `value` - Value to write
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().write_single_holding_register_request(0x0B, 0x0004, 0xABCD);
    /// ```
    pub fn write_single_holding_register_request(
        &self,
        unit_id: u8,
        address: u16,
        value: u16
    ) -> Request {
        let function = Function::WriteSingleHoldingRegister;
        let request_body =
            WriteSingleHoldingRegisterRequest::new(address, value);
        let head = Self::init_head(
            unit_id,
            function,
            request_body.len(),
            false
        );
        Request::WriteSingleHoldingRegister(head, request_body)
    }

    /// Create a write multiple coils request (Function Code: 0x0F)
    ///
    /// * `unit_id` -  Server address
    /// * `address` - Address of Holding Register to write
    /// * `coils_number` - Number of coils to write
    /// * `values` - Coil values. Value of each coil is binary (0 for
    ///   off, 1 for on).
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().write_multiple_coils_request(
    ///     0x0B,
    ///     0x001B,
    ///     0x0009,
    ///     vec![0x4D, 0x01]
    /// );
    /// ```
    pub fn write_multiple_coils_request(
        &self,
        unit_id: u8,
        address: u16,
        coils_number: u16,
        values: Vec<u8>
    ) -> Request {
        let function = Function::WriteMultipleCoils;
        let request_body = WriteMultipleCoilsRequest::new(
            address,
            coils_number,
            values
        );
        let head = Self::init_head(
            unit_id,
            function,
            request_body.len(),
            false
        );
        Request::WriteMultipleCoils(head, request_body)
    }

    /// Create a write multiple coils request (Function Code: 0x10)
    ///
    /// * `unit_id` -  Server address
    /// * `address` - Address of first holding registers to write
    /// * `values` - New values of holding registers
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().write_multiple_holding_registers_request(
    ///     0x0B,
    ///     0x0012,
    ///     vec![0x0B, 0x0A, 0xC1, 0x02],
    /// );
    /// ```
    pub fn write_multiple_holding_registers_request(
        &self,
        unit_id: u8,
        address: u16,
        values: Vec<u8>
    ) -> Request {
        let function = Function::WriteMultipleHoldingRegisters;
        let request_body = WriteMultipleHoldingRegistersRequest::new(
            address, values
        );
        let head = Self::init_head(
            unit_id,
            function,
            request_body.len(),
            false
        );
        Request::WriteMultipleHoldingRegisters(head, request_body)
    }

    /// Build modbus message head
    fn init_head(
        uid: u8,
        function: Function,
        body_length: u16,
        is_exception: bool
    ) -> Head {
        // todo
        Head::new(
            Self::get_tid(uid),
            uid,
            function,
            body_length,
            Version::Rtu,
            is_exception
        )
    }

    /// Get tid by uid from tid_map
    fn get_tid(_unit_id: u8) -> u16 {
        return 0;
        // todo
        // if self.version == Version::Rtu {
        //     return 0;
        // }

        // let mut map = self.tid_map.lock().unwrap();
        // let value = match map.get(&unit_id) {
        //     None => 1,
        //     Some(v) => {
        //         if v < &0xFFFF {
        //             v + 1
        //         } else {
        //             1
        //         }
        //     }
        // };
        // map.insert(unit_id, value);
        // value
    }
}

pub enum Response {
    ReadCoils(
        Head,
        ReadCoilsRequest,
        Result<ReadCoilsResponse, ExceptionResponse>
    ),
    ReadDiscreteInputs(
        Head,
        ReadDiscreteInputsRequest,
        Result<ReadDiscreteInputsResponse, ExceptionResponse>
    ),
    ReadMultipleHoldingRegisters(
        Head,
        ReadMultipleHoldingRegistersRequest,
        Result<
            ReadMultipleHoldingRegistersResponse,
            ExceptionResponse
        >
    ),
    ReadInputRegisters(
        Head,
        ReadInputRegistersRequest,
        Result<ReadInputRegistersResponse, ExceptionResponse>
    ),
    WriteSingleCoil(
        Head,
        WriteSingleCoilRequest,
        Result<WriteSingleCoilResponse, ExceptionResponse>
    ),
    WriteSingleHoldingRegister(
        Head,
        WriteSingleHoldingRegisterRequest,
        Result<WriteSingleHoldingRegisterResponse, ExceptionResponse>
    ),
    WriteMultipleCoils(
        Head,
        WriteMultipleCoilsRequest,
        Result<WriteMultipleCoilsResponse, ExceptionResponse>
    ),
    WriteMultipleHoldingRegisters(
        Head,
        WriteMultipleHoldingRegistersRequest,
        Result<
            WriteMultipleHoldingRegistersResponse,
            ExceptionResponse
        >
    )
}
