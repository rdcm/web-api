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
- `app` - application layer with endpoints
- `domain` - business logic contracts (queries/commands interfaces)
- `domain_impl` - implementation of buisiness logic contracts
- `infra` - infrastructure layer (dal)
- `host` - host process for application with composition root
- `integration-tests` - integration tests for application