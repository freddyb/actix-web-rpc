# json

Json's `Getting Started` guide using json (serde-json or json-rust) for Actix web

## Usage

### server

```bash
cargo run
# Started http server: 127.0.0.1:8080
```

### web client

- POST / (embed serde-json):

  - method : ``POST``
  - url : ``http://127.0.0.1:8080/``
  - header : ``Content-Type`` = ``application/json``
  - body (raw) : ``{"name": "Test user", "number": 100}``

### curl

`curl -v -H "Content-Type: application/json" -d '{"cmd": "Test user", "param": "100"}' localhost:8080/`

