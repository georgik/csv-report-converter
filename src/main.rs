use clap::Parser;
use csv::Reader;
use std::collections::HashMap;
use std::error::Error;
use urlencoding::decode;

#[derive(Parser)]
#[command(name = "CSV Report Processor")]
#[command(about = "Processes a CSV file to produce a filtered work report", long_about = None)]
struct Cli {
    /// Sets the input CSV file to use
    #[arg(value_parser)]
    csv_file: String,

    /// Filters records by a specific date
    #[arg(long, value_name = "DATE")]
    date: Option<String>,

    /// Filters records by a specific author
    #[arg(long, value_name = "AUTHOR")]
    author: Option<String>,
}

struct Record {
    project: String,
    author: String,
    date: String,
    report: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut reader = Reader::from_path(cli.csv_file)?;

    let mut grouped_records: HashMap<String, Vec<Record>> = HashMap::new();

    for result in reader.records() {
        let record = result?;
        let project = record.get(0).unwrap_or_default();
        let author = record.get(1).unwrap_or_default();
        let date = record.get(2).unwrap_or_default();
        let report_encoded = record.get(3).unwrap_or_default();

        // Apply date filter if set
        if let Some(ref filter) = cli.date {
            if date != filter {
                continue;
            }
        }

        // Apply author filter if set
        if let Some(ref filter) = cli.author {
            if author != filter {
                continue;
            }
        }

        // URL-decode the report
        let report = decode(report_encoded).expect("UTF-8").to_string();

        let r = Record {
            project: project.to_string(),
            author: author.to_string(),
            date: date.to_string(),
            report,
        };

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

    // Generate the HTML table using the sorted and grouped records
    for project_name in project_names {
        if let Some(records) = grouped_records.get(&project_name) {
            for record in records {
                html_output.push_str("<tr>");
                html_output.push_str(&format!("<td>{}</td>", record.project));
                html_output.push_str(&format!("<td>{}</td>", record.author));
                html_output.push_str(&format!("<td>{}</td>", record.date));
                html_output.push_str(&format!("<td>{}</td>", record.report));
                html_output.push_str("</tr>\n");
            }
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
