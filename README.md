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

## Dev environment

- `docker` v24.0.6
- `docker-compose` v2.23.0
- `kubernetes` v1.28.2
- `rustc` 1.72.1
- `cargo` 1.72.1

For deploing with helm `/etc/hosts` require `127.0.0.1 dev-wep-api.com` entry for access to application in browser.

## Up & Running

From src folder:
- `docker-compose up -d` - up dev environment
- `run.sh` - run api application for local debuging
- `build.sh` - build all workspaces
- `build_images.sh` - build `acl` and `api` dev images
- `docker-compose --profile dev-build up -d` - up `acl` and `api` dev images with compose
- `format.sh` - format code
- `analyze.sh` - static analysis
- `run_tests.sh` - build and run tests

From root folder:
- `generate_cert.sh` - generate self signed cert and create kubernetes secret
- `deploy_helm.sh` - deploy images to local kubernetes cluster