//! A module containing information related to enums used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/airpope-mango/issues/new/choose) or a [pull request](https://github.com/noaione/airpope-mango/compare).

use serde::{Deserialize, Serialize};
use airpope_macros::{DeserializeEnum32, EnumName, SerializeEnum32};

/// A status of a comic.
#[derive(Debug, Clone, SerializeEnum32, DeserializeEnum32, PartialEq, EnumName)]
pub enum ComicStatus {
    /// The comic is completed.
    Complete = 1,
    /// The comic is ongoing.
    Ongoing = 2,
    /// The comic is suspended/hiatus.
    Hiatus = 3,
}
