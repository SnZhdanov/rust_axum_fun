# Table Of Contents
- [Requirements](#requirements)
- [Initialization and Running](#initialization-and-running)
    - [Set Up](#set-up)
    - [Viewing the Database Records](#viewing-the-database-records)
    - [Run the App](#run-the-app)
    - [Unit Tests](#unit-tests)
    - [Live Simulation Test](#live-simulation-test)
- [Data Models Format](#data-models-format)
- [End Points](#end-points)
    - Table
        - [Create Table](#create-table)
        - [Get Table](#get-table)
        - [List Table](#list-tables)
        - [Delete Table](#delete-table)
    - Orders
        - [Create Order](#create-order)
        - [Get Order](#get-order)
        - [List Orders](#list-orders)
        - [Delete Order](#delete-order)
    - Item
- [TODO Check List](#todo-check-list)
- [Technical Challenges](#technical-challenges)
- [Notes](#notes)

----------------------
----------------------

# Requirements
- rust 1.75 (older versions will freak out on the AppState being private, so make sure your rust is up-to-date, which at the time of writing this is 1.75)
- docker

# Initialization and Running
## Set Up
```
git clone git@github.com:SnZhdanov/rust_axum_fun.git
git checkout develop
cp .sample_env .env

```
export the env variables
```
export $(grep -v '^#' .env | xargs)
```

## Run the App
Once the environment is set, docker compose and then cargo run
```
docker-compose up -d
cargo run
```

One the app is running, you can begin to make curl requests on port 9090 like the following:
```
curl -H "Content-Type: application/json" -X POST 0.0.0.0:9090/table -d '{}'
```

## Viewing the Database Records
After running docker-compose, the container mongo-express will run.
Mongo-express is a way to present the mongo database through an http link.
If you want to look at the databse on a more finer detail follow this link.
```

http://localhost:8081/

```

Note: It might take a few seconds(1-5 seconds) after you run docker-compose up for mongo-express to be ready.

## Unit Tests
To run unit tests, do the following
```
cd restaurant_app
cargo test handlers::
```
expected result
```
running 29 tests
test handlers::item_handler::unit_tests::order_unit_tests::successful_list_items ... ok
test handlers::item_handler::unit_tests::order_unit_tests::failed_list_items_b_deserialization_error ... ok
-- snipped --
```

## Live Simulation Test
To test a simulation of multiple servers interacting with the api do the following.
```
cd restaurant_app
cargo run
```
then in a different terminal
```
cd restaurant_app
cargo test tests::run_async_test
```
It should take 20 seconds since I made it so that a few servers wait for 10 seconds to search for orders that are done.
expected output
```
running 1 test
test tests::integration_tests::integration_tests::run_async_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 29 filtered out; finished in 20.18s
```


----------------------
# Data Models Format
- Table
```
{
    "table_id": Int,
    "orders":[
        Order
    ]
}
```
- Order
```
{
    "order_id": Int,
    "table_id": Int,
    "order_time": DateTime<Utc>,
    "cook_status": Enum (InProgress/Done),
    "item": Item
}
```
- Item
```
{
    "item_name": String,
    "cook_time": Int,
}
```


----------------------
# End Points
- Table
    - [Create Table](#create-table)
    - [Get Table](#get-table)
    - [List Table](#list-tables)
    - [Delete Table](#delete-table)
- Orders
    - [Create Order](#create-order)
    - [Get Order](#get-order)
    - [List Orders](#list-orders)
    - [Delete Order](#delete-order)
- Item
    - [List Items](#list-items)


## Create-Table
- POST
- End Point: `/table`
- Body: {"orders": Vec< String > }
- Output Format
```
{
    "id": ObjectId,
    "table": {Table}    
}

```
- Example Curl
```
curl -H "Content-Type: application/json" -X POST 0.0.0.0:9090/table -d '{}'

curl -H "Content-Type: application/json" -X POST 0.0.0.0:9090/table -d '{"orders": ["Ramen", "Borsht"]}'
```

## Get Table
- GET
- End Point: `/table/:table_id`
- Path Param: table id (Int)
- Output Format
```
{
    "id": ObjectId (the record's index in mongodb),
    "table": Table
}

```
- Example Curl
```
curl -X GET '0.0.0.0:9090/table/1'
```

## List Tables
- GET
- End Point: `/table`
- Query Params
    - limit: Int
    - offset: Int
    - table_id: Int 
        - filter on table_id
    - order_id: Int
        - filter on table that have order_id
    - item_name: String
        - fuzzy match on given string
    - item_names: Vec< String >
        - filter on tables that have specified items
- Output Format
```
    "tables":[Tables],
    "pagination":{
        "total":Int,
        "limit":Int,
        "offset":Int
    },
    "filters":{
        "table_id": Int,
        "order_id": Int,
        "item_name":Strings,
        "item_names":[ Strings ]
    },
    "errors":{
        "failed_table_ids":[String],
        "failed_table_count":Int
    }
```
- Example Curl
```
curl -X GET '0.0.0.0:9090/table?item_names=Hotdog&item_names=Borsht&item_name=Borsht&table_id=2&order_id=3&limit=5&offset=0'

curl -X GET '0.0.0.0:9090/table?item_name=Borsht&limit=5&offset=0'

```
## Delete Table
- DELETE
- End Point: `/table/:table_id`
- Path Param:
    - table_id: Int
- Output Format
```
{
    "table":{
        "table_id": Int,
        "orders":[Order]
    }
}

```
- Example Curl
```
 curl -X DELETE '0.0.0.0:9090/table/1'
```

## Create Order
- POST
- End Point: `/table/:table_id/order`
- Param: table_id: Int
- Body: {"orders": Vec< String > }
- Output Format
```
"table":{
    "table_id": Int,
    "orders":[Order]
}
```
- Example Curl
```
curl -H "Content-Type: application/json" -X POST 0.0.0.0:9090/table/1/order -d '{"orders":["Hotdog", "Borsht"]}'
```
## Get Order
- GET
- End Point: `/table/:table_id/order/:order_id`
- Param: 
    - table_id: Int
    - order_id: Int
- Output Format
```
"order":{
    "order_id": Int,
    "table_id": Int,
    "ordered_time": DateTime,
    "cook_status": Enum(InProgress/Done),
    "item":{
        "item_name": String,
        "cook_time": Int
    }
}
```
- Example Curl
```
curl -X GET '0.0.0.0:9090/table/3/order/2'
```
## List Orders
- GET
- End Point: `/table/order`
- Query Params
    - limit: Int
    - offset: Int
    - table_ids: Vec< Int > 
        - filters orders with a table_id in the vec
    - item_names: Vec< String >
        - filter on tables that have specified items
    - cook status: Enum(InProgress/Done)
        - filter on items depending on their cook status
- Output Format
```
    "orders":[Order],
    "pagination":{
        "total":Int,
        "limit":Int,
        "offset":Int
    },
    "filters":{
        "table_ids": Vec< Int >,
        "item_names":[ Strings ],
        "cook_status": Enum(InProgress/Done)
    }
```
- Example Curl
```
curl -X GET '0.0.0.0:9090/table/order?item_names=Ramen&item_names=Borsht&cook_status=InProgress&limit=5&offset=0'

curl -X GET '0.0.0.0:9090/table/order?cook_status=InProgress&limit=5&offset=0'

```

## Delete Order
- DELETE
- End Point: `/table/:table_id/order/:order_id`
- Path Param: 
    - table_id: Int
    - order_id: Int
- Output Format
```
{
    "table":{
        "table_id": Int,
        "orders":[Order]
    }
}
```
- Example Curl
```
 curl -X DELETE '0.0.0.0:9090/table/1/order/1'
```
## List Items
- GET
- End Point: `/item`
- Output Format
```
{
    "items": [Item]
}
```
- Example Curl
```
curl -X GET '0.0.0.0:9090/item'
```


----------------------
----------------------

# Technical Challenges
So normally in MongoDB, you have `ObjectIds` which act as a unique ID for database indexing. The problem with `ObjectId` is they look like this `659633cd5d59de8dca135ef5` which is kind of a pain for writing in Curls when this is meant for testing purposes and not an actual production environment. So for the sake of simplifying interacting with the database, I utilized an Arc Mutex to keep track of table ids that just increment from 1..N on creation request. This way instead of writing a curl like this
```
curl -X GET 0.0.0.0:9090/table/659633cd5d59de8dca135ef5
```
I can just do this
```
curl -X GET 0.0.0.0:9090/table/1
```
Again, if this were a real production environment and I were using MongoDB, I would default to using the `ObjectId` or overwriting the index field instead of some reference counter that resets when you exit the program (which is why I'm not storing the volume to prevent annoying behaviors).


# TODO Check List
- [x] Mongodb setup
    - [x] containerize mongodb  
    - [x] containerize mongo-express
- [] Axum setup
    - [] containerize rust
- [] Documentation
    - [x] How to run
    - [x] Procedure for running unit tests
    - [x] Procedure for running simulation test
    - [x] All end points
    - [x] Data Format
- [x] Crud for Tables
    - [x] create
        - [x] idpotent
        - [x] insert
        - [x] You should be able to create Orders on POST -- Stretch
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
    - [x] Live Simulation
- [] Integration Tests???  -- STRETCH
    - Tables
        - [] handlers
    - Orders
        - [] handlers













# Notes
So the whole goal of this assignment was to explore more of the my weak spots which is dev-ops. I have experience adding onto docker files and git-actions, however writing from scratch is something I've never really done in a meaningful way, so I decied to explore that here. In addition, I have spent a majority of my professional Rust experience working in a aws cloud environment, utilizing Lambdas/Appsync/GraphQL with some brief experience in using the framework Warp. I wanted to explore at a more in-depth level rust frameworks, and decided to give Axum a shot since it felt low overhead but still light-weight enough.
