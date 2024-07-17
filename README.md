To run the local api, run the following command: 

```
cargo run
```

In a seperate terminal, send request to add movie to storage by running the following command. Note, this requires that you have [jq](https://jqlang.github.io/jq/manual/#invoking-jq) installed: 

```
MOVIE_ID=$(curl -X POST http://localhost:3000/movie -H "Content-Type: application/json" -d @movie_payload.json | jq .id)
```

Query api db with the following command: 

```
curl http://localhost:3000/movie/$MOVIE_ID
```