use calamine::{Reader, open_workbook, Xlsx};

pub fn diff_workbooks(path_left: &str, path_right: &str) {
    let mut workbook_left: Xlsx<_> = open_workbook(path_left)
        .expect(&format!("The workbook '{}' could not be loaded", path_left));
    let mut workbook_right: Xlsx<_> = open_workbook(path_right)
        .expect(&format!("The workbook '{}' could not be loaded", path_right));
}
