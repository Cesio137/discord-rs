use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::model::resources::channel::ChannelType;
use crate::model::resources::emoji::Emoji;

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
    Premium = 6,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum TextInputStyle {
    Short = 1,
    Paragraph = 2,
}

/*TAGS*/
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Component {
    #[serde(rename = "1")]
    ActionRow {
        components: Vec<Component>,
    },
    #[serde(rename = "2")]
    Button {
        style: ButtonStyle,
        label: Option<String>,
        emoji: Option<super::super::resources::emoji::Emoji>,
        custom_id: Option<String>,
        sku_id: Option<String>,
        url: Option<String>,
        disabled: Option<bool>,
    },
    #[serde(rename = "3")]
    SelectMenu {
        custom_id: String,
        options: Option<Vec<SelectOption>>,
        channel_types: Option<Vec<ChannelType>>,
        placeholder: Option<String>,
        default_values: Option<Vec<SelectDefaultValue>>,
        min_values: Option<u32>,
        max_values: Option<u32>,
        disabled: Option<bool>,
    },
    #[serde(rename = "4")]
    TextInput {
        custom_id: String,
        style: TextInputStyle,
        label: String,
        min_length: Option<u32>,
        max_length: Option<u32>,
        required: Option<bool>,
        value: Option<String>,
        placeholder: Option<String>,
    },
    #[serde(rename = "5")]
    UserSelect {
        custom_id: String,
        options: Option<Vec<SelectOption>>,
        channel_types: Option<Vec<ChannelType>>,
        placeholder: Option<String>,
        default_values: Option<Vec<SelectDefaultValue>>,
        min_values: Option<u32>,
        max_values: Option<u32>,
        disabled: Option<bool>,
    },
    #[serde(rename = "6")]
    RoleSelect {
        custom_id: String,
        options: Option<Vec<SelectOption>>,
        channel_types: Option<Vec<ChannelType>>,
        placeholder: Option<String>,
        default_values: Option<Vec<SelectDefaultValue>>,
        min_values: Option<u32>,
        max_values: Option<u32>,
        disabled: Option<bool>,
    },
    #[serde(rename = "7")]
    MentionableSelect {
        custom_id: String,
        options: Option<Vec<SelectOption>>,
        channel_types: Option<Vec<ChannelType>>,
        placeholder: Option<String>,
        default_values: Option<Vec<SelectDefaultValue>>,
        min_values: Option<u32>,
        max_values: Option<u32>,
        disabled: Option<bool>,
    },
    #[serde(rename = "8")]
    ChannelSelect {
        custom_id: String,
        options: Option<Vec<SelectOption>>,
        channel_types: Option<Vec<ChannelType>>,
        placeholder: Option<String>,
        default_values: Option<Vec<SelectDefaultValue>>,
        min_values: Option<u32>,
        max_values: Option<u32>,
        disabled: Option<bool>,
    },
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