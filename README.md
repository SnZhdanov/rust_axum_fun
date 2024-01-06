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
cd axum
cargo run
```

# End Points

## Create Table
- POST
- End Point: `/table`
- Body: {"orders": Vec< String > }
- Output Format
```


```
- Example Curl
```
curl -H "Content-Type: application/json" -X POST 0.0.0.0:8080/table -d '{}'

curl -H "Content-Type: application/json" -X POST 0.0.0.0:8080/table -d '{"orders": ["Ramen", "Borsht"]}'
```

## Get Table
- GET
- End Point: `/table/:table_id`
- Path Param: table id (Int)
- Output Format
```


```
- Example Curl
```
curl -X GET '0.0.0.0:8080/table/1'
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
curl -X GET '0.0.0.0:8080/table?item_names=Hotdog&item_names=Borsht&item_name=Borsht&table_id=2&order_id=3&limit=5&offset=0'

curl -X GET '0.0.0.0:8080/table?item_name=Borsht&limit=5&offset=0'

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
 curl -X DELETE '0.0.0.0:8080/table/1'
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
curl -H "Content-Type: application/json" -X POST 0.0.0.0:8080/table/1/order -d '{"orders":["Hotdog", "Borsht"]}'
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
curl -X GET '0.0.0.0:8080/table/3/order/2'
```








## List Orders
- GET
- End Point: `/table/order`
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
curl -X GET '0.0.0.0:8080/table?item_names=Hotdog&item_names=Borsht&item_name=Borsht&table_id=2&order_id=3&limit=5&offset=0'

curl -X GET '0.0.0.0:8080/table?item_name=Borsht&limit=5&offset=0'

```








## Delete Orders
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
 curl -X DELETE '0.0.0.0:8080/table/1/order/1'
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


# TODO Check List
- [x] Mongodb setup
    - [x] dockerize mongodb  
- [] Axum setup
    - [] dockerize axum
    - [] dockerize rust
- [] Documentation
    - [] How to run
    - [] Procedure for running unit tests
    - [] Procedure for running simulation test
    - [] All end points
    - []
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