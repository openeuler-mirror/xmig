// SPDX-License-Identifier: Mulan PSL v2
/*
 * Copyright (c) 2025 Huawei Technologies Co., Ltd.
 * This software is licensed under Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *         http://license.coscl.org.cn/MulanPSL2
 *
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
 * EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
 * MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 * See the Mulan PSL v2 for more details.
 */

use std::{
    fmt::{Debug, Display},
    io,
    os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd, RawFd},
};

use nix::{
    errno::Errno,
    fcntl::OFlag,
    sys::{
        mman as mm,
        stat::{self, Mode},
    },
    unistd,
};

#[repr(transparent)]
pub struct ShmemName(String);

impl ShmemName {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        let name = name.as_ref().trim();
        debug_assert!(!name.is_empty(), "Shmem name cannot be empty");

        let shmem_name = format!("/{}", name.trim_start_matches('/').replace('/', "_"));
        Self(shmem_name)
    }
}

impl AsRef<str> for ShmemName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Debug for ShmemName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for ShmemName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[derive(Debug)]
pub struct Shmem {
    name: ShmemName,
    fd: OwnedFd,
    size: usize,
    owned: bool,
}

impl Shmem {
    pub fn create<S: AsRef<str>>(name: S, size: usize) -> io::Result<Self> {
        let name = name.as_ref();
        if name.is_empty() || size == 0 {
            return Err(Errno::EINVAL.into());
        }

        let fd = mm::shm_open(
            name,
            OFlag::O_RDWR | OFlag::O_CREAT | OFlag::O_EXCL,
            Mode::S_IRUSR | Mode::S_IWUSR | Mode::S_IRGRP | Mode::S_IWGRP,
        )?;
        unistd::ftruncate(&fd, size.try_into().map_err(|_| Errno::EOVERFLOW)?)?;

        let name = ShmemName::new(name);
        let shmem = Self {
            name,
            fd,
            size,
            owned: true,
        };
        Ok(shmem)
    }

    pub fn open<S: AsRef<str>>(name: S) -> io::Result<Self> {
        let name = name.as_ref();
        if name.is_empty() {
            return Err(Errno::EINVAL.into());
        }

        let fd = mm::shm_open(name, OFlag::O_RDWR, Mode::empty())?;
        let size = {
            let metadata = stat::fstat(&fd)?;
            if metadata.st_size < 0 {
                return Err(Errno::EBADFD.into());
            }
            if metadata.st_size == 0 {
                return Err(Errno::EINVAL.into());
            }
            metadata.st_size.try_into().map_err(|_| Errno::EOVERFLOW)
        }?;

        let name = ShmemName::new(name);
        let shmem = Self {
            name,
            fd,
            size,
            owned: false,
        };
        Ok(shmem)
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_owner(&self) -> bool {
        self.owned
    }
}

impl AsFd for Shmem {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

impl AsRawFd for Shmem {
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}

impl Display for Shmem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.name, f)
    }
}

impl Drop for Shmem {
    fn drop(&mut self) {
        if self.owned {
            let _ = mm::shm_unlink(self.name.as_ref());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        ffi::OsStr,
        process,
        sync::{
            OnceLock,
            atomic::{AtomicUsize, Ordering},
        },
        thread,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    fn unique_shmem_addr() -> String {
        static SEQ: AtomicUsize = AtomicUsize::new(0);
        static EXEC_NAME: OnceLock<String> = OnceLock::new();

        let exec_name = EXEC_NAME.get_or_init(|| {
            std::env::current_exe()
                .ok()
                .and_then(|p| p.file_stem().and_then(OsStr::to_str).map(str::to_owned))
                .unwrap_or_else(|| "unknown".into())
        });
        let curr_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .expect("Failed to get current time");

        format!(
            "/{}_{}_{}_{}",
            exec_name,
            process::id(),
            curr_time,
            SEQ.fetch_add(1, Ordering::Relaxed)
        )
    }

    #[test]
    fn test_create_errors() {
        let name = unique_shmem_addr();
        assert!(Shmem::create(&name, 0).is_err());

        let _shmem = Shmem::create(&name, 1024).unwrap();
        assert!(Shmem::create(&name, 1024).is_err());
        assert!(Shmem::create(&name, 2048).is_err());
    }

    #[test]
    fn test_create_and_open() {
        let name = unique_shmem_addr();
        let size = 4096;

        let shmem = Shmem::create(&name, size).unwrap();
        assert_eq!(shmem.name(), name);
        assert_eq!(shmem.size(), size);
        assert!(shmem.is_owner());

        let shmem2 = Shmem::open(&name).unwrap();
        assert_eq!(shmem2.name(), name);
        assert_eq!(shmem2.size(), size);
        assert!(!shmem2.is_owner());
    }

    #[test]
    fn test_create_and_open_threaded() {
        let name = unique_shmem_addr();
        let size = 1024;

        let _shmem = Shmem::create(&name, size).unwrap();
        let child = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            let shmem_child = Shmem::open(&name).unwrap();
            assert_eq!(shmem_child.size(), size);
        });

        child.join().unwrap();
    }

    #[test]
    fn test_cleanup() {
        let name = unique_shmem_addr();
        {
            let _shmem = Shmem::create(&name, 1024).unwrap();
        }
        assert!(Shmem::open(&name).is_err());
    }
}
