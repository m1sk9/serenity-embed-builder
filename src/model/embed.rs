/// A builder for creating Discord embeds.
/// This struct allows you to build rich embed messages for Discord using a fluent interface.
///
/// Fundamentally, this structure provides the same functionality as Serenity, but this library explicitly removes values that became unnecessary during optimization as a builder.
///
/// # Structure Values
///
/// In Serenity, embeddings are composed of multiple structures such as [serenity::builder::CreateEmbedAuthor].
///
/// serenity-builder represents these structures as Triples or Doubles. Please refer to the documentation to determine which values correspond to them.
///
/// [What is Triples or Doubles?](https://doc.rust-lang.org/book/ch03-02-data-types.html)
///
/// # Values
///
/// The following values have been removed from [serenity::model::channel::Embed]:
///
/// - `kind`: The type of embed. Discord currently only supports "rich" embeds, so this field is unnecessary.
/// - `video`: `rich` embeds do not support video content, so this field is unnecessary.
/// - `provider`: `rich` embeds do not support provider information, so this field is unnecessary.
#[derive(serde::Deserialize, typed_builder::TypedBuilder, Clone)]
pub struct SerenityEmbed {
    /// The title of the embed.
    #[builder(default, setter(strip_option, into))]
    pub title: Option<String>,
    /**
     * The description of the embed.
     *
     * Due to Discord API limitations, a maximum of 4096 characters can be used. If the character count exceeds this limit, [crate::embed::SerenityEmbedConvertError::TooLongDescription] will be returned during conversion.
     */
    #[builder(default, setter(strip_option, into))]
    pub description: Option<String>,
    // The url of the embed.
    #[builder(default, setter(strip_option, into))]
    pub url: Option<String>,
    /**
     * The timestamp of the embed content.
     * It will be displayed at the bottom of the embed.
     */
    #[builder(default, setter(strip_option, into))]
    pub timestamp: Option<serenity::all::Timestamp>,
    /**
     * The color of the embed.
     *
     * Serenity uses [serenity::model::colour::Colour], but Builder only allows direct specification from color codes.
     * e.g. `0xff0000` for red.
     */
    #[builder(default, setter(strip_option, into))]
    pub color: Option<u32>,
    /// The footer of the embed.
    #[builder(default, setter(strip_option, into))]
    pub footer_text: Option<String>,
    /// The footer icon url of the embed.
    #[builder(default, setter(strip_option, into))]
    pub footer_icon_url: Option<String>,
    /// The image url of the embed.
    #[builder(default, setter(strip_option, into))]
    pub image_url: Option<String>,
    /// The thumbnail url of the embed.
    #[builder(default, setter(strip_option, into))]
    pub thumbnail_url: Option<String>,
    /// The author name of the embed.
    #[builder(default, setter(strip_option, into))]
    pub author_name: Option<String>,
    /// The author url of the embed.
    #[builder(default, setter(strip_option, into))]
    pub author_url: Option<String>,
    /// The author icon url of the embed.
    #[builder(default, setter(strip_option, into))]
    pub author_icon_url: Option<String>,
    /**
     * The fields of the embed. (up to 25 fields)
     *
     * Due to Discord API limitations, only 25 fields can be used. Any additional fields will result in a [crate::embed::SerenityEmbedConvertError::TooManyFields] being returned during conversion.
     */
    #[builder(default, setter(strip_option, into))]
    pub fields: Option<Vec<SerenityEmbedField>>,
}

/// Field structures used in [SerenityEmbed].
/// These structures can be used as Vec (arrays) in [SerenityEmbed] and are internally converted to be handled by [serenity::model::channel::Embed].
#[derive(serde::Deserialize, typed_builder::TypedBuilder, Clone)]
pub struct SerenityEmbedField {
    /// The name of the field.
    #[builder(setter(into))]
    pub name: String,
    /// The value of the field.
    #[builder(setter(into))]
    pub value: String,
    /// Whether the field is displayed inline. (default: false)
    #[builder(default = false, setter(into))]
    pub inline: bool,
}
