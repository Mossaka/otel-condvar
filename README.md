# otel-condvar


## Usage

Start a local Jaeger instance:
```bash
docker run -d -p16686:16686 -p4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest
```

Run the example:
```bash
cargo run -- 1
```

See the traces in the Jaeger UI at http://localhost:16686

You will find that under span `a2` there is a warning saying "invalid parent span id". This is because the parent span id is not set correctly.

## License

This project is licensed under the MIT license.