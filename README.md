# rept (receipt vault)

A REST API that scans receipt images using AI and extracts structured data — store name, purchase date, total, return window, and warranty info.

Built with Rust :D

## Tech Stack

- **Axum** — web framework
- **SQLx** — database driver
- **PostgreSQL** — database (Supabase)
- **Gemini Vision** — AI receipt extraction
- **Argon2** — password hashing
- **JWT** — authentication

## Features

- Upload a receipt image and get structured data back automatically
- JWT auth to protect routes
- Stores receipts per user
- Extracts: store name, purchase date, total, return by date, warranty expiry

## Getting Started

### Prerequisites

- Rust
- PostgreSQL (or Supabase)
- Gemini API key (Google AI Studio)

### Setup

```bash
git clone https://github.com/tasvln/rept.git
cd rept
```

Create a `.env` file:

```env
DATABASE_URL=postgresql://... [use session pool URL, if using Supabase]
JWT_SECRET=your_secret
GEMINI_API_KEY=your_key
```

Run migrations:

```bash
sqlx migrate run
```

Start the server:

```bash
cargo run
```

Server runs on `http://localhost:3000`

## API

### Auth

#### Register
```
POST /register
Content-Type: application/json

{
  "username": "ade",
  "password": "Secret123!"
}
```

Password must be 8+ characters with at least one uppercase letter, number, and special character.

#### Login
```
POST /login
Content-Type: application/json

{
  "username": "ade",
  "password": "Secret123!"
}
```

Returns a JWT token.

### Receipts

All receipt endpoints require the `Authorization` header:
```
Authorization: Bearer <token>
```

#### Upload Receipt
```
POST /receipts
Content-Type: multipart/form-data

image: <receipt image file>
```

Example response:
```json
{
  "id": "f7801c96-24eb-4833-b009-0184c70aa5fb",
  "user_id": "f4ef3969-2561-41a4-a03d-d4ce575ff583",
  "store_name": "TARGET",
  "purchase_date": "2021-08-19",
  "total": 585.74,
  "return_by": "2021-11-17",
  "warranty_until": null,
  "image_url": null,
  "raw_text": null
}
```

#### Get All Receipts
```
GET /receipts
```

Returns all receipts for the authenticated user.

#### Get Receipt by ID
```
GET /receipts/{id}
```

## Project Structure

```
src/
  main.rs
  handlers/       # request handlers
  middleware/     # middleware
  models/         # data models
  routes/         # route definitions
  services/       # AI extraction logic
  states/         # shared app state
  utils/          # jwt, password
migrations/       # database migrations
```