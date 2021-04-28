use super::ClientID;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Frame {
    pub client_id: ClientID,
}
