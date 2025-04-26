// SPDX-FileCopyrightText: 2024 John Irle
// SPDX-License-Identifier: MIT
//
// This file is part of rlox-bytecode

use crate::grow_capacity;

pub type Value = f64;

#[derive(Default)]
pub struct ValueArray {
    capacity: i32,
    pub(crate) count: i32,
    pub(crate) values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self {
            capacity: 0,
            count: 0,
            values: Vec::new(),
        }
    }

    pub fn write_value(&mut self, value: Value) {
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = grow_capacity(old_capacity);
            self.values.insert(self.count as usize, value);
            self.count += 1;
        }
    }
}
