# traffic_volume

Parses a CSV file of traffic data, from the [Ontario data catalogue](https://www.ontario.ca/data/traffic-volume), and inserts the records into a MongoDB database.


## Steps:
1. Download CSV
    ```bash
    ./scripts/download
    ```

2. Start Docker
    ```bash
    docker-compose up -d
    ```

3. Import Data to DB
    ```bash
    cargo run
    ```

4. Start GraphQL Server
    ```bash
    cd server
    cargo run
    ```
    visit: http://127.0.0.1:8080/graphiql
