//! Brotli for chunks of about 1 KB, each alone, with the built-in dictionary
//! only.
//!
//! Brotli's encoder at quality 9 matches with a hasher whose tables are
//! 64 KB of counts and 32 MB of positions. A new encoder per chunk got them
//! from the kernel every time, zeroed, and gave them back: about 90% of the
//! simulator's CPU, and the sidecar's on every packet (issue #63). The
//! encoder here takes its large blocks from a per-thread pool instead.
//!
//! A reused block is reset to zero, except the `u32` position table: the
//! hasher reads a position only where its counts say it wrote one, as C
//! Brotli does with memory from `malloc`. So the output is the same as with
//! fresh tables (`pooled_output_is_the_same_as_fresh`), at a tenth of the
//! cost.

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;

use brotli::enc::{Allocator, BrotliAlloc, BrotliEncoderParams, SliceWrapper, SliceWrapperMut};

/// Quality 9 output is about 1% smaller than quality 5 on 1 KB of English
/// text; quality 10 and 11 are smaller again but cost ten times the CPU.
const QUALITY: i32 = 9;
const WINDOW: i32 = 16;
/// Blocks this large or larger are pooled; smaller ones come from the heap.
const POOLED_BYTES: usize = 64 << 10;

/// Free large blocks by element type and length.
type Free = HashMap<(TypeId, usize), Vec<Box<dyn Any>>>;

thread_local! {
    static POOL: RefCell<Free> = RefCell::new(HashMap::new());
}

/// A block of the encoder's memory.
pub struct Block<T>(Box<[T]>);

impl<T> Default for Block<T> {
    fn default() -> Self {
        Block(Box::new([]))
    }
}

impl<T> SliceWrapper<T> for Block<T> {
    fn slice(&self) -> &[T] {
        &self.0
    }
}

impl<T> SliceWrapperMut<T> for Block<T> {
    fn slice_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

/// The encoder's allocator: large blocks go back to the thread's pool.
#[derive(Default)]
pub struct Pool;

fn pooled<T>(len: usize) -> bool {
    len * size_of::<T>() >= POOLED_BYTES
}

impl<T: Clone + Default + 'static> Allocator<T> for Pool {
    type AllocatedMemory = Block<T>;

    fn alloc_cell(&mut self, len: usize) -> Block<T> {
        if pooled::<T>(len) {
            let free = POOL.with(|p| {
                p.borrow_mut()
                    .get_mut(&(TypeId::of::<T>(), len))
                    .and_then(Vec::pop)
            });
            if let Some(Ok(mut block)) = free.map(|b| b.downcast::<Box<[T]>>()) {
                if TypeId::of::<T>() != TypeId::of::<u32>() {
                    block.fill(T::default());
                }
                return Block(*block);
            }
        }
        Block(vec![T::default(); len].into_boxed_slice())
    }

    fn free_cell(&mut self, block: Block<T>) {
        if pooled::<T>(block.0.len()) {
            POOL.with(|p| {
                p.borrow_mut()
                    .entry((TypeId::of::<T>(), block.0.len()))
                    .or_default()
                    .push(Box::new(block.0));
            });
        }
    }
}

impl BrotliAlloc for Pool {}

/// Compresses `raw` with Brotli's built-in dictionary only. None when that
/// does not make it smaller.
pub fn compress(raw: &[u8]) -> Option<Vec<u8>> {
    compress_with(raw, Pool)
}

fn compress_with<A: BrotliAlloc>(raw: &[u8], alloc: A) -> Option<Vec<u8>> {
    let params = BrotliEncoderParams {
        quality: QUALITY,
        lgwin: WINDOW,
        size_hint: raw.len(),
        ..Default::default()
    };
    let mut out = Vec::with_capacity(raw.len());
    let (mut input, mut output) = ([0; 4096], [0; 4096]);
    brotli::BrotliCompressCustomAlloc(
        &mut &raw[..],
        &mut out,
        &mut input,
        &mut output,
        &params,
        alloc,
    )
    .ok()?;
    (out.len() < raw.len()).then_some(out)
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;
    use crate::sim::Rng;

    /// Chunks like the outbox makes: text, JSON and bytes that do not
    /// compress, of many lengths.
    fn chunks() -> Vec<Vec<u8>> {
        let text = "The sync loop starts from what the phone last acknowledged. \
            When the link comes back after an outage, the server does not replay every event. "
            .repeat(20);
        let json = r#"{"id":"turn-item-7","threadId":"t1","type":"assistant_message","ordinal":2000002,"status":"running"},"#
            .repeat(20);
        let mut rng = Rng::new(7);
        let noise: Vec<u8> = (0..2_000).map(|_| rng.next_u64() as u8).collect();
        let mut out = Vec::new();
        for source in [text.as_bytes(), json.as_bytes(), &noise] {
            for len in [48, 100, 300, 700, 1_000, 1_100] {
                for start in [0, 13, 250] {
                    out.push(source[start..start + len].to_vec());
                }
            }
        }
        out
    }

    #[test]
    fn pooled_output_is_the_same_as_fresh() {
        for raw in chunks() {
            let fresh = compress_with(&raw, brotli::enc::StandardAlloc::default());
            assert_eq!(compress(&raw), fresh, "{} bytes", raw.len());
        }
    }

    #[test]
    fn every_chunk_decodes_alone() {
        for raw in chunks() {
            let Some(packed) = compress(&raw) else {
                continue;
            };
            let mut back = Vec::new();
            let decoded = brotli::Decompressor::new(&packed[..], 4096).read_to_end(&mut back);
            assert!(decoded.is_ok(), "{} bytes", raw.len());
            assert_eq!(back, raw);
        }
    }
}
