use excel_column_diff::diff_workbooks;

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
