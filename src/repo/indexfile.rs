use std::cmp::Ordering;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::backend::{FileType, RepoFile};
use crate::blob::BlobType;
use crate::id::Id;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IndexFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) supersedes: Option<Vec<Id>>,
    pub(crate) packs: Vec<IndexPack>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) packs_to_delete: Vec<IndexPack>,
}

impl RepoFile for IndexFile {
    const TYPE: FileType = FileType::Index;
}

impl IndexFile {
    pub fn add(&mut self, p: IndexPack) {
        self.packs.push(p);
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct IndexPack {
    pub(crate) id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) time: Option<DateTime<Local>>,
    pub(crate) blobs: Vec<IndexBlob>,
}

impl IndexPack {
    pub fn set_id(&mut self, id: Id) {
        self.id = id;
    }

    pub fn add(&mut self, id: Id, tpe: BlobType, offset: u32, length: u32) {
        self.blobs.push(IndexBlob {
            id,
            tpe,
            offset,
            length,
        });
    }

    // calculate the pack size from the contained blobs
    pub fn pack_size(&self) -> u32 {
        let mut size = 4 + 32; // 4 + crypto overhead
        for blob in &self.blobs {
            size += blob.length + 37 // 37 = length of blob description
        }
        size
    }

    /// returns the blob type of the pack. Note that only packs with
    /// identical blob types are allowed
    pub fn blob_type(&self) -> BlobType {
        self.blobs[0].tpe
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct IndexBlob {
    pub(crate) id: Id,
    #[serde(rename = "type")]
    pub(crate) tpe: BlobType,
    pub(crate) offset: u32,
    pub(crate) length: u32,
}

impl PartialOrd<IndexBlob> for IndexBlob {
    fn partial_cmp(&self, other: &IndexBlob) -> Option<Ordering> {
        self.offset.partial_cmp(&other.offset)
    }
}

impl Ord for IndexBlob {
    fn cmp(&self, other: &Self) -> Ordering {
        self.offset.cmp(&other.offset)
    }
}