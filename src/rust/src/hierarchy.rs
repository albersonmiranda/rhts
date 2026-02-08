// Copyright (C) 2026 Alberson Miranda
//
// This file is part of rhts.
//
// rhts is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rhts is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rhts.  If not, see <https://www.gnu.org/licenses/\>.

use crate::helpers::{from_robj, to_robj};
use extendr_api::prelude::*;
use hts_core::{HierarchicalTimeSeries, HierarchySpec};

/// Specification of hierarchical and grouped structure.
/// @description
/// Defines the hierarchical and grouped structure of time series data.
/// Hierarchical columns have strict parent-child nesting, while grouped columns
/// cross with the hierarchy at all levels.
/// @param hierarchy (`character`)\cr
/// Columns with strict parent-child nesting, ordered from top to bottom.
/// @usage NULL
/// @format NULL
/// @export
#[extendr]
pub struct HtsSpec {
    inner: HierarchySpec,
}

#[extendr]
impl HtsSpec {
    /// Create a new HierarchySpec
    /// @description
    /// Instantiate a new HierarchySpec
    /// @param hierarchy Character vector of hierarchical column names (ordered from top to bottom)
    /// @param groups Character vector of grouped column names
    /// @return A new HierarchySpec object
    /// @examples
    /// # Hierarchical only
    /// spec <- HierarchySpec$new(hierarchy = c("State", "Region"), groups = c())
    ///
    /// # Hierarchical with groups
    /// spec <- HierarchySpec$new(
    ///   hierarchy = c("State", "Region"),
    ///   groups = c("Purpose")
    /// )
    pub fn new(hierarchy: Vec<String>, groups: Vec<String>) -> Self {
        Self {
            inner: HierarchySpec::new(hierarchy, groups),
        }
    }

    /// Create a spec with only hierarchical columns (no grouping)
    /// @param columns Character vector of hierarchical column names
    /// @return A new HierarchySpec object
    /// @examples
    /// spec <- HierarchySpec$hierarchical(c("State", "Region"))
    pub fn hierarchical(columns: Vec<String>) -> Self {
        Self {
            inner: HierarchySpec::hierarchical(columns),
        }
    }

    /// Create a spec with only grouped columns (no hierarchy)
    /// @param columns Character vector of grouped column names
    /// @return A new HierarchySpec object
    /// @examples
    /// spec <- HierarchySpec$grouped(c("Product", "Category"))
    pub fn grouped(columns: Vec<String>) -> Self {
        Self {
            inner: HierarchySpec::grouped(columns),
        }
    }

    /// Print method for HierarchySpec
    pub fn print(&self) {
        rprintln!("<HierarchySpec>");
        rprintln!("  Hierarchy: {:?}", self.inner.hierarchy);
        rprintln!("  Groups: {:?}", self.inner.groups);
    }
}

/// Hierarchical Time Series
/// @description
/// Main data structure for working with hierarchical and/or grouped time series.
/// Holds bottom-level data along with computed hierarchy tree and summation matrix.
/// @export
#[extendr]
pub struct Hts {
    inner: HierarchicalTimeSeries,
}

