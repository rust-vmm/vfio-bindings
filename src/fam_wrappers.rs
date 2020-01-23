// Copyright © 2019 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause
//

use crate::bindings::vfio::*;
use vmm_sys_util::fam::{FamStruct, FamStructWrapper};

const MSIX_MAX_VECTORS: usize = 2048;

// Implement the FamStruct trait vfio_irq_set.
generate_fam_struct_impl!(vfio_irq_set, u8, data, u32, count, MSIX_MAX_VECTORS);

/// Wrapper over the `vfio_irq_set` structure.
///
/// The `vfio_irq_set` structure contains a flexible array member. For details check the
/// [VFIO userspace API](https://github.com/torvalds/linux/blob/master/include/uapi/linux/vfio.h)
/// documentation on `vfio_irq_set`. To provide safe access to the array
/// elements, this type is implemented using
/// [FamStructWrapper](../vmm_sys_util/fam/struct.FamStructWrapper.html).
pub type IrqSet = FamStructWrapper<vfio_irq_set>;
