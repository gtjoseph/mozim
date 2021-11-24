#[derive(Debug, PartialEq, Clone)]
pub enum ErrorKind {
    InvalidArgument,
    Bug,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DhcpError {
    kind: ErrorKind,
    msg: String,
}

impl DhcpError {
    pub fn new(kind: ErrorKind, msg: String) -> Self {
        Self {
            kind: kind,
            msg: msg,
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for DhcpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

impl From<std::io::Error> for DhcpError {
    fn from(e: std::io::Error) -> Self {
        Self::new(ErrorKind::Bug, format!("IO error: {}", e))
    }
}

impl From<std::ffi::NulError> for DhcpError {
    fn from(e: std::ffi::NulError) -> Self {
        Self::new(ErrorKind::Bug, format!("CString error: {}", e))
    }
}

impl From<dhcproto::v4::EncodeError> for DhcpError {
    fn from(e: dhcproto::v4::EncodeError) -> Self {
        Self::new(ErrorKind::Bug, format!("DHCP protocol error: {}", e))
    }
}

impl From<etherparse::WriteError> for DhcpError {
    fn from(e: etherparse::WriteError) -> Self {
        Self::new(ErrorKind::Bug, format!("etherparse protocol error: {}", e))
    }
}