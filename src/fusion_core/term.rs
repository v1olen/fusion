use tabular::{Row, Table};

pub fn make_vec_printable<T>(strings: Vec<T>) -> String
where
    T: ToString,
{
    let strings: Vec<String> = strings
        .into_iter()
        .map(|string| string.to_string())
        .collect();
    let (term_width, _) = term_size::dimensions().unwrap();
    let dspace_separated_strings: String = strings.join("  ");

    if dspace_separated_strings.len() < term_width {
        return dspace_separated_strings;
    } else {
        let largest_size = strings.iter().fold(0, |acc, element| {
            if element.len() > acc {
                element.len()
            } else {
                acc
            }
        });
        let columns = (term_width / (largest_size + 2)) as u8;
        let table_structure = (0..columns).map(|_| "{:<}  ").collect::<String>();
        let mut table = Table::new(table_structure.as_str());
        let rows = (strings.len() as f32 / columns as f32).ceil() as u8;
        for row in 0..rows {
            let mut table_row = Row::new();
            for cell in 0..columns {
                let index: usize = cell as usize + row as usize * columns as usize;
                table_row = if index + 1 >= strings.len() {
                    table_row.with_cell("")
                } else {
                    table_row.with_cell(strings[index].clone())
                };
            }
            table.add_row(table_row);
        }
        format!("{}", table)
    }
}
