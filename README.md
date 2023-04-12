## City Job Position Counter
City Job Position Counter is a Rust web application that calculates the total number of job positions in specific cities based on a CSV file containing job data. The application is built using the Actix web framework for Rust.

## Features
Calculates the total number of job positions in three cities: New York (NY), Chicago (CHI), and San Francisco (SF).
Reads job data from a CSV file with the following columns: job title, company, and location.
Ignores case sensitivity when matching city names.
Displays the results in a fancy HTML format with hyperlinks for each city.

## CSV File Format
The application expects the job data to be in a CSV file with the following format:

csv

title,company,location
Software Engineer,Acme Inc,New York
Data Scientist,XYZ Corp,Chicago
...
The first row should contain the column names: "title", "company", and "location".
Subsequent rows should contain the job data, with one job per row.
The "location" column should contain the city name where the job is located.

## Serverless Deployment
To deploy the City Job Position Counter application, you can use Cargo Shuttle, a tool for easily deploying Rust projects. Here are the steps to deploy using Cargo Shuttle:

Install Cargo Shuttle by running the following command:
```sh
cargo install cargo-shuttle
