#[cfg(test)]
pub mod tests {
    use std::fs;
    use std::io::prelude::*;
    use std::io;
    use std::path::Path;
    use byteorder::{LittleEndian, ByteOrder};
    use super::super::super::structures::code_segment::CodeSegment;

    pub fn get_test_data<P>(path: P) -> io::Result<Vec<CodeSegment>>
        where P: AsRef<Path>
    {
        let file = fs::File::open(&path)?;
        let fize_size = file.metadata()?.len() as usize;
        let mut buffered_reader = io::BufReader::new(file);

        let mut test_data = Vec::with_capacity(fize_size / 8);

        let mut buffer = [0; 8];
        while let Ok(()) = buffered_reader.read_exact(&mut buffer) {
            let code_segment = CodeSegment {
                code: LittleEndian::read_i32(&buffer[..4]),
                time: LittleEndian::read_i32(&buffer[4..]),
            };
            test_data.push(code_segment);
        }

        Ok(test_data)
    }

    #[test]
    fn proper_test_data()
    {
        let data = get_test_data("test_data").unwrap();

        assert_eq!(data.len(), 6948);
        assert_eq!(data[0].code, 446903);
        assert_eq!(data[0].time, 64);

        assert_eq!(data[6947].code, 809256);
        assert_eq!(data[6947].time, 11739);
    }
}