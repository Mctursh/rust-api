// use std::error::Error;

use crate::constants::{
    PAGE_SIZE,
    DEFAULT_BUFFER_POOL_SIZE
};

use crate::error::DbError;

type PoolResult<T> = Result<T, DbError>;
type PoolPageReuslt<T> = PoolResult<Option<T>>;

#[derive(Debug, Copy, Clone)]
pub struct PoolEntry {
    page_id: Option<u32>,
    data: [u8; PAGE_SIZE as usize],
    is_dirty: bool,
    pin_count: u32,
    last_accessed: u64
}

impl Default for PoolEntry {
    fn default() -> Self {
        Self {
            page_id: None,
            data: [0u8; PAGE_SIZE as usize],
            is_dirty: false,
            pin_count: 0,
            last_accessed: 0
        }
    }
}

pub struct BufferPool {
    entries: [PoolEntry; DEFAULT_BUFFER_POOL_SIZE],
    access_counter: u64, // increments on each access, used instead of timestamps.
}

impl BufferPool {
    pub fn new () -> PoolResult<BufferPool> {
        Ok(Self {
            entries: [Default::default(); DEFAULT_BUFFER_POOL_SIZE],
            access_counter: 0
        })
    }

    pub fn find_page (page_id: u32, pool: &BufferPool) -> PoolPageReuslt<&PoolEntry> {
       let page =  pool.entries.iter().find(|&entry| {
            if let Some(page_number) = entry.page_id {
                page_number == page_id
            } else {
                false
            }
        });
        //TODO: add pin logic to this function and impl an unpin upon drop
        Ok(page)
    }

    pub fn find_empty_slot () -> PoolResult<()> {
        Ok(())
    }

    pub fn evict_lru () -> PoolResult<()> {
        Ok(())
    }
}
mod pool {
    // pub fn new (size: [PoolEntry; DEFAULT_BUFFER_POOL_SIZE]) -> Result<()> {
    //     Ok(())
    // }
    pub fn get_page () {}
    pub fn pin_page () {}
    pub fn unpin_page () {}
    pub fn flush_page () {}
    pub fn flush_all () {}
}