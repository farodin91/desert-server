# desert-server
A concept to use rust as a web server

## Concept

* Based on Futures (tokio-core)
* Hyper
* Routing
* Diesel


## Idea's

### Macros

#### Chain

```rust
chain! {
  Index(ReturnError) {
    JsonRequest<>,
    AccessTokenAuth,
    Processing,    
  }
};
```

#### Router

```rust
router! {
  IndexRouter {
    ("/", Get) -> chain! {...},
    ("/", Post) -> chain! {...},
    ("/:id/", Post) -> chain! {...},
    "/api/" -> router! {...},    
  }
}
```

#### Server

```rust
server! {
  Server(8000) {
    IndexRouter
  }
}
```

#### Model (diesel-rs)

```rust
table! {
  users {
    id -> Text,
    password_hash -> Text,
    active -> Bool,
    created_at -> Timestamp,
    updated_at -> Timestamp,
  }
}
```

#### Blueprint

* get `%s` - get
* get `%s/:id` -> get
* post `%s` -> create
* put `%s/:id` ->
* post `%s/:id` -> create
* delete `%s/:id?` -> delete

```
blueprint! {...} -> Router
```

```rust
blueprint! {
  User(users) {
    get -> [],
    create -> [AccessTokenAuth],
    update -> [AccessTokenAuth],
    delete -> [AccessTokenAuth],
  }
};
```
