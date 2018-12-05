use calamine::{Reader, Range, DataType, open_workbook, Xlsx};

/// Holds the column diff of a specific sheet
struct Diff {
    /// name of the sheet
    name: String,
    added: Vec<String>,
    removed: Vec<String>
}

#[derive(Debug, PartialEq)]
struct Sheet {
    name: String,
    columns: Vec<String>,
}

pub fn diff_workbooks(path_left: &str, path_right: &str) {
    let workbook_left: Xlsx<_> = open_workbook(path_left)
        .expect(&format!("The workbook '{}' could not be loaded", path_left));
    let workbook_right: Xlsx<_> = open_workbook(path_right)
        .expect(&format!("The workbook '{}' could not be loaded", path_right));

    let left_sheets: Vec<Sheet> = build_sheets(workbook_left);
    let right_sheets: Vec<Sheet> = build_sheets(workbook_right);

    println!("left_sheets: {:?}", left_sheets);
    println!("right_sheets: {:?}", right_sheets);
}

/// Iterates over all sheets in the workbook and returns a Vec of sheets with the different
/// columns
fn build_sheets<T: Reader>(mut workbook: T) -> Vec<Sheet> {
    let mut sheets: Vec<Sheet> = vec![];

    for sheet in workbook.sheet_names().to_vec() {
        if let Some(Ok(range)) = workbook.worksheet_range(&sheet) {
            let columns: Vec<String> = first_row_of_range(range).to_vec();
            sheets.push(
                Sheet { name: sheet.to_string(), columns }
            );
        }
    }
    sheets
}

fn first_row_of_range(range: Range<DataType>) -> Vec<String> {
    if let Some(first_row) = range.rows().nth(0) {
        return first_row.iter().map(|i| i.to_string()).collect();
    } else {
        panic!("sheet is empty");
    };
}

#[test]
#[should_panic(expected = "The workbook 'abc' could not be loaded")]
fn test_with_incorrect_left_workbook() {
    diff_workbooks("abc", "def");
}

#[test]
#[should_panic(expected = "The workbook 'def' could not be loaded")]
fn test_with_incorrect_right_workbook() {
    diff_workbooks("./tests/auxiliary/test_left.xlsx", "def");
}

#[test]
fn test_with_two_existing_workbooks() {
    diff_workbooks(
        "./tests/auxiliary/test_left.xlsx",
        "./tests/auxiliary/test_right.xlsx"
    );
}

#[test]
fn test_build_sheets() {
    let workbook: Xlsx<_> = open_workbook("./tests/auxiliary/test_left.xlsx")
        .expect(&format!("The test workbook could not be loaded"));
    let result = build_sheets(workbook);
    let expected = vec![
        Sheet {
            name: "Things".to_string(),
            columns: vec![
                "id".to_string(),
                "description".to_string(),
                "the_title".to_string(),
                "time".to_string()
            ]
        }
    ];
    assert_eq!(result, expected);
}
