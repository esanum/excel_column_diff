use std::env;
use excel_column_diff::diff_workbooks;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3, "must provide two workbooks");

    diff_workbooks(&args[1], &args[2])
}
