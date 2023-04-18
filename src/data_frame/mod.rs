use super::*;
use wasm_bindgen::prelude::*;
use std::fmt::Write;

#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub struct DataFrame {
    columns: Vec<String>,
    data: Vec<Vec<f64>>,
}

impl DataFrame {
    pub fn get_num_columns(&self) -> usize {
        // Assume all cols are the same length.
        self.data[0].len()
    }

    pub fn get_title(&self, index: usize) -> &String {
        &self.columns[index]
    }

    // implementation of the DataFrame struct
    pub fn get_column(&self, index: usize) -> Option<Vec<f64>> {
        let column_values: Vec<f64> = self.data.iter().map(|row| row[index]).collect();
        Some(column_values)
    }
}

pub fn matrix_coefficient(frame: &DataFrame) -> Vec<Vec<f64>> {
    let len = frame.get_num_columns();
    let mut results = Vec::new();
    for x in 0..len - 1 {
        let mut result_col = Vec::new();
        let col1 = frame.get_column(x).unwrap();
        for y in x + 1..len {
            let col2 = frame.get_column(y).unwrap();
            let result = pearson_correlation_coefficient(&col1, &col2);
            result_col.push(result);
        }
        results.push(result_col);
    }
    results
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
                s.trim().parse().map_err(|e| {
                    format!(
                        "Invalid float literal at line {}, column {}: {}",
                        i + 2,
                        j + 1,
                        e
                    )
                })
            })
            .collect();

        let values = values?;

        if values.len() != column_count {
            return Err(format!("Mismatch in column count at line {}", i + 2));
        }

        data.push(values);
    }

    Ok(DataFrame { columns, data })
}


pub fn scatter_plot_svg(x: &[f64], y: &[f64], width: u32, height: u32) -> String {
    let mut svg = String::new();

    writeln!(svg, "<svg width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">", width, height, width, height).unwrap();
    writeln!(svg, "<g transform=\"translate(50, {}) scale(1, -1)\">", height - 50).unwrap();

    let x_min = x.iter().copied().fold(f64::INFINITY, f64::min);
    let x_max = x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let y_min = y.iter().copied().fold(f64::INFINITY, f64::min);
    let y_max = y.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    let x_range = x_max - x_min;
    let y_range = y_max - y_min;

    let x_scale = (width - 100) as f64 / x_range;
    let y_scale = (height - 100) as f64 / y_range;

    // Draw the x-axis
    writeln!(svg, "<line x1=\"50\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" stroke-width=\"2\" />", height - 50, width - 50, height - 50).unwrap();

    // Draw the y-axis
    writeln!(svg, "<line x1=\"50\" y1=\"{}\" x2=\"50\" y2=\"{}\" stroke=\"black\" stroke-width=\"2\" />", height - 50, 50).unwrap();

    for (x_val, y_val) in x.iter().zip(y.iter()) {
        let x_pos = ((x_val - x_min) * x_scale) as u32 + 50;
        let y_pos = ((y_val - y_min) * y_scale) as u32 + 50;

        writeln!(svg, "<circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"blue\" stroke=\"black\" opacity=\"0.3\" stroke-width=\"1\" />", x_pos, y_pos).unwrap();
    }

    writeln!(svg, "</g>").unwrap();
    writeln!(svg, "</svg>").unwrap();

    svg
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
    fn test_dataframe_num_cols() {
        let df = DataFrame {
            columns: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            data: vec![
                vec![1.0, 2.0, 3.0],
                vec![4.0, 5.0, 6.0],
                vec![7.0, 8.0, 9.0],
                vec![7.0, 8.0, 9.0],
            ],
        };
        assert_eq!(df.get_num_columns(), 3);
    }

    #[test]
    fn test_get_col() {
        let df = DataFrame {
            columns: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            data: vec![
                vec![1.0, 2.0, 3.0],
                vec![4.0, 5.0, 6.0],
                vec![7.0, 8.0, 9.0],
                vec![7.0, 8.0, 9.0],
            ],
        };
        assert_eq!(df.get_column(0), Some(vec![1.0, 4.0, 7.0, 7.0]));
        assert_eq!(df.get_column(2), Some(vec![3.0, 6.0, 9.0, 9.0]));
    }

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
            data: vec![vec![25.0, 60.5], vec![30.0, 75.2]],
        };
        assert_eq!(parse_csv(input), Ok(expected));
    }

    #[test]
    fn test_parse_csv_empty_input() {
        let input = "";
        assert_eq!(
            parse_csv(input),
            Err("CSV input has no columns".to_string())
        );
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

    #[test]
    fn test_matrix_coefficient() {
        let df = DataFrame {
            columns: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            data: vec![
                vec![1.0, 0.0, 1.0],
                vec![4.0, 0.0, 4.0],
                vec![7.0, 0.0, 7.0],
                vec![7.0, 0.0, 7.0],
            ],
        };
        let result = matrix_coefficient(&df);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0][0], 0.0);
        assert_eq!(result[0][1], 1.0);
        assert_eq!(result[1][0], 0.0);
    }

   // TODO mod for CSV nd svg HTML etc
   // better tests
}
