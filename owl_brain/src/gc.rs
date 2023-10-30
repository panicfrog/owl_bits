use dashmap::DashMap;
use crate::data::{ Data, DataRef };
use std::{sync::{Arc, Weak}, collections::HashSet};
use rayon::prelude::*;

pub struct GarbageCollector {
    data: DashMap<*const Data, Weak<Data>>,
}

impl GarbageCollector {
    pub fn new() -> Self {
        GarbageCollector {
            data: DashMap::new(),
        }
    }

    pub fn track(&self, data_ref: &DataRef) {
        let ptr = &**data_ref as *const Data;
        let weak_ref = Arc::downgrade(data_ref);
        self.data.insert(ptr, weak_ref);
    }

    pub fn collect(&self) {
        let mut marked = HashSet::new();
        
        // TODO: 使用rayon并行收集
        self.data.iter().for_each(|entry| {
            if let Some(data_ref) = entry.value().upgrade() {
                data_ref.mark(&mut marked);
            }
        });
        
        self.data.retain(|&ptr, weak_ref| {
            weak_ref.upgrade().map_or(false, |_| {
                marked.contains(&(ptr as *const Data))
            })
        });
    }
}
