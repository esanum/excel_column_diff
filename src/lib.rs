use calamine::{Reader, CellType, open_workbook, Xlsx};

/// Holds the column diff of a specific sheet
struct Diff {
    /// name of the sheet
    name: String,
    added: Vec<String>,
    removed: Vec<String>
}

pub fn diff_workbooks(path_left: &str, path_right: &str) {
    let mut workbook_left: Xlsx<_> = open_workbook(path_left)
        .expect(&format!("The workbook '{}' could not be loaded", path_left));
    // let mut workbook_right: Xlsx<_> = open_workbook(path_right)
    //     .expect(&format!("The workbook '{}' could not be loaded", path_right));

    for sheet in &mut workbook_left.sheet_names().to_vec() {
        if let Some(Ok(range)) = workbook_left.worksheet_range(sheet) {
            let wat = first_row_of_range(&range);
            println!("row: {:?}", wat);
        }
    }
}

fn first_row_of_range<'a, T: CellType>(range: &'a calamine::Range<T>) -> &'a [T] {
    if let Some(first_row) = range.rows().nth(0) {
        return first_row;
    } else {
        panic!("sheet is empty");
    };
}
