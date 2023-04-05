//! Pagination response module

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct PaginateResponse<T: Debug + Serialize + PartialEq + Eq> {
    pub data: T,
    pub total: i64,
}
