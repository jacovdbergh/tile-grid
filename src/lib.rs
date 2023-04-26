//! A library for using OGC TileMatrixSets (TMS).

mod common;
mod errors;
mod quadkey;
mod registry;
mod tile;
mod tileset;
mod tms;
mod transform;
mod wmts;

pub use common::*;
pub use quadkey::*;
pub use registry::*;
pub use tile::*;
pub use tileset::*;
pub use tms::*;
pub use wmts::*;

use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;

/// A 2DPoint in the CRS indicated elsewere
type Point2D = [f64; 2];

/// Ordered list of names of the dimensions defined in the CRS
type OrderedAxes = [String; 2];

#[serde_with::serde_as]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TitleDescriptionKeywords {
    /// Title of this resource entity, normally used for display to a human
    pub title: Option<String>,
    /// Brief narrative description of this resoource entity, normally available
    /// for display to a human
    pub description: Option<String>,
    /// Unordered list of one or more commonly used or formalized word(s) or
    /// phrase(s) used to describe this resource entity
    pub keywords: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Query {
    pub collections: Option<String>,
}

/// Minimum bounding rectangle surrounding a 2D resource in the CRS indicated elsewere
#[serde_with::serde_as]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoundingBox2D {
    pub lower_left: Point2D,
    pub upper_right: Point2D,
    #[serde(default)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub crs: Option<Crs>,
    pub orderd_axes: Option<OrderedAxes>,
}
