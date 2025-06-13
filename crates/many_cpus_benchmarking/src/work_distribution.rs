use derive_more::Display;

/// How work is distributed among processors during a benchmark run.
///
/// The work is redistributed for each benchmark iteration, ensuring that hardware-specific
/// performance anomalies are averaged out (e.g. if some processors have worse thermals and
/// throttle more often).
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
#[non_exhaustive]
pub enum WorkDistribution {
    /// One worker pair is spawned for each numerically neighboring memory region pair.
    ///
    /// For example, with 3 memory regions, we would have 3 pairs of workers: (0, 1), (1, 2), (2, 0).
    ///
    /// Each pair will work together, processing one payload per pair. In total,
    /// there will be two workers per memory region (one working with the "previous"
    /// memory region and one working with the "next" one).
    ///
    /// Each worker is pinned to a specific processor.
    ///
    /// Different memory regions may be a different distance apart, so this allows us to average
    /// out any differences - some pairs are faster, some are slower, we just want to average it
    /// out so every benchmark run is consistent (instead of picking two random memory regions).
    ///
    /// This option can only be used if there are at least two memory regions. Benchmark runs with
    /// this distribution will be skipped if the system only has a single memory region.
    PinnedMemoryRegionPairs,

    /// Each worker in a pair is spawned in the same memory region.
    ///
    /// Each pair will work together, processing one payload between the two members. Different
    /// pairs may be in different memory regions.
    ///
    /// Each worker is pinned to a specific processor.
    ///
    /// The number of pairs will match the number that would have been used with
    /// `PinnedMemoryRegionPairs`, for optimal comparability. There will be a minimum of one pair.
    PinnedSameMemoryRegion,

    /// Both workers in each pair are spawned on the same processor, picked arbitrarily from
    /// each memory region. One pair from each memory region, on the same processor.
    ///
    /// Each pair will work together, processing one payload between the two members. Different
    /// pairs may be in different memory regions.
    ///
    /// This can occasionally be insightful when it surprises you by showing that two threads on
    /// the same processor do not need twice as long to get twice as much work done. Not useful
    /// with most scenarios, though - best to skip unless probing specifically for this effect.
    ///
    /// It may also showcase effects of memory regions when compared to `UnpinnedSameProcessor`.
    ///
    /// The number of pairs will match the number that would have been used with
    /// `PinnedMemoryRegionPairs`, for optimal comparability. There will be a minimum of one pair.
    PinnedSameProcessor,

    /// All workers are spawned without regard to memory region or processor, randomly picking
    /// processors for each iteration.
    ///
    /// Each worker is given back its own payload - while we still spawn the same number of workers
    /// as in the paired scenarios, each member of the pair operates independently and processes
    /// its own payload.
    ///
    /// Note that this requires the benchmark scenario logic to be capable of handling its own data
    /// set. If the benchmark logic requires two collaborating workers, you cannot use this work
    /// distribution as it would likely end in a deadlock due to lack of a partner.
    ///
    /// This mode is also unlikely to be informative if the scenario setup does not distinguish
    /// between the two workers (they either do the same thing or pick dynamically who does what).
    PinnedSelf,

    /// Like `PinnedMemoryRegionPairs` but each worker is allowed to float among all the processors
    /// in the memory region, based on the operating system's scheduling decisions.
    ///
    /// We still have the same total number of workers to keep total system load equivalent.
    UnpinnedMemoryRegionPairs,

    /// Like `PinnedSameMemoryRegion` but each worker is allowed to float among half the processors
    /// in the memory region, based on the operating system's scheduling decisions. Each member of
    /// the pair gets one half of the processors in the memory region.
    ///
    /// We still have the same total number of workers to keep total system load equivalent.
    ConstrainedSameMemoryRegion,

    /// All workers are spawned without regard to memory region or processor, leaving it up
    /// to the operating system to decide where to run them. Note that, typically, this will still
    /// result in them running in the same memory region, as that tends to be the default behavior.
    ///
    /// Each worker is given back its own payload - while we still spawn the same number of workers
    /// as in the paired scenarios, each member of the pair operates independently and processes
    /// its own payload.
    ///
    /// Note that this requires the benchmark scenario logic to be capable of handling its own data
    /// set. If the benchmark logic requires two collaborating workers, you cannot use this work
    /// distribution as it would likely end in a deadlock due to lack of a partner.
    ///
    /// This mode is also unlikely to be informative if the scenario setup does not distinguish
    /// between the two workers (they either do the same thing or pick dynamically who does what).
    UnpinnedSelf,

    /// All workers are spawned without regard to processor but in specific memory regions, matching
    /// the memory regions used in `PinnedMemoryRegionPairs`. Each worker is allowed to float among
    /// all the processors in the memory region, based on the operating system's scheduling
    /// decisions.
    ///
    /// Each worker is given back its own payload - while we still spawn the same number of workers
    /// as in the paired scenarios, each member of the pair operates independently and processes
    /// its own payload.
    ///
    /// Note that this requires the benchmark scenario logic to be capable of handling its own data
    /// set. If the benchmark logic requires two collaborating workers, you cannot use this work
    /// distribution as it would likely end in a deadlock due to lack of a partner.
    UnpinnedPerMemoryRegionSelf,
}

impl WorkDistribution {
    /// All the work distribution modes.
    #[must_use]
    pub fn all() -> &'static [Self] {
        &[
            Self::PinnedMemoryRegionPairs,
            Self::PinnedSameMemoryRegion,
            Self::PinnedSameProcessor,
            Self::PinnedSelf,
            Self::UnpinnedMemoryRegionPairs,
            Self::ConstrainedSameMemoryRegion,
            Self::UnpinnedSelf,
            Self::UnpinnedPerMemoryRegionSelf,
        ]
    }

    /// All the work distribution modes that exchange payloads between processors
    /// before starting the benchmark.
    #[must_use]
    pub fn all_without_self() -> &'static [Self] {
        &[
            Self::PinnedMemoryRegionPairs,
            Self::PinnedSameMemoryRegion,
            Self::PinnedSameProcessor,
            Self::UnpinnedMemoryRegionPairs,
            Self::ConstrainedSameMemoryRegion,
        ]
    }

    /// All the work distribution modes that allow every worker to be placed
    /// on a different processor (either explicitly or by allowing them to float
    /// based on OS scheduling decisions).
    #[must_use]
    pub fn all_with_unique_processors() -> &'static [Self] {
        &[
            Self::PinnedMemoryRegionPairs,
            Self::PinnedSameMemoryRegion,
            Self::PinnedSelf,
            Self::UnpinnedMemoryRegionPairs,
            Self::ConstrainedSameMemoryRegion,
            Self::UnpinnedSelf,
            Self::UnpinnedPerMemoryRegionSelf,
        ]
    }

    /// All the work distribution modes that allow every worker to be placed
    /// on a different processor (either explicitly or by allowing them to float
    /// based on OS scheduling decisions) and exchange payloads between workers
    /// before starting the benchmark.
    #[must_use]
    pub fn all_with_unique_processors_without_self() -> &'static [Self] {
        &[
            Self::PinnedMemoryRegionPairs,
            Self::PinnedSameMemoryRegion,
            Self::UnpinnedMemoryRegionPairs,
            Self::ConstrainedSameMemoryRegion,
        ]
    }
}
