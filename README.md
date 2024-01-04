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


# Technical Challenges
So normally in MongoDB, you have `ObjectIds` which act as a unique ID for database indexing. The problem with `ObjectId` is they look like this `659633cd5d59de8dca135ef5` which is kind of a pain for writing in Curls when this is meant for testing purposes and not an actual production environment. So for the sake of simplifying interacting with the database, I utilized an Arc Mutex to keep track of table ids that just increment from 1..N on creation request. This way instead of writing a curl like this
```
curl -X GET 0.0.0.0:8080/table/659633cd5d59de8dca135ef5
```
I can just do this
```
curl -X GET 0.0.0.0:8080/table/1
```
Again, if this were a real production environment and I were using MongoDB, I would default to using the `ObjectId`instead of some reference counter that resets when you exit the program.




# TODO
- Mongodb setup[x]
    - dockerize mongodb [x] 
- Axum setup[]
    - dockerize axum[]
    - dockerize rust[]
- Crud for Tables[]
    - create[x]
        - idpotent[x]
        - insert[x]
    - get[x]
    - list[]
        - sort[]
        - pagination[x]
    - delete[x]
    - update???[]
- Crud for Orders[]
    - create[]
    - get[]
    - list[]
    - delete[]
    - update????[]
- Error Handling
    - Tables[]
    - Orders
- Unit Tests
    - Tables
        - handlers[]
        - db impls[]
    - Orders
        - handlers[]
        - db impls[]
- Integration Tests???[]
    - Tables
        - handlers[]
        - db impls[]
    - Orders
        - handlers[]
        - db impls[]
- Live Testing
    - todo[]