#[extendr]
impl Hts {
    /// Create a new HierarchicalTimeSeries
    /// @description
    /// Instantiate a new HierarchicalTimeSeries
    /// @param bottom_level Dataframe containing the time series data
    /// @param spec HierarchySpec object defining the structure
    /// @param time_col Name of the time/period column
    /// @param value_col Name of the value column
    /// @return A new HierarchicalTimeSeries object
    /// @examples
    /// hts_data <- data.frame(
    ///  state = c(
    ///    rep("Rio de Janeiro", 4), rep("São Paulo", 4),
    ///    rep("Rio de Janeiro", 4), rep("São Paulo", 4)
    ///  ),
    ///  city = c(
    ///    rep("Rio de Janeiro", 2), rep("Duque de Caxias", 2),
    ///    rep("São Paulo", 2), rep("Campinas", 2),
    ///    rep("Rio de Janeiro", 2), rep("Duque de Caxias", 2),
    ///    rep("São Paulo", 2), rep("Campinas", 2)
    ///  ),
    ///  sector = c(
    ///    rep("Industry", 8), rep("Agriculture", 8)
    ///  ),
    ///  quarter = c(
    ///    rep("2024 Q1", 16), rep("2024 Q2", 16)
    ///  ),
    ///  gdp = c(
    ///    1000, 500, 150, 120,
    ///    2000, 800, 300, 200,
    ///    1500, 800, 200, 150,
    ///    2200, 900, 400, 300,
    ///    1100, 600, 180, 130,
    ///    2100, 850, 320, 220,
    ///    1600, 850, 220, 160,
    ///    2300, 950, 420, 320
    ///  )
    ///)
    /// spec <- HtsSpec$new(c("state", "city"), c("sector"))
    /// hts <- Hts$new(hts_data, spec, "quarter", "gdp")
    pub fn new(
        bottom_level: Robj,
        spec: &HtsSpec,
        time_col: &str,
        value_col: &str,
    ) -> Result<Self, Error> {
        // create polars dataframe
        let df = from_robj(bottom_level)?;

        // create Hts
        let inner = HierarchicalTimeSeries::new(df, spec.inner.clone(), time_col, value_col)
            .map_err(|e| Error::from(e.to_string()))?;

        Ok(Self { inner })
    }

    /// Load from CSV file
    /// @param path Path to CSV file
    /// @param spec HierarchySpec object defining the structure
    /// @param time_col Name of the time/period column
    /// @param value_col Name of the value column
    /// @return A new HierarchicalTimeSeries object
    /// @examples
    /// \dontrun{
    /// write.csv(tsibble::tourism, "data.csv", row.names = FALSE)
    /// spec <- HierarchySpec$new(c("State", "Region"), c("Purpose"))
    /// hts <- HierarchicalTimeSeries$from_csv("data.csv", spec, "Quarter", "Trips")
    /// }
    pub fn from_csv(
        path: &str,
        spec: &HtsSpec,
        time_col: &str,
        value_col: &str,
    ) -> Result<Self, Error> {
        let inner = HierarchicalTimeSeries::from_csv(path, spec.inner.clone(), time_col, value_col)
            .map_err(|e| format!("Failed to load CSV: {}", e))?;
        Ok(Self { inner })
    }

    /// Get total number of series (all aggregation levels)
    /// @return Integer count of total series
    pub fn n_series(&self) -> i32 {
        self.inner.n_series() as i32
    }

    /// Get number of bottom-level series
    /// @return Integer count of bottom-level series
    pub fn n_bottom(&self) -> i32 {
        self.inner.n_bottom() as i32
    }

    /// Get number of time periods
    /// @return Integer count of time periods
    pub fn n_periods(&self) -> i32 {
        self.inner.n_periods() as i32
    }

    /// Print summary of the hierarchical structure
    pub fn print(&self) {
        let summary = self.inner.summary();
        rprintln!("{}", summary);
    }

    /// Get summation matrix with row and column labels
    /// @return List containing the matrix and its labels
    pub fn summation_matrix(&self) -> List {
        let matrix = self.inner.summation_matrix();
        let data = &matrix.matrix;
        let shape = data.shape();
        let mat = RMatrix::new_matrix(
            shape.0 as usize,
            shape.1 as usize,
            |row, col| data[(row, col)]
        );

        list!(
            matrix = mat,
            row_labels = matrix.row_labels.clone(),
            col_labels = matrix.col_labels.clone()
        )
    }

    /// Get aggregated series
    /// @return Dataframe containing all series with their hierarchical labels
    pub fn aggregated_series(&self) -> Result<Robj, Error> {
        let polars_df = self.inner.aggregate_all()
            .map_err(|e| Error::from(e.to_string()))?;

        // convert polars dataframe to R dataframe
        Ok(to_robj(polars_df).into())
    }
}

extendr_module! {
    mod hierarchy;
    impl HtsSpec;
    impl Hts;
}
