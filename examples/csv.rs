use anyhow::Result;
use polars::prelude::*;

fn main() {
    let df: DataFrame = CsvReader::from_path("data/data.csv")
        .unwrap()
        .finish()
        .unwrap();
    println!("{}", df.head(Some(5)));

    let res = df
        .clone()
        .lazy()
        .group_by(["score_phrase"])
        .agg(vec![col("score_phrase").count().alias("count")])
        .sort(
            "count",
            SortOptions {
                descending: true,
                ..Default::default()
            },
        )
        .collect();
    println!("{:?}", res);

    write_dataframe_to_csv(res.unwrap(), "data/data_grouped.csv").unwrap();
}

fn write_dataframe_to_csv(mut df: DataFrame, path: &str) -> Result<()> {
    let mut file = std::fs::File::create(path).unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();
    Ok(())
}
