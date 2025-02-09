# LogAnalyzer

A high-performance log analysis system written in Rust that helps process and analyze large-scale application logs.

## Features

- Multi-format log ingestion (JSON, CSV, plain text)
- Parallel processing using Rayon
- Structured data extraction
- Multiple input source support (files, network streams)

## Project Structure

# logsAnalyzer
The log analysis system helps make sense of the data ( data here corresponds to the massive log files containing information about everything that happens in the servers , databases etc.. )


![Logs Analyzer](./assets/logAnalyser_design.png)