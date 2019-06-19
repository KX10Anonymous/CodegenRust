use super::code_segment::CodeSegment;

pub struct Fingerprint {
    pub codes : Vec<CodeSegment>
}

impl Fingerprint {
    pub fn new(codes: Vec<CodeSegment>) -> Self {
        Fingerprint {
            codes: codes,
        }
    }
}