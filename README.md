# desert-server
A concept to use rust as a web server

## Concept

* Routing
* Customizable
* Fast development
* View System
* Admin backend


## Based on

* Hyper 0.10
* Diesel
* Macros 1.1 / Macros 2.0

## Idea's

### Server

* defining multiple endpoints
* possible `DDos` prevention functionality

```rust

server! {
  HttpServer(8080) {
    IndexMount
  }
  HttpsServer(8443) {
    SecureMount
  }
}
```

### Mount Point

* Router
* Chain
* Serve
* Blueprint

### Chain

* A `Chain` has an defined Input and an defined Output.
* A `Chain` could be a future or linear function.

```rust
pub enum ChainResult<T, E> {
  Future(T),
  Linear(T),
  Error(E),
}

pub trait Chain {
  pub Input;
  pub Output;
  pub Error;
  pub chain(&self, input: Input) -> ChainResult<Output, Error>
}
```

```rust
chain! {
  GetIndexChain(ReturnError) {
    JsonRequest<>,
    AccessTokenAuth,
    Processing,    
  }
};
```

### Router

* nested router
* matching use finite automate
* global before and after `Chain`'s


```rust
router! {
  IndexRouter {
    ("/", Get) -> chain!{...},
    ("/", Post) -> chain!{},
    ("/:id/", Post) -> chain!{...},
    "/api/" -> router! {...},    
  }
}
```

### Static files

```rust
```

### Blueprint

* using blueprint to easily write a customizable `REST` Api.

* get `%s` - get
* get `%s/:id` -> get
* post `%s` -> create
* put `%s/:id` -> update
* post `%s/:id` -> create
* delete `%s/:id?` -> delete
* hasMany
  * post `%s/add` -> create
  * put `%s/update/:id` -> update
  * delete `%s/remove/:id` -> delete
* hasOne
  * put `%s/:key` -> update

```
blueprint! {...} -> Mount
```

```rust
blueprint! {
  User(users) {
    get -> [],
    create -> {
      before -> [AccessTokenAuth]
    },
    update -> {
      before -> [AccessTokenAuth]
    },
    delete -> {
      before -> [AccessTokenAuth]
    },
  }
};
```

### Model (diesel-rs)

* Generate a Insertable, Queryable and Identifiable

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

### View

* pattern precompile view's
* possible caching
* shadow function for (server side rendering (React))
* hot reload

```rust
let value = html! {
  head! {
    title! {
      format!("{:?}", title),
    }
  },
  body! {...},
};
```
