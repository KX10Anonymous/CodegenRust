#[derive(PartialEq, Debug)]
#[repr(C)]
pub struct CodeSegment {
    pub code: i32,
    pub time: i32,
}