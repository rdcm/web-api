## About

Web api application prototype written in rust

## Features

- Clean architecture design with CQRS pattern
- Crud api endpoints with actix-web and mongodb driver
- Kafka Consumer implemented with `rdkafka` 
- Integration tests
- Workspaces usage
- OpenApi and Swagger-UI
- Multi-stage docker build
- Code coverage report
- Static analysis report

## Structure

- `acl` - anti corruption layer implemented as kafka consumer
- `api` - application layer with endpoints
- `domain` - business logic contracts (queries/commands interfaces)
- `domain_impl` - implementation of buisiness logic contracts
- `infra` - infrastructure layer (dal)
- `host` - host process for application with composition root
- `integration-tests` - integration tests for application

## Up & Running

From root folder:
- `make setup` - setup environment (brew, docker, helm, rust)
- `make deploy` - deploy to kubernetes with helm (you should enable k8s support manually)
- `make delete` - delete helm chart

From src folder:
- `make docker-up-env` - up environment only
- `make docker-up-apps` - up all include applications
- `make docker-down-env` - down environment only
- `make docker-down-apps` - down all include applications
- `make docker-build-apps` - build apps images
- `make build` - build local binaries
- `make lint` - run clippy for static analysis
- `make format` - run fmt for code formatting
- `make tests` - run tests
- `make run` - run local build

## Web interfaces:

K8s:  
`swagger`  - https://dev-wep-api.com/swagger-ui/  

Docker-compose:  
`swagger`  - http://localhost:8080/swagger-ui/  
`kafka-ui` - http://localhost:8085/  