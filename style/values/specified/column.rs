/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Specified types for the column properties.

use crate::parser::{Parse, ParserContext};
use crate::values::generics::column::{GenericColumnCount, GenericColumnWidth};
use crate::values::specified::{NonNegativeLength, NonNegativeLengthPercentage, PositiveInteger};
use cssparser::Parser;
use style_traits::ParseError;

/// A specified type for `column-count` values.
pub type ColumnCount = GenericColumnCount<PositiveInteger>;

/// A specified type for `column-width` values.
pub type ColumnWidth = GenericColumnWidth<NonNegativeLength, NonNegativeLengthPercentage>;

impl Parse for ColumnWidth {
    fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        if let Ok(length) = input.try_parse(|i| NonNegativeLength::parse(context, i)) {
            return Ok(Self::Length(length));
        }
        if input
            .try_parse(|i| i.expect_function_matching("fit-content"))
            .is_ok()
        {
            return input.parse_nested_block(|i| {
                Ok(Self::FitContent(NonNegativeLengthPercentage::parse(
                    context, i,
                )?))
            });
        }
        Ok(try_match_ident_ignore_ascii_case! { input,
            "auto" => Self::Auto,
            "min-content" => Self::MinContent,
            "max-content" => Self::MaxContent,
        })
    }
}
