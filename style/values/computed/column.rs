/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Computed types for the column properties.

use crate::values::computed::{NonNegativeLength, NonNegativeLengthPercentage, PositiveInteger};
use crate::values::generics::column::{GenericColumnCount, GenericColumnWidth};

/// A computed type for `column-count` values.
pub type ColumnCount = GenericColumnCount<PositiveInteger>;

/// A computed type for `column-width` values.
pub type ColumnWidth = GenericColumnWidth<NonNegativeLength, NonNegativeLengthPercentage>;
