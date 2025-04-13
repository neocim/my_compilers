//! ### FIXME!
//! I dont know where to put this module. On the one hand, it's probably an
//! independent crate, and it shouldn't be in the `symbol`. On the other hand, I don't
//! use it anywhere else except in the `symbol`, so I left this module here.
//!
//! ### About
//! This `FxHash` is taken from `rustc`. I could use ready-made solutions, but it
//! was interesting for me to implement it manually. This is not a cryptographically secure
//! hash and also based on [`MurmurHash`](https://ru.wikipedia.org/wiki/MurmurHash).
#![allow(dead_code)]
use std::hash::{BuildHasherDefault, Hasher};

#[cfg(target_pointer_width = "32")]
const SEED: usize = 0x9e3779b9;
#[cfg(target_pointer_width = "64")]
const SEED: usize = 0x517cc1b727220a95;

pub type FxIndexSet<T> = indexmap::IndexSet<T, BuildHasherDefault<FxHasher>>;

pub struct FxHasher {
    hash: usize,
}

impl FxHasher {
    fn hash(&mut self, word: usize) {
        self.hash = self.hash.wrapping_add(word).wrapping_mul(SEED);
    }
}

impl Default for FxHasher {
    fn default() -> FxHasher {
        FxHasher { hash: 0 }
    }
}

impl Hasher for FxHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.hash = write(self.hash, bytes);
    }

    fn finish(&self) -> u64 {
        self.hash as u64
    }

    fn write_u8(&mut self, i: u8) {
        self.hash(i as usize);
    }

    fn write_u16(&mut self, i: u16) {
        self.hash(i as usize);
    }

    fn write_u32(&mut self, i: u32) {
        self.hash(i as usize);
    }

    #[cfg(target_pointer_width = "32")]
    fn write_u64(&mut self, i: u64) {
        self.hash(i as usize);
        self.hash((i >> 32) as usize);
    }

    #[cfg(target_pointer_width = "64")]
    fn write_u64(&mut self, i: u64) {
        self.hash(i as usize);
    }

    fn write_usize(&mut self, i: usize) {
        self.hash = self.hash.wrapping_add(i);
    }
}

#[cfg(target_pointer_width = "32")]
fn write(hash: usize, bytes: &[u8]) -> usize {
    write32(hash as u32, bytes) as usize
}

#[cfg(target_pointer_width = "64")]
fn write(hash: usize, bytes: &[u8]) -> usize {
    write64(hash as u64, bytes) as usize
}

fn write32(mut hash: u32, mut bytes: &[u8]) -> u32 {
    while bytes.len() >= 4 {
        hash = hash.wrapping_add(u32::from_le_bytes(bytes[0..4].try_into().unwrap()));
        bytes = &bytes[4..];
    }

    if bytes.len() >= 2 {
        hash = hash.wrapping_add(u16::from_le_bytes(bytes[0..2].try_into().unwrap()) as u32);
        bytes = &bytes[2..];
    }

    if let Some(&byte) = bytes.first() {
        hash = hash.wrapping_add(u32::from(byte));
    }

    hash
}

fn write64(mut hash: u64, mut bytes: &[u8]) -> u64 {
    while bytes.len() >= 8 {
        hash = hash.wrapping_add(u64::from_le_bytes(bytes[0..8].try_into().unwrap()));
        bytes = &bytes[8..];
    }

    if bytes.len() >= 4 {
        hash = hash.wrapping_add(u32::from_le_bytes(bytes[0..4].try_into().unwrap()) as u64);
        bytes = &bytes[4..];
    }

    if bytes.len() >= 2 {
        hash = hash.wrapping_add(u16::from_le_bytes(bytes[0..2].try_into().unwrap()) as u64);
        bytes = &bytes[2..];
    }

    if let Some(&byte) = bytes.first() {
        hash = hash.wrapping_add(u64::from(byte));
    }

    hash
}
