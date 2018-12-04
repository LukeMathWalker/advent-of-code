pub struct BoxId {
    id: String,
}

impl From<String> for BoxId {
    fn from(s: String) -> Self {
        BoxId {id: s}
    }
}