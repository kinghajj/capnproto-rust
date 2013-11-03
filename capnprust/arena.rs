/*
 * Copyright (c) 2013, David Renshaw (dwrenshaw@gmail.com)
 *
 * See the LICENSE file in the capnproto-rust root directory.
 */

use std;
use common::*;
use message;

pub type SegmentId = u32;

pub struct SegmentReader<'self> {
    messageReader : &'self message::MessageReader<'self>,
    segment : &'self [u8]
}


pub struct SegmentBuilder {
    messageBuilder : *mut message::MessageBuilder,
    id : SegmentId,
    pos : WordCount,
    size : WordCount
}

impl SegmentBuilder {

    pub fn new(messageBuilder : *mut message::MessageBuilder,
               size : WordCount) -> SegmentBuilder {
        SegmentBuilder {
            messageBuilder : messageBuilder,
            id : unsafe {(*messageBuilder).segments.len() as SegmentId},
            pos : 0,
            size : size
        }
    }

    pub fn getWordOffsetTo(&mut self, ptr : *mut u8) -> WordCount {
        let thisAddr : uint =
            unsafe { std::cast::transmute(
                (*self.messageBuilder).segments[self.id].unsafe_mut_ref(0)) };
        let ptrAddr : uint = unsafe {std::cast::transmute(ptr)};
        assert!(ptrAddr >= thisAddr);
        let result = (ptrAddr - thisAddr) / BYTES_PER_WORD;
        return result;
    }

    pub fn getByteOffsetTo(&mut self, ptr : *mut u8) -> ByteCount {
        let thisAddr : uint =
            unsafe { std::cast::transmute(
                (*self.messageBuilder).segments[self.id].unsafe_mut_ref(0)) };
        let ptrAddr : uint = unsafe {std::cast::transmute(ptr)};
        assert!(ptrAddr >= thisAddr);
        return (ptrAddr - thisAddr);
    }


    pub fn allocate(&mut self, amount : WordCount) -> Option<*mut u8> {
        if (amount > self.size - self.pos) {
            return None;
        } else {
            let result = unsafe {
                (*self.messageBuilder).segments[self.id].unsafe_mut_ref(self.pos * BYTES_PER_WORD)
            };
            self.pos += amount;
            return Some(result);
        }
    }

    pub fn available(&self) -> WordCount {
        self.size - self.pos
    }

    pub fn withMutSegment<T>(&mut self, f : &fn(&mut [u8]) -> T) -> T {
        unsafe {f((*self.messageBuilder).segments[self.id])}
    }

    #[inline]
    pub fn memset(&mut self, ptr: uint, c: u8, count: uint) {
        do self.withMutSegment |segment| {
            unsafe {
                let p = segment.unsafe_mut_ref(ptr);
                std::ptr::set_memory(p, c, count)
            }
        }
    }

    pub fn asReader<T>(&mut self, f : &fn(SegmentReader) -> T) -> T {
        unsafe {
            do (*self.messageBuilder).asReader |messageReader| {
                f(SegmentReader {
                        messageReader : &messageReader,
                        segment : messageReader.segments[self.id]
                    })
            }
        }
    }

}