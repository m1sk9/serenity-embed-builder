# serenity-builder

[![CI](https://github.com/m1sk9/serenity-builder/actions/workflows/ci.yaml/badge.svg)](https://github.com/m1sk9/serenity-builder/actions/workflows/ci.yaml)
[![Release serenity-builder](https://github.com/m1sk9/serenity-builder/actions/workflows/release.yaml/badge.svg)](https://github.com/m1sk9/serenity-builder/actions/workflows/release.yaml)
[![Apache License 2.0](https://img.shields.io/github/license/m1sk9/serenity-builder?color=%239944ee)](https://github.com/m1sk9/serenity-builder/blob/main/LICENSE)

A utility library to make Serenity's builder easier to use.

- [Docs](https://docs.rs/serenity-builder)

```toml
[dependencies]
serenity-builder = "0.3"
```

# Overview

Use Serenity's builder with `typed_builder` for intuitive handling.

```rs
// serenity
let embed = CreateEmbed::default()
   .title("This is a test title.")
   .description("This is test description!")
   .author(
       serenity::builder::CreateEmbedAuthor::new(MOCK_TEXT)
           .url(MOCK_URL)
           .icon_url(MOCK_URL),
   );

// serenity-builder
let embed = SerenityEmbed::builder()
   .title("This is a test title.")
   .description("This is test description!")
   .author_name("m1sk9")
   .author_url("https://m1sk9.dev/avatar.png")
   .build();
```

# Features

| Feature | Description | Default Feature? |
| ------- | ----------- | ----------------- |
| `embed` | Enable embed builder | Yes |
| `message` | Enable message builder | Yes |

Basic builders (`embed`, `message`) are default features. You can use them immediately by adding the crate with `cargo add` or adding it to your `Cargo.toml`.

If you only want to enable specific features, install with `--no-default-features` and `--features`:

```sh
cargo add serenity-builder --no-default-features --features embed
```

# Roadmap

| # | Step | Status |
|---:|------|:------:|
| 1 | Embed Builder | ✅ |
| 2 | Message Builder | ⚠️ |
| 3 | Model Builder | ❌ |
| 4 | Button Builder | ❌ |
| 5 | Invite Builder | ❌ |
| 6 | Thread Builder | ❌ |
| 7 | Channel Builder | ❌ |
| 8 | Sticker Builder | ❌ |
| 9 | Command Builder | ❌ |
| 10 | Webhook Builder | ❌ |
| 11 | ForumTag Builder | ❌ |
| 12 | ActionRow Builder | ❌ |
| 13 | ForumPost Builder | ❌ |
| 14 | Attachments Builder | ❌ |
| 15 | SelectMenu Builder | ❌ |
| 16 | Event Builder | ❌ |

...and more!

# License

This project is licensed under the [Apache License 2.0](./LICENSE).
