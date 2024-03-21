
use serde::Serialize;

use crate::mp4box::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GpmfBox {
    pub data: Vec<u8>,
}

impl GpmfBox {
    pub fn get_type(&self) -> BoxType {
        BoxType::GpmfBox
    }

    pub fn get_size(&self) -> u64 {
        HEADER_SIZE + self.data.len() as u64
    }
}

impl Mp4Box for GpmfBox {
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
        Ok(format!("Data: {} bytes", self.data.len()))
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for GpmfBox {
    fn read_box(reader: &mut R, size: u64) -> Result<Self> {
        box_start(reader)?;

        let data_size = size - HEADER_SIZE;

        let mut data: Vec<u8> = vec![0; data_size as usize];
        reader.read_exact(&mut data)?;

        Ok(GpmfBox {
          data,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for GpmfBox {
    fn write_box(&self, writer: &mut W) -> Result<u64> {
        let size = self.box_size();
        BoxHeader::new(self.box_type(), size).write(writer)?;

        writer.write_all(&self.data)?;

        Ok(self.get_size())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_gpmf() {
        // Just check that we get the same data back.

        let src_box = GpmfBox {
            data: vec![0x01, 0x02, 0x03, 0x04],
        };

        let mut buf = Vec::new();
        src_box.write_box(&mut buf).unwrap();
        assert_eq!(buf.len(), src_box.box_size() as usize);

        let mut reader = Cursor::new(&buf);
        let header = BoxHeader::read(&mut reader).unwrap();
        assert_eq!(header.name, BoxType::GpmfBox);
        assert_eq!(header.size, src_box.box_size());

        let dst_box = GpmfBox::read_box(&mut reader, header.size).unwrap();
        assert_eq!(dst_box, src_box);
    }
}
