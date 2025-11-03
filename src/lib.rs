//! [![CI](https://github.com/m1sk9/serenity-builder/actions/workflows/ci.yaml/badge.svg)](https://github.com/m1sk9/serenity-builder/actions/workflows/ci.yaml)
//! [![Release serenity-builder](https://github.com/m1sk9/serenity-builder/actions/workflows/release.yaml/badge.svg)](https://github.com/m1sk9/serenity-builder/actions/workflows/release.yaml)
//! [![Apache License 2.0](https://img.shields.io/github/license/m1sk9/serenity-builder?color=%239944ee)](https://github.com/m1sk9/serenity-builder/blob/main/LICENSE)
//!
//! A utility library to make Serenity's builder easier to use
//!
//! ```toml
//! [dependencies]
//! serenity-builder = "0.2"
//! ```
//!
//! # Overview
//!
//! Use Serenity's builder with typed_builder for intuitive handling.
//!
//! ```rs
//! // serenity
//! let embed = CreateEmbed::default()
//!    .title("This is a test title.")
//!    .description("aaaaaaaaa!")
//!    .author(
//!        serenity::builder::CreateEmbedAuthor::new(MOCK_TEXT)
//!            .url(MOCK_URL)
//!            .icon_url(MOCK_URL),
//!    )
//!
//! // serenity-builder
//! let embed = SerenityEmbed::builder()
//!    .title("This is a test title.")
//!    .description("aaaaaaaaa!")
//!    .author_name("m1sk9")
//!    .author_url("https://m1sk9.dev/avatar.png")
//!    .build();
//! ```
//!
//! # Features
//!
//! Basic builders (`embed`, `message`) are default features. You can use them immediately by entering the version and crate name with `cargo add` or in Cargo.toml.
//!
//! If you only want to use specific features, use the `--features` flag with `cargo add`:
//!
//! ```sh
//! cargo add serenity-builder --no-default-features --features embed
//! ```
//!
//!
//! # License
//!
//! This project is licensed under the Apache License 2.0.

#![deny(clippy::all)]
#![allow(dead_code)]

#[cfg(feature = "embed")]
pub mod embed;
#[cfg(feature = "message")]
pub mod message;

pub mod model;
