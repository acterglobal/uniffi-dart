use uniffi;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

static LIVE_COUNT: Lazy<RwLock<i32>> = Lazy::new(|| RwLock::new(0));

#[derive(Debug, Clone, uniffi::Object)]
pub struct Resource {}

impl Resource {
    #[uniffi::constructor]
    pub fn new() -> Self {
        *LIVE_COUNT.write().unwrap() += 1;
        Resource {}
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        *LIVE_COUNT.write().unwrap() -= 1;
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct ResourceJournalList {
    resources: Vec<Arc<Resource>>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct ResourceJournalMap {
    resources: HashMap<i32, Arc<Resource>>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct ResourceJournalMapList {
    resources: HashMap<i32, Option<Vec<Arc<Resource>>>>,
}

#[derive(Debug, Clone, uniffi::Enum)]
pub enum MaybeResourceJournal {
    Some { resource: ResourceJournalList },
    None,
}

#[uniffi::export]
fn get_live_count() -> i32 {
    *LIVE_COUNT.read().unwrap()
}

#[uniffi::export]
fn get_resource() -> Arc<Resource> {
    Arc::new(Resource::new())
}

#[uniffi::export]
fn get_resource_journal_list() -> ResourceJournalList {
    ResourceJournalList {
        resources: vec![get_resource(), get_resource()],
    }
}

#[uniffi::export]
fn get_resource_journal_map() -> ResourceJournalMap {
    ResourceJournalMap {
        resources: HashMap::from([(1, get_resource()), (2, get_resource())]),
    }
}

#[uniffi::export]
fn get_resource_journal_map_list() -> ResourceJournalMapList {
    ResourceJournalMapList {
        resources: HashMap::from([(1, Some(vec![get_resource(), get_resource()])), (2, None)]),
    }
}

#[uniffi::export]
fn get_maybe_resource_journal() -> MaybeResourceJournal {
    MaybeResourceJournal::Some {
        resource: get_resource_journal_list(),
    }
}

uniffi::include_scaffolding!("api");
