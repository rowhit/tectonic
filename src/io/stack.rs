// src/io/stack.rs -- a stack of other IoProviders
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use std::ffi::OsStr;

use status::StatusBackend;
use super::{InputHandle, IoProvider, OpenResult, OutputHandle};


/// An IoStack is an IoProvider that delegates to an ordered list of
/// subordinate IoProviders. It also checks the order in which files are read
/// and written to detect "circular" access patterns that indicate whether we
/// need to run multiple passes of the TeX engine.

pub struct IoStack<'a> {
    items: Vec<&'a mut IoProvider>,
}


impl<'a> IoStack<'a> {
    pub fn new(items: Vec<&'a mut IoProvider>) -> IoStack<'a> {
        IoStack {
            items: items,
        }
    }
}


impl<'a> IoProvider for IoStack<'a> {
    fn output_open_name(&mut self, name: &OsStr) -> OpenResult<OutputHandle> {
        for item in self.items.iter_mut() {
            let r = item.output_open_name(name);

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r
            };
        }

        OpenResult::NotAvailable
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        for item in self.items.iter_mut() {
            let r = item.output_open_stdout();

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r
            };
        }

        OpenResult::NotAvailable
    }

    fn input_open_name(&mut self, name: &OsStr, status: &mut StatusBackend) -> OpenResult<InputHandle> {
        for item in self.items.iter_mut() {
            let r = item.input_open_name(name, status);

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r
            };
        }

        OpenResult::NotAvailable
    }

    fn input_open_primary(&mut self, status: &mut StatusBackend) -> OpenResult<InputHandle> {
        for item in self.items.iter_mut() {
            let r = item.input_open_primary(status);

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r
            };
        }

        OpenResult::NotAvailable
    }

    fn input_open_format(&mut self, name: &OsStr, status: &mut StatusBackend) -> OpenResult<InputHandle> {
        for item in self.items.iter_mut() {
            let r = item.input_open_format(name, status);

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r
            };
        }

        OpenResult::NotAvailable
    }
}
