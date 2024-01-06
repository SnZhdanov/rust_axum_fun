# Forward
So the whole goal of this assignment was to explore more of the my weak spots which is dev-ops. I have experience adding onto docker files and git-actions, however writing from scratch is something I've never really done in a meaningful way, so I decied to explore that here. In addition, I have spent a majority of my professional Rust experience working in a aws cloud environment, utilizing Lambdas/Appsync/GraphQL with some brief experience in using the framework Warp. I wanted to explore at a more in-depth level rust frameworks, and decided to give Axum a shot since it felt low overhead but still light-weight enough.

# Required Tools
- mongodb
- docker

# Initialization Steps
```
git clone git@github.com:SnZhdanov/rust_axum_fun.git
cp .sample_env .env
```
fill out the env variables, username and password for the database can be something like admin/password

```
#/.env
MONGO_INITDB_ROOT_USERNAME=admin
MONGO_INITDB_ROOT_PASSWORD=password
```
Once the environment is set, docker compose and then cargo run
```
docker-compose up mongo
cd axum/src/
cargo run
```

# Technical Challenges
So normally in MongoDB, you have `ObjectIds` which act as a unique ID for database indexing. The problem with `ObjectId` is they look like this `659633cd5d59de8dca135ef5` which is kind of a pain for writing in Curls when this is meant for testing purposes and not an actual production environment. So for the sake of simplifying interacting with the database, I utilized an Arc Mutex to keep track of table ids that just increment from 1..N on creation request. This way instead of writing a curl like this
```
curl -X GET 0.0.0.0:8080/table/659633cd5d59de8dca135ef5
```
I can just do this
```
curl -X GET 0.0.0.0:8080/table/1
```
Again, if this were a real production environment and I were using MongoDB, I would default to using the `ObjectId` or overwriting the index field instead of some reference counter that resets when you exit the program (which is why I'm not storing the volume to prevent annoying behaviors).


# TODO
- [x] Mongodb setup
    - [x] dockerize mongodb  
- [] Axum setup
    - [] dockerize axum
    - [] dockerize rust
- [x] Crud for Tables
    - [x] create
        - [x] idpotent
        - [x] insert
        - [] You should be able to create Orders on POST -- Stretch
    - [x] get
    - [x] list
        - [x] filter
            - [x] no filters
            - [x] table_id
            - [x] order_id
            - [x] item_name fuzzy check
            - [x] list of item_names
        - [x] pagination
        - [] sort -- STRETCH
    - [x] delete
    - [] update -- STRETCH
- [x] Crud for Orders
    - [x] create
    - [x] get
    - [] /table/1/order should output the table's order-- Stretch
    - [x] list
        - [x] filters
            - [x] no filters
            - [x] item_names
            - [x] order status
        - [x] pagination    
        - [] sort -- STRETCH
    - [x] delete
    - [] update  -- STRETCH
- [x] Crud for Items
    - [x] list
        - [x] filters
            - [x] no filters
            - [x] item_names
        - [x] pagination    
        - [] sort -- STRETCH
- Error Handling
    - [x] Tables
        - [x] tables
        - [x] tables_db
    - [x] Orders
        - [x] order
        - [x] order_db
    - [x] Items 
        - [x] item
        - [x] items_db
- Unit Tests
    - Tables
        - [x] handlers
    - Orders
        - [x] handlers
    - Items
        - [x] handlers 
- Live Testing
    - [] todo
- [] Integration Tests???  -- STRETCH
    - Tables
        - [] handlers
    - Orders
        - [] handlers

