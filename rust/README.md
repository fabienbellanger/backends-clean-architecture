# Rust Clean Architecture

## Install

### Rust

To install `Rust`: [Getting started](https://www.rust-lang.org/learn/get-started)

### SQLx

#### Install CLI

To install `sqlx-cli`: [GitHub](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#install)

#### Migrations

To create a migration:

```bash
$ sqlx migrate add -r <name>
$ sqlx migrate add -r create_users_table
```

Run migration:

```bash
$ sqlx migrate run
```

Revert migration:

```bash
$ sqlx migrate revert
```

> **ATTENTION**
>
> For security reasons, auto migrations are disabled
> because the project uses POS database (the code is commented)!

### Project

Configure your own `.env` file.

Update your database (in pos v2)

Then run:

```bash
$ cd infrastructure && cargo run -- serve
```

or

```bash
$ make serve
```

## Deployment

### Build project

Build the project

```bash
$ make build
```

### Nginx

1. Create the vhost `clean_rust` in `/etc/nginx/sites-available/`

   ```
   server {
       listen                      443 ssl;
       http2                       on;

       ssl_certificate             fullchain.pem;
       ssl_certificate_key         privkey.pem;

       server_name                 clean_rust.my-domain.com;

       chunked_transfer_encoding   on;
       charset                     utf-8;

       # access_log                  /var/log/nginx/clean_rust-access.log;
       error_log                   /var/log/nginx/clean_rust-error.log;

       location / {
               # limit_req           zone=one burst=15 nodelay;
               proxy_pass          http://127.0.0.1:<port>;
               proxy_http_version  1.1;
               proxy_set_header    Upgrade $http_upgrade;
               proxy_set_header    Connection 'upgrade';
               proxy_set_header    Host $host;
               proxy_cache_bypass  $http_upgrade;
       }
   }
   ```

2. Create a symbolic link
   ```bash
   $ sudo ln -s /etc/nginx/sites-available/clean_rust /etc/nginx/sites-enabled/clean_rust
   ```
3. Check configuration
   ```bash
   $ sudo nginx -t
   ```
4. Restart or reload Nginx
   ```bash
   $ sudo service nginx reload
   $ sudo service nginx restart
   ```

### Systemctl

#### Service creation

Create the service `clean_rust` in `/lib/systemd/system/`

```
[Unit]
Description=Clean API in Rust
After=network.target

[Service]
Type=simple
Restart=always
RestartSec=5s
ExecStart=<absolute path to server.sh>

[Install]
WantedBy=multi-user.target
```

#### Commands list

| Command                                    | Description        |
| ------------------------------------------ | ------------------ |
| `systemctl start <service name>.service`   | To launch          |
| `systemctl enable <service name>.service`  | To enable on boot  |
| `systemctl disable <service name>.service` | To disable on boot |
| `systemctl status <service name>.service`  | To show status     |
| `systemctl stop <service name>.service`    | To stop            |

## Architecture

The workspace is composed of 3 modules:

- `domain` contains all business logic (entities, repositories, use cases, etc.)
- `infrastructure` contains the implementation of the repositories with MySQL, the HTTP server and calls to external APIs
- `shared` contains common structures and functions (errors, authentication, database, validation, etc.)

## Documentation

### Project

Run :

```bash
$ cargo doc --open --no-deps --document-private-items # without external dependencies
$ cargo doc --open --document-private-items           # with external dependencies
```

or

```bash
$ make doc      # without external dependencies
$ make doc-deps # with external dependencies
```

### API

The documentation is an OpenAPI locally available on `http://localhost:8087/doc/api-v1`.

## Benchmark

Use [Drill](https://github.com/fcsonline/drill)

```bash
drill --benchmark drill.yml --stats --quiet
```

## Generate JWT ES384 keys

```bash
# Private key
openssl ecparam -name secp384r1 -genkey -noout -out infrastructure/keys/private.ec.key

# Public key
openssl ec -in infrastructure/keys/private.ec.key -pubout -out infrastructure/keys/public.ec.pem

# Convert SEC1 private key to PKCS8
openssl pkcs8 -topk8 -nocrypt -in infrastructure/keys/private.ec.key -out infrastructure/keys/private.ec.pem

rm infrastructure/keys/private.ec.key
```

## TODO

- [ ] Build in different targets (Linux on Mac)
