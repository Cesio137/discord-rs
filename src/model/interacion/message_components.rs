use serde::{Deserialize, Serialize};
use crate::internal::traits::DiscordTypes;
use crate::model::resources::channel::ChannelType;
use crate::model::resources::emoji::Emoji;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Component {
    #[serde(rename = "1")]
    ActionRow {
        components: Vec<Component>,
    },
    #[serde(rename = "2")]
    Button(Button),
    #[serde(rename = "3")]
    SelectMenu(SelectMenu),
    #[serde(rename = "4")]
    TextInput(TextInput),
    #[serde(rename = "5")]
    UserSelect(SelectMenu),
    #[serde(rename = "6")]
    RoleSelect(SelectMenu),
    #[serde(rename = "7")]
    MentionableSelect(SelectMenu),
    #[serde(rename = "8")]
    ChannelSelect(SelectMenu),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
    Premium = 6,
}

impl DiscordTypes for ButtonStyle {
    fn from(value: u8) -> Self {
        match value {
            1 => ButtonStyle::Primary,
            2 => ButtonStyle::Secondary,
            3 => ButtonStyle::Success,
            4 => ButtonStyle::Danger,
            5 => ButtonStyle::Link,
            6 => ButtonStyle::Premium,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> u8 {
        match self {
            ButtonStyle::Primary => 1,
            ButtonStyle::Secondary => 2,
            ButtonStyle::Success => 3,
            ButtonStyle::Danger => 4,
            ButtonStyle::Link => 5,
            ButtonStyle::Premium => 6,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TextInputStyle {
    Short = 1,
    Paragraph = 2,
}

impl DiscordTypes for TextInputStyle {
    fn from(value: u8) -> Self {
        match value {
            1 => TextInputStyle::Short,
            2 => TextInputStyle::Paragraph,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> u8 {
        match self {
            TextInputStyle::Short => 1,
            TextInputStyle::Paragraph => 2,
        }
    }
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Button {
    pub style: ButtonStyle,
    pub label: Option<String>,
    pub emoji: Option<super::super::resources::emoji::Emoji>,
    pub custom_id: Option<String>,
    pub sku_id: Option<String>,
    pub url: Option<String>,
    pub disabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectMenu {
    pub custom_id: String,
    pub options: Option<Vec<SelectOption>>,
    pub channel_types: Option<Vec<ChannelType>>,
    pub placeholder: Option<String>,
    pub default_values: Option<Vec<SelectDefaultValue>>,
    pub min_values: Option<u32>,
    pub max_values: Option<u32>,
    pub disabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    pub discription: Option<String>,
    pub emoji: Option<Emoji>,
    pub default: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectDefaultValue {
    pub id: String,
    pub format_type: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextInput {
    pub custom_id: String,
    pub style: TextInputStyle,
    pub label: String,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub required: Option<bool>,
    pub value: Option<String>,
    pub placeholder: Option<String>,
} 