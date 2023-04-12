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

1. Install Cargo Shuttle by running the following command:
```sh
cargo install cargo-shuttle
```

2. Login to your Cargo Shuttle account by running the following command：
```sh
cargo shuttle login
3. Authenticate your account by providing your API key:
```sh
cargo shuttle login --api-key YOURAPI
4. Initialize your project by running the following command:
```sh
cargo shuttle init
5. deploy: run the command to deploy the project
```sh
cargo shuttle deploy
6. If the code is existed, use the following code to new a project
```sh
cargo shuttle project new
7. Deploy and run
6. If the code is existed, use the following code to new a project
```sh
cargo shuttle deploy --allow-dirty

service is available at {project_name}.shuttleapp.rs.
![image](https://user-images.githubusercontent.com/122952572/231333896-8b7ab6be-a5a3-447c-b0f7-d3a2da7cfb4a.png)


<img width="653" alt="6e9a67f99190580969f670ffe5da9ba" src="https://user-images.githubusercontent.com/122952572/231333940-7f3b0abc-6556-4882-8e95-e725265b9aee.png">
<img width="505" alt="46503c77a4a2232bceec438d677ad13" src="https://user-images.githubusercontent.com/122952572/231333975-9c4c5e15-05ee-43e2-9c64-75bed6e38d60.png">
<img width="489" alt="62600410bf7ed6ee734e61afec6772e" src="https://user-images.githubusercontent.com/122952572/231333989-20bc00ff-3835-4e6f-ac34-3b94d1240384.png">
<img width="527" alt="6f825eb398280dafba2c2a76911fcf8" src="https://user-images.githubusercontent.com/122952572/231334006-f7573216-9be1-4394-bb0d-1ea6fcde3f34.png">


