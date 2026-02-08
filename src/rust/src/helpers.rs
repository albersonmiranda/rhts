use extendr_api::prelude::*;
use polars::prelude::*;

// convert R dataframe to polars dataframe
pub fn from_robj(robj: Robj) -> Result<DataFrame, Error> {
    // check if robj is a dataframe
    if !robj.is_frame() {
        return Err(Error::from("robj must be a data.frame"));
    }

    // get columns names
    let names = robj
        .names()
        .ok_or(Error::from("Failed to get column names"))?;

    // get columns as list
    let list = robj
        .as_list()
        .ok_or(Error::from("Failed to convert robj to list"))?;

    // convert to polars series
    let mut series_vec = Vec::new();
    for (name, col) in names.zip(list.values()) {
        let s = if col.is_real() {
            let v = col
                .as_real_vector()
                .ok_or(Error::from(format!("Failed to convert {} to f64", name)))?;
            Series::new(name.into(), v)
        } else if col.is_integer() {
            let v = col
                .as_integer_vector()
                .ok_or(Error::from(format!("Failed to convert {} to i32", name)))?;
            Series::new(name.into(), v)
        } else if col.is_string() || col.is_factor() {
            let v = col
                .as_string_vector()
                .ok_or(Error::from(format!("Failed to convert {} to string", name)))?;
            Series::new(name.into(), v)
        } else {
            return Err(Error::from(format!("Unsupported column type: {}", name)));
        };
        series_vec.push(s.into());
    }

    // create polars dataframe
    let df = DataFrame::new(series_vec).map_err(|e| Error::from(e.to_string()))?;

    Ok(df)
}

pub fn to_robj(df: DataFrame) -> Robj {
    let mut r_cols = Vec::new();
    for s in df.get_columns() {
        let name = s.name().to_string();
        let robj = if s.dtype().is_float() {
            Robj::from(s.f64().unwrap().into_iter().map(|v| v.unwrap_or(f64::NAN)).collect::<Vec<_>>())
        } else if s.dtype().is_integer() {
            Robj::from(s.i32().unwrap().into_iter().map(|v| v.unwrap_or(i32::MIN)).collect::<Vec<_>>())
        } else if s.dtype().is_string() {
            Robj::from(s.str().unwrap().into_iter().map(|v| v.unwrap_or("").to_string()).collect::<Vec<_>>())
        } else if s.dtype().is_date() {
            Robj::from(s.date().unwrap().into_iter().map(|v| v.unwrap_or(0)).collect::<Vec<_>>())
        } else {
            Robj::from(())
        };
        r_cols.push((name, robj));
    }

    let names: Vec<String> = r_cols.iter().map(|(name, _)| name.clone()).collect();
    let values: Vec<Robj> = r_cols.into_iter().map(|(_, col)| col).collect();

    let mut result = List::from_values(values);
    result.set_names(&names).unwrap();
    
    data_frame!(result)
}