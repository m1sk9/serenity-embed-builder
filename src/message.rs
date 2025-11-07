use crate::model::message::{SerenityMessage, SerenityMessageMentionType};
use serenity::all::CreateMessage;
use serenity::builder::CreateAllowedMentions as Am;

/// Errors that can occur when converting a custom message struct to a [serenity::all::CreateMessage].
#[derive(thiserror::Error, Debug)]
pub enum SerenityMessageConvertError {
    /**
     * This occurs when the message content exceeds 2000 characters, which is a limitation imposed by the Discord API.
     *
     * Serenity-builder will report an error during conversion.
     * Serenity does not return an error and leaves the response entirely up to the Discord API.
     */
    #[error("The content exceeds the maximum length of 2000 characters.")]
    TooLongContent,
    /**
     * This occurs when there is an error converting an embedded structure.
     * The specific error details are encapsulated in the [crate::embed::SerenityEmbedConvertError].
     */
    #[error(transparent)]
    EmbedConvertError(#[from] crate::embed::SerenityEmbedConvertError),
}

impl SerenityMessage {
    /// Convert the message structure created in Builder into a model usable in Serenity.
    ///
    /// ```rs
    /// let message = SerenityMessage::builder()
    ///   .content("This is a test message.")
    ///   .build();
    ///
    /// let serenity_message = message.convert()?; // Result<CreateMessage, SerenityMessageConvertError>
    /// ```
    ///
    /// # How to use
    ///
    /// ```rs
    /// // 1. Create a SerenityMessage using the builder
    /// let message = SerenityMessage::builder()
    ///   .content("This is a test message.")
    ///   .build(); // Don't forget!: If you forget this, you won't be able to use `convert()`.
    ///
    /// // 2. Convert to Serenity's CreateMessage
    /// let serenity_message = message.convert()?; // Result<CreateMessage, SerenityMessageConvertError>
    ///
    /// // 3. Use the converted message in your bot
    /// if let Err(e) = message.channel_id.send_message(&ctx.http, serenity_message).await {
    ///     tracing::error!("Failed to send preview: {:?}", e);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function may return the following error:
    ///
    /// - [SerenityMessageConvertError::TooLongContent]: The content exceeds the maximum length of 2000 characters.
    /// - [SerenityMessageConvertError::EmbedConvertError]: Failed to perform internal conversion for embed. (error [crate::embed::SerenityEmbedConvertError] reported by thiserror)
    pub fn convert(&self) -> Result<CreateMessage, SerenityMessageConvertError> {
        let mut message = serenity::builder::CreateMessage::default();

        if let Some(content) = &self.content {
            // Internal string data in the Discord API is handled in UTF-16 code units.
            if content.encode_utf16().count() > 2000 {
                return Err(SerenityMessageConvertError::TooLongContent);
            }
            message = message.content(content);
        }

        if let Some(embeds) = &self.embeds {
            for embed in embeds {
                let serenity_embed = embed.convert()?;
                message = message.add_embed(serenity_embed);
            }
        }

        if let Some(mention) = &self.mention_type {
            match mention {
                SerenityMessageMentionType::Everyone => {
                    message = message.allowed_mentions(Am::new().everyone(true));
                }
                SerenityMessageMentionType::Here => {
                    message = message.allowed_mentions(Am::new().all_users(true).all_roles(true));
                }
                SerenityMessageMentionType::Users(user_ids) => {
                    message = message.allowed_mentions(Am::new().users(user_ids.clone()));
                }
                SerenityMessageMentionType::Roles(role_ids) => {
                    message =
                        message.allowed_mentions(Am::new().all_users(true).roles(role_ids.clone()));
                }
                SerenityMessageMentionType::Reply(ref_msg) => {
                    message = message.reference_message(&**ref_msg);
                    message = message.allowed_mentions(Am::new().replied_user(true));
                }
            }
        }

        if let Some(sticker_ids) = &self.sticker_ids {
            message = message.sticker_ids(sticker_ids);
        }

        message = message.tts(self.tts);
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    // TODO: Comparison Test with Serenity's CreateMessage.

    use serenity::all::StickerId;

    use super::*;
    use crate::model::embed::SerenityEmbed;

    static MOCK_TEST: &str = "This is a test message.";
    static MOCK_STICKER_ID: u64 = 123456789012345678;

    #[test]
    fn test_message_conversion() {
        let embed = SerenityEmbed::builder()
            .title("Test Embed")
            .description("This is a test embed description.")
            .build();

        // serenity-builder
        let mock_message = SerenityMessage::builder()
            .content(MOCK_TEST)
            .embeds(vec![embed.clone()])
            .mention_type(SerenityMessageMentionType::Everyone)
            .sticker_ids(vec![StickerId::new(MOCK_STICKER_ID)])
            .build();

        let converted = mock_message.convert();
        assert!(converted.is_ok());
    }
}
