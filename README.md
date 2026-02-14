# Randomail

Self-hosted disposable email alias manager powered by Cloudflare Email Routing. Create, list, and delete email aliases that forward to your real address — from the CLI or a lightweight web UI you run on your own server.

## Features

- Create aliases like `shop-signup@yourdomain.com` that forward to your inbox
- Delete aliases when you're done with them
- Web UI with mobile support
- CLI tool for scripting and quick access
- Stateless — config lives in a single JSON file, aliases live in Cloudflare

## Prerequisites

- A domain with [Cloudflare Email Routing](https://developers.cloudflare.com/email-routing/) enabled
- A Cloudflare API token with email routing permissions

## Setup

Create a `config.json`:

```json
{
    "token": "your-cloudflare-api-token",
    "destination_email": "you@example.com",
    "zone": "yourdomain.com"
}
```

Or use the CLI to configure interactively:

```
randomail config -i <ACCOUNT_ID> -t <TOKEN> -e <EMAIL> -d <DOMAIN>
```

## Usage

### Web UI

```
cargo run --bin randomail-app
```

Open `http://localhost:3000`.

### CLI

```
randomail list          # show all aliases
randomail add <NAME>    # create an alias
randomail delete <ID>   # remove an alias
```

## Deployment

Run the app behind Nginx with basic auth and Let's Encrypt TLS.

### Docker

```yaml
services:
    randomail:
        image: randomail-app
        ports:
            - "127.0.0.1:3000:3000"
        volumes:
            - ./config.json:/config.json:ro
        restart: unless-stopped
```

### Nginx + Let's Encrypt

Install Certbot and grab a certificate:

```
sudo apt install certbot python3-certbot-nginx
sudo certbot --nginx -d mail.yourdomain.com
```

Create a password file for basic auth:

```
sudo apt install apache2-utils
sudo htpasswd -c /etc/nginx/.htpasswd youruser
```

Nginx config (`/etc/nginx/sites-available/randomail`):

```nginx
server {
    listen 443 ssl;
    server_name mail.yourdomain.com;

    ssl_certificate /etc/letsencrypt/live/mail.yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/mail.yourdomain.com/privkey.pem;

    auth_basic "Randomail";
    auth_basic_user_file /etc/nginx/.htpasswd;

    location / {
        proxy_pass http://127.0.0.1:3000;
    }
}
```

Enable and reload:

```
sudo ln -s /etc/nginx/sites-available/randomail /etc/nginx/sites-enabled/
sudo nginx -t && sudo systemctl reload nginx
```

## Tech Stack

Rust, Axum, Cloudflare API, vanilla HTML/CSS/JS frontend.

## Building

```
cargo build --release
```

## License

MIT
