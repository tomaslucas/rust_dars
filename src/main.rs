use polars::prelude::*;
use rand::{thread_rng, Rng};
use std::path::Path;

fn main() {
    // Configuro el número de filas y columnas a mostrar
    // https://docs.rs/polars/latest/polars/
    std::env::set_var("POLARS_FMT_MAX_ROWS", "-1".to_string());
    std::env::set_var("POLARS_FMT_MAX_COLS", "-1".to_string());
    let mut arr = [0f64; 5];
    thread_rng().fill(&mut arr);

    let df = df! (
        "nrs" => &[Some(1), Some(2), Some(3), None, Some(5)],
        "names" => &[Some("foo"), Some("ham"), Some("spam"), Some("eggs"), None],
        "random" => &arr,
        "groups" => &["A", "A", "B", "C", "B"],
    )
    .unwrap();

    let df1 = df
        .clone()
        .lazy()
        .sort_by_exprs(
            vec![col("nrs"), col("random")],
            vec![true, true],
            false,
            false,
        )
        .collect()
        .unwrap();

    println!("{}", df1);

    let out = df
        .clone()
        .lazy()
        .select([
            sum("nrs"),
            col("names").sort(false),
            col("names").first().alias("first name"),
            (mean("nrs") * lit(10)).alias("10xnrs"),
        ])
        .collect()
        .unwrap();
    println!("{}", out);
    //dataf();
    //series_null();

    //let s = Series::new("data", &[Some(1.0), None, Some(3.0), Some(4.0)]);

    let s: Series = Series::new("data", vec![Some(1f32), None, Some(3.0), Some(4.0)]);
    println!("{:?}", s);
    // let s = s.f64().unwrap().fill_null_with_values(42.);
    println!("{:?}", s.f32().unwrap().fill_null_with_values(42.));
    let s = s
        .f32()
        .unwrap()
        .fill_null_with_values(44.)
        .unwrap()
        .into_series();
    // ChunkFillNullValue(42));
    println!("{:?}", s);

    let flights_file_path: &Path = Path::new("./dataset/Flight_on_time_HIX.csv");
    let columns = [
        "Airline",
        "Origin_Airport",
        "Destination_Airport",
        "Departure_Delay_Minutes",
        "Arrival_Delay_Minutes",
    ];
    let flights_df: DataFrame = read_data_frame_from_csv(flights_file_path)
        .select(columns)
        .unwrap();
    println!("{:?}", flights_df.head(Some(5)));

    let arr_delay_mean_df: DataFrame = flights_df
        .clone()
        .lazy()
        .group_by(["Airline"])
        .agg([
            col("Arrival_Delay_Minutes")
                .mean()
                .round(3)
                .alias("Arrival_Delay"),
            col("Departure_Delay_Minutes")
                .mean()
                .round(3)
                .alias("Departure_Delay"),
        ])
        .sort_by_exprs(
            vec![col("Arrival_Delay"), col("Departure_Delay")],
            vec![true, true],
            false,
            false,
        )
        .collect()
        .unwrap();

    println!("{:?}", arr_delay_mean_df.shape().0);
    println!(
        "{}",
        arr_delay_mean_df.head(Some(arr_delay_mean_df.shape().0))
    );
}

fn read_data_frame_from_csv(csv_file_path: &Path) -> DataFrame {
    CsvReader::from_path(csv_file_path)
        .expect("Cannot open file.")
        .has_header(true)
        .finish()
        .unwrap()
}
