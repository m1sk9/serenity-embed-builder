use serenity::all::{Colour, CreateEmbed};

use crate::model::embed::SerenityEmbed;

/// Errors that can occur when converting a [SerenityEmbed] to a [serenity::all::CreateEmbed].
#[derive(thiserror::Error, Debug)]
pub enum SerenityEmbedConvertError {
    /**
     * This occurs when the embedded description exceeds 4096 characters and is subject to Discord API limitations.
     *
     * Serenity-builder will report an error during conversion.
     * Serenity does not return an error and leaves the response entirely up to the Discord API.
     */
    #[error("The description exceeds the maximum length of 4096 characters.")]
    TooLongDescription,
    /**
     * This occurs when the number of embedded fields exceeds 25 and hits the Discord API limit.
     *
     * Serenity-builder will report an error during conversion.
     * Serenity does not return an error and leaves the response entirely up to the Discord API.
     */
    #[error("The number of fields exceeds the maximum of 25.")]
    TooManyFields,
}

impl SerenityEmbed {
    /// Convert the embedded structure created in Builder into a model usable in Serenity.
    ///
    /// ```rs
    /// let embed = SerenityEmbed::builder()
    ///    .title("This is a test title.")
    ///    /// ... other fields ...
    ///   .build();
    ///
    /// let serenity_embed = embed.convert()?; // Result<CreateEmbed, SerenityEmbedConvertError>
    /// ```
    ///
    /// # How to use
    ///
    /// ```rs
    /// // 1. Create a SerenityEmbed using the builder
    /// let embed = SerenityEmbed::builder()
    ///   .title("This is a test title.")
    ///   .description("This is a test description.")
    ///   .build(); // Don't forget!: If you forget this, you won't be able to use `convert()`.
    ///
    /// // 2. Convert to Serenity's CreateEmbed
    /// let serenity_embed = embed.convert()?; // Result<CreateEmbed, SerenityEmbedConvertError>
    ///
    /// // 3. Use the converted embed in your Serenity message
    /// let message = serenity::builder::CreateMessage::default()
    ///  .content("Here is an embed!")
    ///  // ... other message fields ...
    /// ```
    ///
    /// # Errors
    ///
    /// This function may return the following error:
    ///
    /// - [`SerenityEmbedConvertError::TooLongDescription`]: The description exceeds the maximum length of 4096 characters.
    /// - [`SerenityEmbedConvertError::TooManyFields`]: The number of fields exceeds the maximum of 25.
    pub fn convert(&self) -> Result<CreateEmbed, SerenityEmbedConvertError> {
        let mut embed = serenity::builder::CreateEmbed::default();

        if let Some(title) = &self.title {
            embed = embed.title(title)
        }

        if let Some(description) = &self.description {
            if description.len() > 4096 {
                return Err(SerenityEmbedConvertError::TooLongDescription);
            }

            embed = embed.description(description);
        }

        if let Some(url) = &self.url {
            embed = embed.url(url);
        }

        if let Some(timestamp) = &self.timestamp {
            embed = embed.timestamp(timestamp);
        }

        if let Some(color) = &self.color {
            embed = embed.color(Colour(*color));
        }

        if let Some(footer_text) = &self.footer_text {
            let mut footer = serenity::builder::CreateEmbedFooter::new(footer_text);
            if let Some(icon_url) = &self.footer_icon_url {
                footer = footer.icon_url(icon_url);
            }
            embed = embed.footer(footer);
        }

        if let Some(image_url) = &self.image_url {
            embed = embed.image(image_url);
        }

        if let Some(thumbnail_url) = &self.thumbnail_url {
            embed = embed.thumbnail(thumbnail_url);
        }

        if let Some(author_name) = &self.author_name {
            let mut author = serenity::builder::CreateEmbedAuthor::new(author_name);
            if let Some(url) = &self.author_url {
                author = author.url(url);
            }
            if let Some(icon_url) = &self.author_icon_url {
                author = author.icon_url(icon_url);
            }
            embed = embed.author(author);
        }

        if let Some(fields) = &self.fields {
            if fields.len() > 25 {
                return Err(SerenityEmbedConvertError::TooManyFields);
            }
            // Explicitly create and pass (String, String, bool) to avoid ambiguity in `Into<String>` (inference failure due to multiple impls).
            let mapped = fields
                .iter()
                .map(|f| (f.name.clone(), f.value.clone(), f.inline));
            embed = embed.fields(mapped)
        }

        Ok(embed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::embed::SerenityEmbedField;
    use serenity::all::Timestamp;

    static MOCK_TEXT: &str = "This is a test text.";
    static MOCK_URL: &str = "https://example.com";
    static MOCK_TIMESTAMP_STR: &str = "2024-01-01T00:00:00Z";
    static MOCK_COLOR: u32 = 0xff0000;

    #[test]
    fn test_embed_conversion() {
        let fields = vec![
            SerenityEmbedField::builder()
                .name(MOCK_TEXT)
                .value(MOCK_TEXT)
                .inline(true)
                .build(),
            SerenityEmbedField::builder()
                .name(MOCK_TEXT)
                .value(MOCK_TEXT)
                .build(),
        ];

        // serenity-builder
        let mock_embed = SerenityEmbed::builder()
            .title(MOCK_TEXT)
            .description(MOCK_TEXT)
            .url(MOCK_URL)
            .timestamp(Timestamp::parse(MOCK_TIMESTAMP_STR).unwrap())
            .color(MOCK_COLOR)
            .footer_text(MOCK_TEXT)
            .footer_icon_url(MOCK_URL)
            .image_url(MOCK_URL)
            .thumbnail_url(MOCK_URL)
            .author_name(MOCK_TEXT)
            .author_url(MOCK_URL)
            .author_icon_url(MOCK_URL)
            .fields(fields)
            .build();
        // serenity
        let serenity_embed = CreateEmbed::default()
            .title(MOCK_TEXT)
            .url(MOCK_URL)
            .description(MOCK_TEXT)
            .timestamp(Timestamp::parse(MOCK_TIMESTAMP_STR).unwrap())
            .color(Colour(MOCK_COLOR))
            .footer(serenity::builder::CreateEmbedFooter::new(MOCK_TEXT).icon_url(MOCK_URL))
            .image(MOCK_URL)
            .thumbnail(MOCK_URL)
            .author(
                serenity::builder::CreateEmbedAuthor::new(MOCK_TEXT)
                    .url(MOCK_URL)
                    .icon_url(MOCK_URL),
            )
            .fields(vec![
                (MOCK_TEXT.to_string(), MOCK_TEXT.to_string(), true),
                (MOCK_TEXT.to_string(), MOCK_TEXT.to_string(), false),
            ]);

        let converted = mock_embed.convert();

        assert!(converted.is_ok());
        assert_eq!(converted.unwrap(), serenity_embed);
    }
}
