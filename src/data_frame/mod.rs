use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub struct DataFrame {
    columns: Vec<String>,
    data: Vec<Vec<f64>>,
}

#[wasm_bindgen]
pub fn parse_csv(input: &str) -> Result<DataFrame, String> {
    let mut lines = input.lines();

    // Parse column headers
    let columns: Vec<String> = match lines.next() {
        Some(line) => line.split(',').map(|s| s.to_string()).collect(),
        None => return Err("CSV input has no columns".to_string()),
    };
    let column_count = columns.len();

    // Parse data lines
    let mut data = Vec::new();
    for (i, line) in lines.enumerate() {
        let values: Result<Vec<f64>, String> = line
            .split(',')
            .enumerate()
            .map(|(j, s)| {
                s.trim()
                    .parse()
                    .map_err(|e| format!("Invalid float literal at line {}, column {}: {}", i+2, j+1, e))
            })
            .collect();

        let values = values?;

        if values.len() != column_count {
            return Err(format!("Mismatch in column count at line {}", i+2));
        }

        data.push(values);
    }

    Ok(DataFrame { columns, data })
}

#[wasm_bindgen]
pub fn dataframe_to_html_table(df: &DataFrame) -> String {
    let mut table_html = String::new();

    // Start the table element and add column headers
    table_html.push_str("<table>");
    table_html.push_str("<thead><tr>");
    for col in &df.columns {
        table_html.push_str(&format!("<th>{}</th>", col));
    }
    table_html.push_str("</tr></thead>");

    // Add data rows
    table_html.push_str("<tbody>");
    for row in &df.data {
        table_html.push_str("<tr>");
        for val in row {
            table_html.push_str(&format!("<td>{}</td>", val));
        }
        table_html.push_str("</tr>");
    }
    table_html.push_str("</tbody>");

    // End the table element and return the HTML string
    table_html.push_str("</table>");
    table_html
}


#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn test_dataframe_to_html_table() {
		let df = DataFrame {
			columns: vec!["A".to_string(), "B".to_string(), "C".to_string()],
			data: vec![
				vec![1.0, 2.0, 3.0],
				vec![4.0, 5.0, 6.0],
				vec![7.0, 8.0, 9.0],
			],
		};

		let expected_html = "<table><thead><tr><th>A</th><th>B</th><th>C</th></tr></thead><tbody><tr><td>1</td><td>2</td><td>3</td></tr><tr><td>4</td><td>5</td><td>6</td></tr><tr><td>7</td><td>8</td><td>9</td></tr></tbody></table>".to_string();
		assert_eq!(dataframe_to_html_table(&df), expected_html);
	}

    #[test]
    fn test_parse_csv() {
        let input = "age,weight\n25,60.5\n30,75.2\n";
        let expected = DataFrame {
            columns: vec!["age".to_string(), "weight".to_string()],
            data: vec![
                vec![25.0, 60.5],
                vec![30.0, 75.2],
            ],
        };
        assert_eq!(parse_csv(input), Ok(expected));
    }

    #[test]
    fn test_parse_csv_empty_input() {
        let input = "";
        assert_eq!(parse_csv(input), Err("CSV input has no columns".to_string()));
    }

    #[test]
    fn test_parse_csv_empty_line() {
        let input = "age,weight\n25,60.5\n1\n30,75.2\n";
        assert_eq!(
            parse_csv(input), 
            Err("Mismatch in column count at line 3".to_string())
        );
    }

    #[test]
    fn test_parse_csv_invalid_input() {
        let input = "age,weight\n25,60.5\nthirty,75.2\n";
        assert_eq!(
            parse_csv(input),
            Err("Invalid float literal at line 3, column 1: invalid float literal".to_string())
        );
    }
}
