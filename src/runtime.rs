#![allow(clippy::redundant_clone)]
#![allow(clippy::too_many_arguments)]
#[subxt::subxt(
    runtime_metadata_path = "eden.scale",
    derive_for_all_types = "Clone,Eq, PartialEq",
    // To apply derives to specific generated types, add a `derive_for_type` per type,
    // mapping the type path to the derives which should be added for that type only.
    // Note that these derives will be in addition to those specified above in
    // `derive_for_all_types`
)]
pub mod eden {}
