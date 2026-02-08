
<!-- README.md is generated from README.Rmd. Please edit that file -->

# rhts

<!-- badges: start -->

[![extendr](https://img.shields.io/badge/extendr-%5E0.8.1-276DC2)](https://extendr.github.io/extendr/extendr_api/)
<!-- badges: end -->

`rhts` provides hierarchical and grouped time series data structures for
R, powered by the Rust crates in
[`hts-rs`](https://github.com/albersonmiranda/hts-rs).

## Why `rhts`

It aims to fill the gap between literature and software, offering the
latest algorithms for hierarchical time series in a user-friendly R
package.

## Key concepts

- **Hierarchical series**: strict parent-child nesting (e.g., State
  $\rightarrow$ City).
- **Grouped series**: cross-cutting dimensions applied across all
  hierarchy levels (e.g., Sector).
- **Summation matrix** $S$: maps bottom-level series to every
  aggregation level via $y_t = Sb_t$.

## Features

- Flexible hierarchy and grouping specifications.
- Aggregation across all levels from bottom-level series.
- Summation matrix.
- Time frequency agnostic: Annual, Quarterly, Monthly, Weekly, and Daily
  formats.

## Installation

You can install the development version of `rhts` from GitHub. A [Rust
toolchain](https://rust-lang.org/) is required to build the package.

``` r
# install.packages("remotes")
remotes::install_github("albersonmiranda/rhts")
```

## Getting started

`rhts` exposes two core classes:

- `HtsSpec`: defines hierarchical and grouped columns.
- `Hts`: loads data, builds the hierarchy, and exposes aggregation
  utilities.

### Example

This is a basic example using a mixed hierarchy (state, city) with a
grouped dimension (sector).

``` r
library(rhts)
hts_data <- data.frame(
  state = c(
    rep("Rio de Janeiro", 4), rep("São Paulo", 4),
    rep("Rio de Janeiro", 4), rep("São Paulo", 4)
  ),
  city = c(
    rep("Rio de Janeiro", 2), rep("Duque de Caxias", 2),
    rep("São Paulo", 2), rep("Campinas", 2),
    rep("Rio de Janeiro", 2), rep("Duque de Caxias", 2),
    rep("São Paulo", 2), rep("Campinas", 2)
  ),
  sector = c(
    rep("Industry", 8), rep("Agriculture", 8)
  ),
  quarter = c(
    rep("2024 Q1", 16), rep("2024 Q2", 16)
  ),
  gdp = c(
    1000, 500, 150, 120,
    2000, 800, 300, 200,
    1500, 800, 200, 150,
    2200, 900, 400, 300,
    1100, 600, 180, 130,
    2100, 850, 320, 220,
    1600, 850, 220, 160,
    2300, 950, 420, 320
  )
)

spec <- HtsSpec$new(c("state", "city"), "sector")
hts <- Hts$new(hts_data, spec, "quarter", "gdp")

# Summation matrix
hts$summation_matrix()
#> $matrix
#>       [,1] [,2] [,3] [,4] [,5] [,6] [,7] [,8]
#>  [1,]    1    1    1    1    1    1    1    1
#>  [2,]    1    0    1    0    1    0    1    0
#>  [3,]    0    1    0    1    0    1    0    1
#>  [4,]    1    1    1    1    0    0    0    0
#>  [5,]    0    0    0    0    1    1    1    1
#>  [6,]    1    0    1    0    0    0    0    0
#>  [7,]    0    1    0    1    0    0    0    0
#>  [8,]    0    0    0    0    1    0    1    0
#>  [9,]    0    0    0    0    0    1    0    1
#> [10,]    1    1    0    0    0    0    0    0
#> [11,]    0    0    1    1    0    0    0    0
#> [12,]    0    0    0    0    1    1    0    0
#> [13,]    0    0    0    0    0    0    1    1
#> [14,]    1    0    0    0    0    0    0    0
#> [15,]    0    1    0    0    0    0    0    0
#> [16,]    0    0    1    0    0    0    0    0
#> [17,]    0    0    0    1    0    0    0    0
#> [18,]    0    0    0    0    1    0    0    0
#> [19,]    0    0    0    0    0    1    0    0
#> [20,]    0    0    0    0    0    0    1    0
#> [21,]    0    0    0    0    0    0    0    1
#> 
#> $row_labels
#>  [1] "Total"                                     
#>  [2] "Agriculture"                               
#>  [3] "Industry"                                  
#>  [4] "Rio de Janeiro"                            
#>  [5] "São Paulo"                                 
#>  [6] "Rio de Janeiro/Agriculture"                
#>  [7] "Rio de Janeiro/Industry"                   
#>  [8] "São Paulo/Agriculture"                     
#>  [9] "São Paulo/Industry"                        
#> [10] "Rio de Janeiro/Duque de Caxias"            
#> [11] "Rio de Janeiro/Rio de Janeiro"             
#> [12] "São Paulo/Campinas"                        
#> [13] "São Paulo/São Paulo"                       
#> [14] "Rio de Janeiro/Duque de Caxias/Agriculture"
#> [15] "Rio de Janeiro/Duque de Caxias/Industry"   
#> [16] "Rio de Janeiro/Rio de Janeiro/Agriculture" 
#> [17] "Rio de Janeiro/Rio de Janeiro/Industry"    
#> [18] "São Paulo/Campinas/Agriculture"            
#> [19] "São Paulo/Campinas/Industry"               
#> [20] "São Paulo/São Paulo/Agriculture"           
#> [21] "São Paulo/São Paulo/Industry"              
#> 
#> $col_labels
#> [1] "Rio de Janeiro/Duque de Caxias/Agriculture"
#> [2] "Rio de Janeiro/Duque de Caxias/Industry"   
#> [3] "Rio de Janeiro/Rio de Janeiro/Agriculture" 
#> [4] "Rio de Janeiro/Rio de Janeiro/Industry"    
#> [5] "São Paulo/Campinas/Agriculture"            
#> [6] "São Paulo/Campinas/Industry"               
#> [7] "São Paulo/São Paulo/Agriculture"           
#> [8] "São Paulo/São Paulo/Industry"

# Aggregated series across all levels
hts$aggregated_series()
#>             state            city       sector quarter   gdp
#> 1    <aggregated>    <aggregated> <aggregated> 2024 Q1 11520
#> 2    <aggregated>    <aggregated> <aggregated> 2024 Q2 12320
#> 3    <aggregated>    <aggregated>     Industry 2024 Q2  5500
#> 4    <aggregated>    <aggregated>     Industry 2024 Q1  5070
#> 5    <aggregated>    <aggregated>  Agriculture 2024 Q2  6820
#> 6    <aggregated>    <aggregated>  Agriculture 2024 Q1  6450
#> 7  Rio de Janeiro    <aggregated> <aggregated> 2024 Q1  4420
#> 8       São Paulo    <aggregated> <aggregated> 2024 Q1  7100
#> 9       São Paulo    <aggregated> <aggregated> 2024 Q2  7480
#> 10 Rio de Janeiro    <aggregated> <aggregated> 2024 Q2  4840
#> 11      São Paulo    <aggregated>     Industry 2024 Q1  3300
#> 12      São Paulo    <aggregated>     Industry 2024 Q2  3490
#> 13      São Paulo    <aggregated>  Agriculture 2024 Q1  3800
#> 14 Rio de Janeiro    <aggregated>  Agriculture 2024 Q2  2830
#> 15      São Paulo    <aggregated>  Agriculture 2024 Q2  3990
#> 16 Rio de Janeiro    <aggregated>     Industry 2024 Q1  1770
#> 17 Rio de Janeiro    <aggregated>  Agriculture 2024 Q1  2650
#> 18 Rio de Janeiro    <aggregated>     Industry 2024 Q2  2010
#> 19 Rio de Janeiro  Rio de Janeiro <aggregated> 2024 Q2  4150
#> 20      São Paulo       São Paulo <aggregated> 2024 Q2  6200
#> 21      São Paulo        Campinas <aggregated> 2024 Q2  1280
#> 22      São Paulo       São Paulo <aggregated> 2024 Q1  5900
#> 23 Rio de Janeiro  Rio de Janeiro <aggregated> 2024 Q1  3800
#> 24      São Paulo        Campinas <aggregated> 2024 Q1  1200
#> 25 Rio de Janeiro Duque de Caxias <aggregated> 2024 Q1   620
#> 26 Rio de Janeiro Duque de Caxias <aggregated> 2024 Q2   690
#> 27      São Paulo        Campinas  Agriculture 2024 Q1   700
#> 28 Rio de Janeiro Duque de Caxias     Industry 2024 Q1   270
#> 29 Rio de Janeiro Duque de Caxias     Industry 2024 Q2   310
#> 30      São Paulo        Campinas     Industry 2024 Q1   500
#> 31 Rio de Janeiro Duque de Caxias  Agriculture 2024 Q1   350
#> 32      São Paulo       São Paulo     Industry 2024 Q1  2800
#> 33 Rio de Janeiro  Rio de Janeiro     Industry 2024 Q1  1500
#> 34      São Paulo       São Paulo  Agriculture 2024 Q1  3100
#> 35      São Paulo       São Paulo  Agriculture 2024 Q2  3250
#> 36      São Paulo        Campinas     Industry 2024 Q2   540
#> 37 Rio de Janeiro Duque de Caxias  Agriculture 2024 Q2   380
#> 38 Rio de Janeiro  Rio de Janeiro  Agriculture 2024 Q1  2300
#> 39 Rio de Janeiro  Rio de Janeiro  Agriculture 2024 Q2  2450
#> 40      São Paulo       São Paulo     Industry 2024 Q2  2950
#> 41 Rio de Janeiro  Rio de Janeiro     Industry 2024 Q2  1700
#> 42      São Paulo        Campinas  Agriculture 2024 Q2   740
```

### From CSV

``` r
path <- system.file("extdata", "tourism.csv", package = "rhts")

spec <- HtsSpec$new(c("Region", "State"), "Purpose")
hts <- Hts$from_csv(path, spec, "Quarter", "Trips")

hts$print()
#> Hierarchical Time Series Summary
#> ================================
#> Total series:  765
#> Bottom series: 304
#> Time periods:  80
#> Hierarchy:     ["Region", "State"]
#> Groups:        ["Purpose"]
#> S matrix:      765 × 304
#> NULL
```

## Data requirements

- Bottom-level data must contain all hierarchy and group columns, plus a
  time column and a numeric value column.
- Time strings can be Annual (“2024”), Quarterly (“2024 Q1”), Monthly
  (“2024 M01”), Weekly (“2024 W01”), or Daily (“2024-01-01”).

## Related project

The R package is backed by the Rust crates in
[`hts-rs`](https://github.com/albersonmiranda/hts-rs).
