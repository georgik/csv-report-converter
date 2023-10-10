use csv::Reader;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use urlencoding::decode;

struct Record {
    project: String,
    author: String,
    date: String,
    report: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get the filename from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <csv_file> <date_filter>", args[0]);
        return Ok(());
    }
    let file_name = &args[1];
    let date_filter = &args[2];

    // Create a new CSV reader from the given file
    let mut reader = Reader::from_path(file_name)?;

    // Start the HTML5 document
    let mut html_output = String::from("<!DOCTYPE html>\n");
    html_output.push_str("<html lang=\"en\">\n");
    html_output.push_str("<head>\n");
    html_output.push_str("<meta charset=\"UTF-8\">\n");
    html_output.push_str("<title>Weekly Work Report</title>\n");
    html_output.push_str("</head>\n");
    html_output.push_str("<body>\n");

    // Start the HTML table
    html_output.push_str("<table border='1'>\n");
    html_output.push_str("<tr><th>Project</th><th>Author</th><th>Date</th><th>Report</th></tr>\n");

    let mut grouped_records: HashMap<String, Vec<Record>> = HashMap::new();

    for result in reader.records() {
            let record = result?;
            let project: &str = &record[0];
            let author: &str = &record[1];
            let date: &str = &record[2];
            let report_encoded: &str = &record[3];

            // URL-decode the report
            let report = decode(report_encoded).expect("UTF-8").to_string();

            let r = Record {
                project: project.to_string(),
                author: author.to_string(),
                date: date.to_string(),
                report,
            };

            if date != date_filter {
                continue;
            }

            grouped_records
                .entry(project.to_string())
                .or_insert_with(Vec::new)
                .push(r);
        }

        // Sort the project names, ensuring that "Rust" is first
        let mut project_names: Vec<String> = grouped_records.keys().cloned().collect();
        project_names.sort_by(|a, b| {
            if a == "Rust" {
                std::cmp::Ordering::Less
            } else if b == "Rust" {
                std::cmp::Ordering::Greater
            } else {
                a.cmp(b)
            }
        });

    // Generate the HTML table using the sorted and grouped records
    for project_name in project_names {
        let records = grouped_records.get(&project_name).unwrap();
        for record in records {
            html_output.push_str("<tr>");
            html_output.push_str(&format!("<td>{}</td>", record.author));
            html_output.push_str(&format!("<td>{}</td>", record.project));
            html_output.push_str(&format!("<td>{}</td>", record.date));
            html_output.push_str(&format!("<td>{}</td>", record.report));
            html_output.push_str("</tr>\n");
        }
    }

    // End the HTML table
    html_output.push_str("</table>\n");

    // End the HTML5 document
    html_output.push_str("</body>\n");
    html_output.push_str("</html>\n");

    // Output the HTML
    println!("{}", html_output);

    Ok(())
}
