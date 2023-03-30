//! Pagination response module

use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct PaginateResponse<T: Debug + Serialize + PartialEq + Eq> {
    pub data: T,
    pub total: i64,
}
