use crate::model::embed::SerenityEmbed;
use serenity::all::{MessageReference, RoleId, StickerId, UserId};

/// An enumeration representing the different types of mentions that can be included in a Discord message.
/// These mention types allow you to specify who or what should be mentioned in the message.
#[derive(serde::Deserialize, Clone)]
pub enum SerenityMessageMentionType {
    /// Mentions everyone in the guild.
    Everyone,
    /// Mentions online members in the guild.
    Here,
    /// Mentions specific users by their user IDs.
    Users(Vec<UserId>),
    /// Mentions specific roles by their role IDs.
    Roles(Vec<RoleId>),
    /// Mentions a specific message.
    Reply(MessageReference),
}

/// A builder for creating Discord messages using Serenity.
/// This struct allows you to specify various parameters for the message.
/// such as content, embeds, text-to-speech (TTS) settings, mention types, and sticker IDs.
///
/// **Notes:** At the time of the v0.3.0 release, serenity-builder can only generate messages consisting of text strings, embeds, and stickers.
#[derive(serde::Deserialize, typed_builder::TypedBuilder, Clone)]
pub struct SerenityMessage {
    /**
     * The content of the message.
     *
     * Due to Discord API limitations, a maximum of 2000 characters can be used. If the character count exceeds this limit, [crate::message::SerenityMessageConvertError::TooLongContent] will be returned during conversion.
     */
    #[builder(default, setter(strip_option, into))]
    pub content: Option<String>,
    /// The embeds to include in the message.
    #[builder(default, setter(strip_option, into))]
    pub embeds: Option<Vec<SerenityEmbed>>,
    /// Whether the message should be sent as text-to-speech (TTS).
    #[builder(default = false, setter(into))]
    pub tts: bool,
    /**
     * The type of mentions to include in the message.
     * For the types of mentions that can be specified, see [crate::model::message::SerenityMessageMentionType].
     */
    #[builder(default, setter(strip_option, into))]
    pub mention_type: Option<SerenityMessageMentionType>,
    /// The sticker IDs to include in the message.
    #[builder(default, setter(strip_option, into))]
    pub sticker_ids: Option<Vec<StickerId>>,
}
