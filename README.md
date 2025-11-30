# How to run

## Requirements
### Docker
To run the project easily , you will need to install docker on your computer , here is the required documentation to install the requirements :
[Docker installation guide for linux](https://docs.docker.com/engine/install/)
[Docker installation guide for mac](https://docs.docker.com/desktop/setup/install/mac-install/)
[Docker installation guide for windows](https://docs.docker.com/desktop/setup/install/windows-install/)
To fetch required images to run the project you will need a working internet connection , or the images mentioned in [the docker compose file](docker-compose.yaml) which are :
- [postgres:16](https://hub.docker.com/_/postgres)
- [redis:7](https://hub.docker.com/_/redis)
- [python:3.12-slim](https://hub.docker.com/_/python)
- [rust:1.83](https://hub.docker.com/_/rust)
- [debian:stable-slim](https://hub.docker.com/_/debian)
Also , The port 8080 must be free on your system , if you want to change that , feel free to change the values on [docker-compose.yaml](docker-compose.yaml#)
## Architecture
 <img src="architecture serveur.png">

## How to run
You need to run on the root of the project this command : 
```
docker compose up -d --build
```
or 
```
docker-compose up -d --build
```
To get the data from the rust api , you might want to use postman , or curl using this command :
```
curl "http://localhost:8080/prices/average?start_date=2025-01-06&end_date=2025-01-07"
```
a swagger is setup for both python-api and rust-api , you can access it via :
rust swagger : http://localhost:8080/swagger-ui/
python swagger : http://localhost:8000/swagger-ui/


## How does it work
### Rust api
Rust was used to create a basic API , accessible to the public , it uses the Python api route to get the average prices between the two dates given by the route
### Python api
The python-api is an internal-only FastAPI service.
a swagger page is accessible on http://HOST:PORT/docs
when calling its route , it creates a task on Redis and waits for it to be finished inside the same http request.
Dealing with the calculation in the same http request was used because of how fast and easy calculation were , Choosing to use a Result retrieval endpoint would have been a good option of calculation were taking a long time 
### Celery
Celery was used to treat the tasks created by the python api , and return the result to redis
### Redis 
Redis is used to create a queue of tasks for celery to run , and provide the results to the python api


# Project specification

# Technical Test for Backend Application (Quantfox)

## Objective

The objective of this technical test is to evaluate your ability to design a complete backend system involving multiple microservices, asynchronous computation, correct data handling, robust validation and containerization. You are expected to deliver a Rust HTTP API using Actix Web and Tokio, a Python HTTP API using Celery for background computation, and a fully functional Docker Compose environment that includes all services, including the database. Every configuration file used in this project must be written in TOML format.

## Overall Architecture

The system must consist of two main services. The first one is a public Rust service named “rust-api”. This service exposes an HTTP endpoint, validates incoming query parameters, and forwards valid requests to a second service named “python-api”. The second service receives the validated dates, reads pricing data from a database and computes one average price per day within the specified date range. This computation must be optimized, preferably by performing the aggregation directly in the database instead of processing large datasets in Python.

The python-api must integrate Celery, and the daily average computation must be implemented as a Celery task. You may wait for the task result directly inside the HTTP route handler or implement a retrieval endpoint, but your choice must be clearly explained in your documentation.

The architecture must run entirely through Docker Compose, including the Rust service, the Python service, the Celery worker, the Celery broker (such as Redis), and the database that stores the pricing data. When the reviewer starts the project, the database must already be available and ready to serve queries.

## Rust Service Requirements

The Rust service will expose an HTTP endpoint whose purpose is to receive two date parameters, named start_date and end_date, formatted as ISO 8601 strings such as YYYY-MM-DD. The service must validate that both parameters are provided, correctly formatted, and represent a valid range where the start date is earlier than or equal to the end date.

Once validation succeeds, the Rust service must call the Python API, handle any network failures or invalid responses, and return a structured JSON response to the client. The service must return clear and consistent error messages, always using appropriate HTTP status codes such as 400 for invalid parameters, 502 for upstream errors, or 500 for internal failures.

The Rust service must load all configuration through TOML files, including the listening address, the URL of the python-api, timeout settings and logging configuration.

## Python Service Requirements

The Python service must expose an internal HTTP endpoint that receives the same parameters as the Rust service. It must validate these parameters independently and then start a Celery task responsible for computing one average price per day between the provided dates. The Python service must interact with a database and must use optimized queries to reduce unnecessary memory usage. For example, computing daily averages at the database level using grouping logic is strongly encouraged.

The Celery task must return the daily aggregated results. The Python API must then return these results in JSON format. Error conditions must be handled cleanly and consistently. Configuration for this service must also be stored in TOML, including database credentials, Celery broker configuration and result backend settings.

## Database Requirements

The database must contain at least one table dedicated to price storage. This table must include a primary key, a timestamp or date column and a numeric price column. You may include additional columns if necessary. You must provide a mechanism to initialize or migrate the schema automatically when Docker Compose starts. The Python service must be able to query the database immediately after service startup.

## Error Handling and Validation

A strong emphasis is placed on error validation and consistent handling of failure scenarios. Every incoming request must be validated thoroughly. Missing or incorrectly formatted dates must lead to a clear 400 response. A reversed or invalid date range must also produce a 400 response. Upstream errors, such as failures coming from python-api or timeouts, must be translated into appropriate status codes and structured error messages.

Error responses must always follow the same structure and must not leak internal stack traces or sensitive details. The entire system must be predictable and robust when facing invalid input or system malfunction.

## Docker Compose Requirements

The reviewer must be able to clone the repository, read the documentation and start the full system using Docker Compose alone. The Docker setup must include the Rust API, the Python API, the Celery worker, the Celery broker and the database. The database must be started and initialized as part of the Docker Compose environment.

A fully functioning system must come up without any manual intervention other than the steps explicitly documented. If unclear or missing documentation blocks the reviewer from running the project, this may result in elimination.

## TOML Configuration

All configuration used by the services must be expressed in TOML files. This includes application-level settings, database credentials, port numbers, Celery configuration and external service references. Both Rust and Python services must load configuration exclusively from TOML files, although they may optionally combine these with environment variables.

## Documentation Requirements

Your repository must contain a NOTICE or README file explaining the architecture and presenting the steps required to run the system. The documentation must clearly describe prerequisites, configuration preparation, the Docker Compose startup command, optional database seeding instructions, example HTTP requests and the expected responses.

The reviewer must be able to follow your instructions without guessing or filling in missing steps. Documentation quality is a critical part of the evaluation, and incomplete instructions may lead to project rejection.

## Evaluation Criteria

Your project will be assessed based on correctness of the implemented logic, efficiency of the daily price aggregation, quality and consistency of error handling, clarity and robustness of the codebase, correct usage of TOML configuration, reliability of the Docker environment and quality of the documentation. Optional elements such as tests, linting or health checks may add additional value but are not mandatory.
