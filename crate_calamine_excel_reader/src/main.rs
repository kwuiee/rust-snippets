use calamine::{open_workbook, DataType, Reader, Xlsx};

fn main() {
    let path = "simple1.xlsx";

    // opens a new workbook
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

    // Read whole worksheet data and provide some statistics
    if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
        let total_cells = range.get_size().0 * range.get_size().1;
        let non_empty_cells: usize = range.used_cells().count();
        println!(
            "Found {} cells in 'Sheet1', including {} non empty cells",
            total_cells, non_empty_cells
        );
        // alternatively, we can manually filter rows
        assert_eq!(
            non_empty_cells,
            range
                .rows()
                .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty))
                .count()
        );
    }

    // Check if the workbook has a vba project
    if let Some(Ok(mut vba)) = workbook.vba_project() {
        let vba = vba.to_mut();
        let module1 = vba.get_module("Module 1").unwrap();
        println!("Module 1 code:");
        println!("{}", module1);
        for r in vba.get_references() {
            if r.is_missing() {
                println!("Reference {} is broken or not accessible", r.name);
            }
        }
    }

    // You can also get defined names definition (string representation only)
    for name in workbook.defined_names() {
        println!("name: {}, formula: {}", name.0, name.1);
    }

    // Now get all formula!
    let sheets = workbook.sheet_names().to_owned();
    for s in sheets {
        println!(
            "found {} formula in '{}'",
            workbook
                .worksheet_formula(&s)
                .expect("sheet not found")
                .expect("error while getting formula")
                .rows()
                .flat_map(|r| r.iter().filter(|f| !f.is_empty()))
                .count(),
            s
        );
    }
}
