// Copyright 2019 Intel Corporation. All Rights Reserved.
// SPDX-License-Identifier: (BSD-3-Clause OR Apache-2.0)

#[cfg(feature = "vfio-v5_0_0")]
mod bindings_v5_0_0;

// Default to latest version if no version is specified by using the features.
#[cfg(not(feature = "vfio-v5_0_0"))]
mod bindings_v5_0_0;

pub mod bindings {
    #[cfg(feature = "vfio-v5_0_0")]
    pub use super::bindings_v5_0_0::*;

    #[cfg(not(feature = "vfio-v5_0_0"))]
    pub use super::bindings_v5_0_0::*;
}
