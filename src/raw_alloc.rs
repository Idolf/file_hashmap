// Copyright 2017 Mathias Svensson. See the COPYRIGHT file at the top-level
// directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cast::i64;
use nix;
use nix::fcntl;
use nix::unistd;
use nix::sys::mman;
use nix::sys::stat::Mode;
use std::{mem, ptr};
use std::os::unix::io::RawFd;
use std::ops::Drop;
use std::sync::RwLock;
use std::path::{Path, PathBuf};
use std::env;

lazy_static! {
    static ref PATH: RwLock<PathBuf> = RwLock::new(env::home_dir().unwrap_or(".".into()));
}

#[inline]
pub fn set_path<P: ?Sized + Into<PathBuf>>(p: P) {
    let p: PathBuf = p.into();
    *PATH.write().unwrap() = p;
}

#[inline]
fn with_path<T, F>(f: F) -> T
    where F: FnOnce(&Path) -> T
{
    f(&PATH.read().unwrap())
}

#[inline]
fn with_tmp_fd<F, R>(f: F) -> nix::Result<R>
    where F: FnOnce(RawFd) -> nix::Result<R>
{
    // TODO(idolf): Remove once nix-rust/nix#501 lands.
    #[allow(non_snake_case)]
    let O_TMPFILE = fcntl::O_TMPFILE | fcntl::O_DIRECTORY;
    let fd = with_path(|path| {
        fcntl::open(path,
                    O_TMPFILE | fcntl::O_RDWR | fcntl::O_EXCL,
                    Mode::empty())
    })?;

    // Make sure that the fd is closed, even in the case of panics. This should not be needed with
    // the current code, but you never know.
    struct CloseFd(RawFd);
    impl Drop for CloseFd {
        fn drop(&mut self) {
            unistd::close(self.0).expect("Could not close file");
        }
    }
    let close_fd = CloseFd(fd);
    let res = f(fd);
    mem::drop(close_fd);
    res
}

#[inline]
pub unsafe fn allocate(size: usize, align: usize) -> *mut u8 {
    if size == 0 || align == 0 || (align - 1) & align != 0 || align > 4096 {
        return ptr::null_mut();
    }

    with_tmp_fd(|fd| {
            let signed_size = i64(size).map_err(|_| nix::Errno::ENOMEM)?;
            unistd::ftruncate(fd, signed_size)?;
            let addr = mman::mmap(ptr::null_mut(),
                                  size,
                                  mman::PROT_READ | mman::PROT_WRITE,
                                  mman::MAP_SHARED,
                                  fd,
                                  0)?;
            Ok(addr as *mut u8)
        })
        .unwrap_or(ptr::null_mut())
}

#[inline]
pub unsafe fn deallocate(ptr: *mut u8, old_size: usize, _align: usize) {
    mman::munmap(ptr as *mut _, old_size).expect("Could not munmap");
}
