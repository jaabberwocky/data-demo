use polars::prelude::*;

fn main() {
    let df = CsvReader::from_path("examples/data.csv")
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
}
