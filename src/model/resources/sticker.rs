use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum StickerType {
    STANDART = 1,
    GUILD = 2
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum StickerFormatType {
    PNG = 1,
    APNG = 2,
    LOTTIE = 3,
    GIF = 4
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sticker {
    pub id: String,
    pub pack_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,
    #[serde(rename = "type")]
    pub format_type: StickerType,
    #[serde(rename = "format_type")]
    pub format: StickerFormatType,
    pub available: Option<bool>,
    pub guild_id: Option<String>,
    pub user: Option<super::user::User>,
    pub sort_value: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StickerItem {
    pub id: String,
    pub name: String,
    #[serde(rename = "format_type")]
    pub format: StickerFormatType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StickerPack {
    pub id: String,
    pub stickers: Vec<Sticker>,
    pub name: String,
    pub sku_id: Option<String>,
    pub cover_sticker_id: Option<String>,
    pub description: String,
    pub banner_asset_id: Option<String>,
}