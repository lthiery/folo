/// Information about the resource quota that the operating system enforces for the current process.
///
/// The active resource quota may change over time. You can use [`HardwareTracker`][1] to obtain
/// fresh information about the current resource quota at any time.
///
/// [1]: crate::HardwareTracker
#[derive(Debug)]
pub struct ResourceQuota {
    max_processor_time: f64,
}

impl ResourceQuota {
    pub(crate) fn new(max_processor_time: f64) -> Self {
        Self { max_processor_time }
    }

    /// How many seconds of processor time the process is allowed to use per second of real time.
    ///
    /// This will never be more than the number of processors available to the current process.
    #[must_use]
    #[inline]
    pub fn max_processor_time(&self) -> f64 {
        self.max_processor_time
    }
}
