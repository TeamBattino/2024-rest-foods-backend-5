# 2024-rustic-foods-backend-5

[Documentation](https://docs.google.com/document/d/1KHPHudExMSFTogelUQM4CoiMmWO4WIbcFoCGPUv4fiU/edit#heading=h.980yua1tt23w) in Google Docs

## Prerequisites

- Rust
- Diesel CLI
- Docker

## Start postgresql in docker

```bash
docker run --name postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres
```

Put the following in your `.env` file:

```bash
DATABASE_URL=postgres://postgres:postgres@localhost/postgres
```

Run Diesel migrations:

```bash
diesel migration run
```

Sample data is given in the file `data.sql`
