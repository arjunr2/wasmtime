//! Wasmtime's Record and Replay support

use crate::ValRaw;
use crate::prelude::*;
use core::fmt;
use core::mem::{self, MaybeUninit};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

const VAL_RAW_SIZE: usize = mem::size_of::<ValRaw>();

// Since unions cannot be serialized, we encode them as transmutable byte arrays
//type ValRawSer = [u8; VAL_RAW_SIZE];
#[derive(Serialize, Deserialize)]
struct ValRawSer([u8; VAL_RAW_SIZE]);

impl From<ValRaw> for ValRawSer {
    fn from(value: ValRaw) -> Self {
        unsafe { Self(mem::transmute(value)) }
    }
}

impl fmt::Debug for ValRawSer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex_digits_per_byte = 2;
        let _ = write!(f, "0x..");
        for b in self.0.iter().rev() {
            let _ = write!(f, "{:0width$x}", b, width = hex_digits_per_byte);
        }
        Ok(())
    }
}

/// A single recording/replay event
#[derive(Debug, Serialize, Deserialize)]
pub enum RREvent {
    ExternCall(Vec<ValRawSer>),
    ExternReturn(Vec<ValRawSer>),
}

impl RREvent {
    fn raw_to_vec(args: &[MaybeUninit<ValRaw>]) -> Vec<ValRawSer> {
        args.iter()
            .map(|x| unsafe { ValRawSer::from(x.assume_init()) })
            .collect::<Vec<_>>()
    }

    pub fn extern_call_from_valraw_slice(args: &[MaybeUninit<ValRaw>]) -> Self {
        Self::ExternCall(Self::raw_to_vec(args))
    }
    pub fn extern_return_from_valraw_slice(args: &[MaybeUninit<ValRaw>]) -> Self {
        Self::ExternReturn(Self::raw_to_vec(args))
    }
}

/// Buffer to read/write record/replay data respectively
pub struct RRBuffer {
    inner: VecDeque<RREvent>,
}

impl RRBuffer {
    /// Constructs an new (empty) buffer
    pub fn new() -> Self {
        RRBuffer {
            inner: VecDeque::new(),
        }
    }

    /// Appends a new [`RREvent`] to the buffer
    pub fn append(&mut self, event: RREvent) {
        self.inner.push_back(event)
    }

    pub fn pop_front(&mut self) -> RREvent {
        self.inner
            .pop_front()
            .expect("Replay event buffer is empty")
    }

    /// Flush all the contents of the entire buffer to a writer
    ///
    /// Buffer is emptied during this process
    fn to_file<W>(writer: W)
    where
        W: std::io::Write,
    {
    }

    /// Read the
    fn from_file<R>(reader: R)
    where
        R: std::io::Read,
    {
    }
}
