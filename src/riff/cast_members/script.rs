use crate::reader::{ReadBytesExt, Reader};

#[derive(Debug)]
pub enum ScriptType {
    Score,
    Movie,
    Parent,
}

pub struct InvalidScriptTypeValue(pub u16);

impl TryFrom<u16> for ScriptType {
    type Error = InvalidScriptTypeValue;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ScriptType::Score),
            3 => Ok(ScriptType::Movie),
            7 => Ok(ScriptType::Parent),
            _ => Err(InvalidScriptTypeValue(value)),
        }
    }
}

#[derive(Debug)]
pub struct Script {
    pub r#type: ScriptType,
}

impl Script {
    pub fn read(r: &mut Reader) -> std::io::Result<Self> {
        let r#type = r
            .read_be_u16()?
            .try_into()
            .map_err(|e: InvalidScriptTypeValue| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid script type {}", e.0),
                )
            })?;

        Ok(Script { r#type })
    }
}
