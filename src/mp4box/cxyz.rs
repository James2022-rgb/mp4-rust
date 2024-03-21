
use serde::Serialize;

use crate::mp4box::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CxyzBox {
    pub language_code: u16,
    pub text: String,
}

impl CxyzBox {
    pub fn get_type(&self) -> BoxType {
        BoxType::CxyzBox
    }

    pub fn get_size(&self) -> u64 {
        HEADER_SIZE + 2 + 2 + self.text.len() as u64
    }
}

impl Mp4Box for CxyzBox {
    fn box_type(&self) -> BoxType {
        self.get_type()
    }

    fn box_size(&self) -> u64 {
        self.get_size()
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        Ok(self.text.clone())
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for CxyzBox {
    fn read_box(reader: &mut R, _size: u64) -> Result<Self> {
        box_start(reader)?;

        let text_len = reader.read_u16::<BigEndian>()? as usize;
        let language_code = reader.read_u16::<BigEndian>()?;

        let mut text: Vec<u8> = vec![0; text_len];
        reader.read_exact(&mut text)?;

        let text = String::from_utf8_lossy(&text).into_owned();

        Ok(CxyzBox {
          language_code,
          text,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for CxyzBox {
    fn write_box(&self, writer: &mut W) -> Result<u64> {
        let size = self.box_size();
        BoxHeader::new(self.box_type(), size).write(writer)?;

        writer.write_u16::<BigEndian>(self.text.len() as u16)?;
        writer.write_u16::<BigEndian>(self.language_code)?;
        writer.write_all(self.text.as_bytes())?;

        Ok(size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_cxyz() {
        let src_box = CxyzBox {
          language_code: 0x15c7,
          text: "+41.3758+002.1492/".to_owned(),
        };

        let mut buf = Vec::new();
        src_box.write_box(&mut buf).unwrap();
        assert_eq!(buf.len(), src_box.box_size() as usize);

        let mut reader = Cursor::new(&buf);
        let header = BoxHeader::read(&mut reader).unwrap();
        assert_eq!(header.name, BoxType::CxyzBox);
        assert_eq!(header.size, src_box.box_size());

        let dst_box = CxyzBox::read_box(&mut reader, header.size).unwrap();
        assert_eq!(dst_box, src_box);
    }
}
