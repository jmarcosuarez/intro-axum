# intro-axum

## To start:

```
cargo watch -x run
```

## Start database

```
docker compose up -d --wait
```

NOTE: This app has 2 route files `src/routes` and `src/data_routes` but only the later is connected. To swap routes just change the import in `src/lib.rs`
