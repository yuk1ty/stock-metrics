# stock-metrics

Stock price and stats viewer.

## Getting Started

### Middleware

Launch the middleware by executing docker compose:

```
cd local-middleware
docker compose up -d
```

This app now takes advantage of the following tools:

- MySQL
- DynamoDB (future)

### Setting up database tables

Please run SQLs in `migrations` directory. `up.sql` can be up tables, `down.sql` removes them.

### Run the app

```
cargo run
```

After running the command, you can see tracing logs.

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/bootstrap`
2021-12-21T13:49:54.407374Z  INFO stock_metrics_driver::startup: Server listening on 127.0.0.1:8080
```

## Architecture

This example has 4 workspaces as following:

- stock-metrics-driver (driver or controller)
- stock-metrics-app (app or usecase)
- stock-metrics-kernel (kernel or domain)
- stock-metrics-adapter (adapter or infrastructure)

The upper side in this list is to be an upper layer, the lower ones are to be a lower layer. The upper layers can call or use the lower ones but the opposite calling isn't allowed. For instance, the driver layer can call the app layer's modules but the app layer cannot call the driver layer's modules.

DIP (Dependency Inversion Principle) is applied between kernel and adapter layer. For example, the kernel layer's repositories have just definitions of traits, these implementations are in the adapter layer. As a sample, `StockViewRepository` and `DatabaseRepositoryImpl<Stock>` help us understand that.

The driver layer has only around Axum's definition. Axum's `Router`, handler and launching the server. Things around definitions and settings for web applications have to be defined within this layer.

The app layer has a so-called "use case" layer (in the context of clean architecture). The layer controls the entire application process and logic has to be defined in the range of this layer.

The kernel layer has a so-called "domain" layer (in the context of clean architecture as well). This layer is the core context of the application. For instance, calculators for stock stats have to be described within this layer.

The adapter layer has around infrastructure's concerns. This layer can connect and access outside middlewares, services or APIs. The access and connection processes have to be bounded in this layer.
