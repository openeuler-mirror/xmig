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

use std::sync::atomic::{AtomicU32, Ordering};

use linux_futex::{Futex, Shared, WaitError};

mod limits {
    pub const MAX_SPIN_COUNT: u32 = 100;
}

mod state {
    pub const UNLOCKED: u32 = 0;
    pub const LOCKED_UNCONTENDED: u32 = 1;
    pub const LOCKED_CONTENDED: u32 = 2;
}

/// A cross-process mutex implemented by futex.
#[repr(C)]
#[derive(Debug)]
pub struct FutexMutex {
    state: Futex<Shared>,
}

impl FutexMutex {
    #[inline(always)]
    fn value(&self) -> &AtomicU32 {
        &self.state.value
    }

    /// The slow path for acquiring the lock, invoked only when the initial fast-path fails.
    /// This function handles contention by first spinning for a short period, then falling
    /// back to a kernel-level wait (futex).
    #[cold]
    fn lock_contended(&self) {
        // --- Phase 1: Spinning ---
        // We spin for a limited number of iterations to see if the lock is released quickly.
        // This avoids the high cost of a context switch if contention is brief.
        for _ in 0..limits::MAX_SPIN_COUNT {
            // On each spin iteration, we optimistically try to acquire the lock as
            // "uncontended". If we succeed here, it means we avoided a true collision.
            if self
                .value()
                .compare_exchange(
                    state::UNLOCKED,
                    state::LOCKED_UNCONTENDED,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                // We acquired the lock while spinning.
                return;
            }

            // Hint to the CPU that we are in a spin-wait loop.
            std::hint::spin_loop();
        }

        // --- Phase 2: Blocking ---
        // If the spin loop completes, we transition to a blocking strategy.
        let mut local_state = self.value().load(Ordering::Relaxed);
        loop {
            match local_state {
                state::LOCKED_UNCONTENDED => {
                    // If the lock is held but not marked as contended, we must mark it
                    // before we can safely wait.
                    match self.value().compare_exchange(
                        state::LOCKED_UNCONTENDED,
                        state::LOCKED_CONTENDED,
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => {
                            // We successfully marked the lock as contended.
                            // Update our local state and proceed to wait.
                            local_state = state::LOCKED_CONTENDED;
                        }
                        Err(new_state) => {
                            // The CAS failed, meaning the state changed under us.
                            // We must restart the loop with the new, fresh state.
                            local_state = new_state;
                            continue;
                        }
                    }
                }
                state::UNLOCKED => {
                    // If the lock was released, we try to acquire it.
                    // *** THIS IS THE CRITICAL FIX FOR THE DEADLOCK ***
                    // By setting the state to LOCKED_CONTENDED (2), we are being "pessimistic".
                    // We acknowledge that we were part of a contended slow path, so there might
                    // be other waiters. This ensures that when we eventually unlock, a wake-up
                    // call will be triggered, preventing other threads from sleeping forever.
                    match self.value().compare_exchange(
                        state::UNLOCKED,
                        state::LOCKED_CONTENDED,
                        Ordering::Acquire,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => {
                            // Success! We acquired the lock and preserved contention info.
                            return;
                        }
                        Err(new_state) => {
                            // We lost the race to acquire the lock. Restart with the new state.
                            local_state = new_state;
                            continue;
                        }
                    }
                }
                state::LOCKED_CONTENDED => {
                    // It is now safe to wait.
                    match self.state.wait(state::LOCKED_CONTENDED) {
                        Ok(()) | Err(WaitError::WrongValue) | Err(WaitError::Interrupted) => {
                            // After waking up for any reason, we don't own the lock yet.
                            // We must reload the state and re-run the loop to decide what to do next.
                            local_state = self.value().load(Ordering::Relaxed);
                        }
                    }
                }
                _ => unreachable!("Invalid futex mutex status"),
            }
        }
    }
}

impl FutexMutex {
    /// Creates a new, unlocked FutexMutex.
    pub const fn new() -> Self {
        Self {
            state: Futex::new(state::UNLOCKED),
        }
    }

    /// Acquires the lock, blocking the current thread/process until it's available.
    pub fn lock(&self) -> FutexMutexGuard<'_> {
        // Fast path: Try to acquire the lock if it's uncontended.
        if self
            .value()
            .compare_exchange(
                state::UNLOCKED,
                state::LOCKED_UNCONTENDED,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_ok()
        {
            return FutexMutexGuard { mutex: self };
        }

        // Slow path: The lock is contended, we need to wait.
        self.lock_contended();

        FutexMutexGuard { mutex: self }
    }

    /// Unlocks the mutex.
    fn unlock(&self) {
        // Atomically set the state back to UNLOCKED.
        let old_state = self.value().swap(state::UNLOCKED, Ordering::Release);
        if old_state == state::LOCKED_CONTENDED {
            // If the state was `LOCKED_CONTENDED`, it means someone is waiting.
            // We must wake them up.
            self.state.wake(1);
        }
    }
}

impl Default for FutexMutex {
    fn default() -> Self {
        Self::new()
    }
}

/// An RAII guard that releases the lock when it goes out of scope.
#[derive(Debug)]
pub struct FutexMutexGuard<'a> {
    mutex: &'a FutexMutex,
}

impl Drop for FutexMutexGuard<'_> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Arc, thread};

    #[test]
    fn test_mutex_contend() {
        const NUM_THREADS: usize = 10;
        const INCREMENTS_PER_THREAD: usize = 10_000;

        // Shared state: a counter protected by our FutexMutex.
        // We wrap them in an Arc to share them safely across threads.
        let counter = Arc::new(0_usize);
        let mutex = Arc::new(FutexMutex::new());

        let mut handles = Vec::new();

        for _ in 0..NUM_THREADS {
            let counter_clone = Arc::clone(&counter);
            let mutex_clone = Arc::clone(&mutex);

            let handle = thread::spawn(move || {
                for _ in 0..INCREMENTS_PER_THREAD {
                    // Acquire the lock. The guard will auto-release it when it goes out of scope.
                    let _guard = mutex_clone.lock();

                    // --- Critical Section ---
                    // This is where the mutual exclusion is tested. Without a correct lock,
                    // this read-modify-write operation would be a classic race condition.
                    let num = unsafe { &mut *(Arc::as_ptr(&counter_clone) as *mut usize) };
                    *num += 1;
                    // --- End of Critical Section ---
                }
            });

            handles.push(handle);
        }

        // Wait for all threads to complete their work.
        for handle in handles {
            handle.join().unwrap();
        }

        // The final assertion. If the mutex works correctly, the final count
        // must be exactly the total number of increments.
        let final_count = unsafe { *Arc::into_raw(counter) };
        assert_eq!(
            final_count,
            NUM_THREADS * INCREMENTS_PER_THREAD,
            "The final count is incorrect, indicating a race condition occurred."
        );
    }
}
