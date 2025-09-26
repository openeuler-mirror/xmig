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

use std::sync::OnceLock;

use nix::unistd::{SysconfVar, sysconf};

pub fn page_size() -> usize {
    static PAGE_SIZE: OnceLock<usize> = OnceLock::new();

    *PAGE_SIZE.get_or_init(|| {
        let size = sysconf(SysconfVar::PAGE_SIZE)
            .expect("Failed to execute sysconf(_SC_PAGESIZE)")
            .expect("System does not support PAGE_SIZE query");
        size.try_into()
            .unwrap_or_else(|_| panic!("Page size {} exceeds usize::MAX ({})", size, usize::MAX))
    })
}

pub fn page_align(value: usize) -> usize {
    let page_size = self::page_size();
    value.div_ceil(page_size).saturating_mul(page_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_size() {
        let page_size = page_size();

        assert!(page_size > 0);
        assert!(page_size.is_power_of_two());
    }

    #[test]
    fn test_page_align() {
        let page_size = page_size();

        assert_eq!(page_align(4096), 4096);
        assert_eq!(page_align(page_size), page_size);

        assert_eq!(page_align(1024), page_size);
        assert_eq!(page_align(page_size - 1), page_size);
        assert_eq!(page_align(page_size + 1), page_size * 2);

        let large_value = usize::MAX - page_size + 1;
        assert_eq!(
            page_align(large_value),
            usize::MAX - (usize::MAX % page_size)
        );
    }

    #[test]
    fn test_test_page_align_edge_cases() {
        let page_size = page_size();

        assert_eq!(page_align(usize::MAX), usize::MAX);

        let value = page_size * 3 - 1;
        assert_eq!(page_align(value), page_size * 3);
    }
}
