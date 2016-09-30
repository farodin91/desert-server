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
  chain -> [
    JsonRequest<>,
    AccessTokenAuth,
    Processing,
  ],
  error -> [
    ReturnError,
  ]  
};
```

#### Router

```rust
router! {
  ("/", Get) -> chain! {...},
  ("/", Post) -> chain! {...},
  ("/:id/", Post) -> chain! {...},
  "/api/" -> router! {...},
}
```

#### Model

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


```rust
blueprint! {
  table -> users,
  methods -> {
    get -> [],
    create -> [AccessTokenAuth],
    update -> [AccessTokenAuth],
    delete -> [AccessTokenAuth],
  },
};
```
