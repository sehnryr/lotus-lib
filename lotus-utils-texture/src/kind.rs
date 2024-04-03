use anyhow::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextureKind {
    DiffuseEmissionTint = 0xA3,
    BillboardSpritemapDiffuse = 0xA4,
    BillboardSpritemapNormal = 0xA5,
    Roughness = 0xA7,
    Skybox = 0xAB,
    Texture174 = 0xAE, // TODO: Rename to something more descriptive
    Texture176 = 0xB0, // TODO: Rename to something more descriptive
    Cubemap = 0xB1,
    NormalMap = 0xB8,
    Packmap = 0xBC,
    Texture194 = 0xC2, // TODO: Rename to something more descriptive
    DetailsPack = 0xC3,
}

impl TryFrom<u32> for TextureKind {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0xA3 => Ok(TextureKind::DiffuseEmissionTint),
            0xA4 => Ok(TextureKind::BillboardSpritemapDiffuse),
            0xA5 => Ok(TextureKind::BillboardSpritemapNormal),
            0xA7 => Ok(TextureKind::Roughness),
            0xAB => Ok(TextureKind::Skybox),
            0xAE => Ok(TextureKind::Texture174),
            0xB0 => Ok(TextureKind::Texture176),
            0xB1 => Ok(TextureKind::Cubemap),
            0xB8 => Ok(TextureKind::NormalMap),
            0xBC => Ok(TextureKind::Packmap),
            0xC2 => Ok(TextureKind::Texture194),
            0xC3 => Ok(TextureKind::DetailsPack),
            _ => Err(Error::msg("Unknown texture kind")),
        }
    }
}
