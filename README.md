# Pulsarr
Pulsarr music rating app


DB diagramming tool: https://dbdiagram.io/

Dev setup:
 1. Create .env files for both pulsarr_fulcrum and pulsarr_web
    1. web needs to have:
       * VITE_API_URL=http://localhost:4004
    2. fulcrum (rust) needs to have:
       * ENV_NAME=local 
       * POSTGRES_PORT=5432 
       * POSTGRES_URL=postgres@local-postgres:5432/pulsarrdb
         * change this to postgres@localhost:5432/pulsarrdb if not running the rust project in docker.
       * RUST_PORT=4004
 2. create docker network to emulate deployed env by running:
    * docker network create dokploy-network