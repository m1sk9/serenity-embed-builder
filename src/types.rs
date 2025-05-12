/// A collection of Embed structures that works like [serenity::model::Embed] in Serenity. Except for Timestamp, all members are implemented independently and can be created by the Builder using Typed_Builder.
#[derive(Debug, Clone, PartialEq, typed_builder::TypedBuilder)]
pub struct SerenityEmbedArgs {
    #[builder(setter(strip_option, into), default)]
    pub title: Option<String>,
    #[builder(default = "rich".to_string(), setter(into))]
    pub kind: String,
    #[builder(setter(strip_option, into), default)]
    pub description: Option<String>,
    #[builder(setter(strip_option, into), default)]
    pub url: Option<String>,
    #[builder(setter(into), default)]
    pub timestamp: Option<serenity::model::Timestamp>,
    #[builder(setter(into), default)]
    pub color: Option<u32>,
    #[builder(setter(into), default)]
    pub footer: Option<SerenityEmbedFooterArgs>,
    #[builder(setter(into), default)]
    pub image: Option<SerenityMediaArgs>,
    #[builder(setter(into), default)]
    pub thumbnail: Option<SerenityMediaArgs>,
    #[builder(setter(into), default)]
    pub video: Option<SerenityMediaArgs>,
    #[builder(setter(into), default)]
    pub provider: Option<SerenityEmbedProviderArgs>,
    #[builder(setter(into), default)]
    pub author: Option<SerenityEmbedAuthorArgs>,
    #[builder(setter(into), default)]
    pub fields: Option<Vec<SerenityEmbedFieldArgs>>,

}

/// Structure to build the footer portion of the Serenity embedding.
#[derive(Debug, Clone, PartialEq, typed_builder::TypedBuilder)]
pub struct SerenityEmbedFooterArgs {
    #[builder(setter(strip_option, into), default)]
    pub text: Option<String>,
    #[builder(default = "https://cdn.discordapp.com/embed/avatars/0.png".to_string(), setter(into))]
    pub icon_url: String,
    #[builder(setter(strip_option, into), default)]
    pub proxy_icon_url: Option<String>,
}

/// Structures related to images, videos, thumbnails, and other media in Serenity embedding.
#[derive(Debug, Clone, PartialEq, typed_builder::TypedBuilder)]
pub struct SerenityMediaArgs {
    #[builder(setter(strip_option, into), default)]
    pub url: Option<String>,
    #[builder(setter(strip_option, into), default)]
    pub proxy_url: Option<String>,
    #[builder(setter(into), default)]
    pub height: Option<u32>,
    #[builder(setter(into), default)]
    pub width: Option<u32>,
}

/// Structure to build the provider portion of the Serenity embedding.
#[derive(Debug, Clone, PartialEq, typed_builder::TypedBuilder)]
pub struct SerenityEmbedProviderArgs {
    #[builder(setter(strip_option, into), default)]
    pub name: Option<String>,
    #[builder(setter(strip_option, into), default)]
    pub url: Option<String>,
}

/// Structure to build the author portion of the Serenity embedding.
#[derive(Debug, Clone, PartialEq, typed_builder::TypedBuilder)]
pub struct SerenityEmbedAuthorArgs {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(strip_option, into), default)]
    pub url: Option<String>,
    #[builder(default = "https://cdn.discordapp.com/embed/avatars/0.png".to_string(), setter(into))]
    pub icon_url: String,
    #[builder(setter(strip_option, into), default)]
    pub proxy_icon_url: Option<String>,
}

/// Structure to build the fields portion of the Serenity embedding.
#[derive(Debug, Clone, PartialEq, typed_builder::TypedBuilder)]
pub struct SerenityEmbedFieldArgs {
    #[builder(setter(into), default)]
    pub name: String,
    #[builder(setter(into), default)]
    pub value: String,
    #[builder(setter(into), default)]
    pub inline: Option<bool>,
}
