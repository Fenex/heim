use std::fmt;

use heim_common::prelude::*;

use crate::sys;

/// CPU statistics.
///
/// See [OS-specific extensions](./os/index.html) for more statistics.
#[derive(heim_derive::ImplWrap)]
pub struct CpuStats(sys::CpuStats);

impl CpuStats {
    /// Returns number of context switches (voluntary + involuntary) since system boot.
    pub fn ctx_switches(&self) -> u64 {
        self.as_ref().ctx_switches()
    }

    /// Returns number of interrupts since system boot.
    pub fn interrupts(&self) -> u64 {
        self.as_ref().interrupts()
    }
}

impl fmt::Debug for CpuStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CpuStats")
            .field("ctx_switches", &self.ctx_switches())
            .field("interrupts", &self.interrupts())
            .finish()
    }
}

/// Returns future which will resolve into [CpuStats].
///
/// [CpuStats]: ./struct.CpuStats.html
pub fn stats() -> impl Future<Output = Result<CpuStats>> {
    sys::stats().map_ok(Into::into)
}
