use crate::types::resource::PosterShape;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnNull, NoneAsEmptyString};

#[serde_as]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct LibraryItem {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub r#type: String,
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull<NoneAsEmptyString>")]
    pub poster: Option<String>,
    #[serde(default)]
    pub poster_shape: PosterShape,
    pub removed: bool,
    pub temp: bool,
    #[serde(default, rename = "_ctime")]
    #[serde_as(deserialize_as = "DefaultOnNull<NoneAsEmptyString>")]
    pub ctime: Option<DateTime<Utc>>,
    #[serde(rename = "_mtime")]
    pub mtime: DateTime<Utc>,
    pub state: LibraryItemState,
    #[serde(default)]
    pub behavior_hints: LibraryItemBehaviorHints,
}

impl LibraryItem {
    #[inline]
    pub fn should_sync(&self) -> bool {
        !self.removed || self.state.overall_time_watched > 60_000
    }
    #[inline]
    pub fn is_in_continue_watching(&self) -> bool {
        self.should_sync() && (!self.removed || self.temp) && self.state.time_offset > 0
    }
}

#[serde_as]
#[serde(rename_all = "camelCase")]
#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct LibraryItemState {
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull<NoneAsEmptyString>")]
    pub last_watched: Option<DateTime<Utc>>,
    pub time_watched: u64,
    pub time_offset: u64,
    pub overall_time_watched: u64,
    pub times_watched: u32,
    // @TODO: consider bool that can be deserialized from an integer
    pub flagged_watched: u32,
    pub duration: u64,
    #[serde(default, rename = "video_id")]
    #[serde_as(deserialize_as = "DefaultOnNull<NoneAsEmptyString>")]
    pub video_id: Option<String>,
    // @TODO bitfield, special type
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull<NoneAsEmptyString>")]
    pub watched: Option<String>,
    // release date of last observed video
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull<NoneAsEmptyString>")]
    pub last_vid_released: Option<DateTime<Utc>>,
    pub no_notif: bool,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct LibraryItemBehaviorHints {
    pub default_video_id: Option<String>,
}
