# Nanoservice

This Nanoservice can be directly compiled into another server or ran as a standalone server for the following:

- Actix Web (http)
- Axum (http)
- Tokio (tcp)

The core can also be directly compiled into whatever rust project you have. Below are the installation configs
for different frameworks and the core. The calc example code was coded by [Jimmy Calahorrano](https://www.linkedin.com/in/jcalahor/) 
and we decided to lift the example into the standard example for a nanoservice. 

## Core

If you don't want to expose the server to external traffic but want another server to compile this server into the binary and
call it directly then just compiling the core is enough. Below is the config for compiling the core into your project:

```toml
[nanoservices.calc_server]
dev_image = "<YOUR-DOCKER-IMAGE-YOU-UPLOAD-THIS-SERVER-TO>"
prod_image = "<YOUR-DOCKER-IMAGE-YOU-UPLOAD-THIS-SERVER-TO>"
entrypoint = "core"
package = "core"
kernel = { entrypoint = "kernel", package = "kernel", name = "calc-server-kernel" }
```

After running the `nanoforge prep` command in the root of your project where the `Cargo.toml` is located, you will be able
to access the core functions in your project with `use::calc_server` and the kernel which is the structs for your functions
with `use::calc_server_kernel`.

## Actix Web

### Running Actix
You can run the server with the following command:

```bash
cd servers/actix && cargo run
```

You can run a test client against the server with the following command:

```bash
cd clients/http && cargo run
```

### Compiling Actix using NanoForge
You can compile the Actix endpoints into your actix server project with the following config in your `Cargo.toml`:

```toml
[nanoservices.calc_server]
dev_image = "<YOUR-DOCKER-IMAGE-YOU-UPLOAD-THIS-SERVER-TO>"
prod_image = "<YOUR-DOCKER-IMAGE-YOU-UPLOAD-THIS-SERVER-TO>"
entrypoint = "servers/actix"
package = "actix-server"
kernel = { entrypoint = "kernel", package = "kernel", name = "calc-server-kernel" }
```

After running the `nanoforge prep` command in the root of your project where the `Cargo.toml` is located, you will be able
to access the core functions in your project with `use::calc_server` and the kernel which is the structs for your functions
with `use::calc_server_kernel`.

## Axum

### Running Axum
You can run the server with the following command:

```bash
cd servers/axum && cargo run
```

You can run a test client against the server with the following command:

```bash
cd clients/http && cargo run
```

### Compiling Axum using NanoForge
You can compile the Axum endpoints into your axum server project with the following config in your `Cargo.toml`:

```toml
[nanoservices.calc_server]
dev_image = "<YOUR-DOCKER-IMAGE-YOU-UPLOAD-THIS-SERVER-TO>"
prod_image = "<YOUR-DOCKER-IMAGE-YOU-UPLOAD-THIS-SERVER-TO>"
entrypoint = "servers/axum"
package = "axum-server"
kernel = { entrypoint = "kernel", package = "kernel", name = "calc-server-kernel" }
```

After running the `nanoforge prep` command in the root of your project where the `Cargo.toml` is located, you will be able
to access the core functions in your project with `use::calc_server` and the kernel which is the structs for your functions
with `use::calc_server_kernel`.

## Tokio TCP

### Running Tokio
You can run the server with the following command:

```bash
cd servers/tcp && cargo run
```

You can run a test client against the server with the following command:

```bash
cd clients/tcp && cargo run
```

### Compiling Tokio using NanoForge
You can compile the Tokio endpoints into your tokio server project with the following config in your `Cargo.toml`:

```toml
[nanoservices.calc_server]
dev_image = "<YOUR-DOCKER-IMAGE-YOU-UPLOAD-THIS-SERVER-TO>"
prod_image = "<YOUR-DOCKER-IMAGE-YOU-UPLOAD-THIS-SERVER-TO>"
entrypoint = "servers/tcp"
package = "tcp-server"
kernel = { entrypoint = "kernel", package = "kernel", name = "calc-server-kernel" }
```

After running the `nanoforge prep` command in the root of your project where the `Cargo.toml` is located, you will be able
to access the core functions in your project with `use::calc_server` and the kernel which is the structs for your functions
with `use::calc_server_kernel`.
