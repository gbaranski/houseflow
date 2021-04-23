use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Frame {
    /// First frame that should be sent from Client to Server
    ///
    /// Opcode: 0x01
    Connect {
        client_id: [u8; 16],
    },
    /// First frame that should be sent from Server to Client as a response for Connect
    ///
    /// Opcode: 0x02
    ConnACK {
        response_code: u8,
    },

}
