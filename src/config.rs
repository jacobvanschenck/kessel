use crate::chart::Chart;
use crate::pdf::create_pdf;
use std::error::Error;
use std::fs;

pub struct Config {
    pub input_file: String,
    pub output_file: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let input_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an input file."),
        };

        let output_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an output file."),
        };

        Ok(Config {
            input_file,
            output_file,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Input file: {}", config.input_file);
    println!("Output file: {}", config.output_file);

    let contents = fs::read_to_string(config.input_file)?;
    let sections = contents.split("\n\n");

    let mut chart = Chart::build();

    sections.for_each(|s| chart.parse_section(s));

    create_pdf(chart);
    Ok(())
}
