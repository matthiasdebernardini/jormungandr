use crate::fragment::FragmentId;
use jormungandr_lib::{
    crypto::hash::Hash,
    interfaces::{FragmentLog, FragmentOrigin, FragmentStatus},
};
use lru::LruCache;
use std::collections::HashMap;

pub struct Logs {
    entries: LruCache<Hash, FragmentLog>,
}

impl Logs {
    pub fn new(max_entries: usize) -> Self {
        Logs {
            entries: LruCache::new(max_entries),
        }
    }

    pub fn exists(&self, fragment_id: FragmentId) -> bool {
        let fragment_id: Hash = fragment_id.into();
        self.entries.contains(&fragment_id)
    }

    pub fn exist_all(&self, fragment_ids: impl IntoIterator<Item = FragmentId>) -> Vec<bool> {
        fragment_ids
            .into_iter()
            .map(|fragment_id| self.exists(fragment_id))
            .collect()
    }

    /// Returns true if fragment was registered
    pub fn insert(&mut self, log: FragmentLog) -> bool {
        let fragment_id = *log.fragment_id();
        if self.entries.contains(&fragment_id) {
            false
        } else {
            self.entries.put(fragment_id, log);
            true
        }
    }

    /// Returns number of registered fragments
    pub fn insert_all(&mut self, logs: impl IntoIterator<Item = FragmentLog>) -> usize {
        logs.into_iter()
            .map(|log| self.insert(log))
            .filter(|was_modified| *was_modified)
            .count()
    }

    pub fn modify(&mut self, fragment_id: FragmentId, status: FragmentStatus) {
        let fragment_id: Hash = fragment_id.into();
        match self.entries.get_mut(&fragment_id) {
            Some(entry) => {
                entry.modify(status);
            }
            None => {
                // while a log modification, if the log was not already present in the
                // logs it means we received it from the a new block from the network.
                // we can mark the status of the transaction so newly received transaction
                // be stored.

                self.entries.put(
                    fragment_id,
                    FragmentLog::new(fragment_id.clone().into_hash(), FragmentOrigin::Network),
                );
            }
        }
    }

    pub fn modify_all(
        &mut self,
        fragment_ids: impl IntoIterator<Item = FragmentId>,
        status: FragmentStatus,
    ) {
        for fragment_id in fragment_ids {
            self.modify(fragment_id, status.clone());
        }
    }

    pub fn logs_by_ids(
        &self,
        fragment_ids: impl IntoIterator<Item = FragmentId>,
    ) -> HashMap<FragmentId, &FragmentLog> {
        let mut result = HashMap::new();
        fragment_ids
            .into_iter()
            .filter_map(|fragment_id| {
                let key: Hash = fragment_id.clone().into();
                self.entries.peek(&key).map(|log| (fragment_id, log))
            })
            .for_each(|(k, v)| {
                result.insert(k, v);
            });
        result
    }

    pub fn logs<'a>(&'a self) -> impl Iterator<Item = &'a FragmentLog> {
        self.entries.iter().map(|(_, v)| v)
    }
}
