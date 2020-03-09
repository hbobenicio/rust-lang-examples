# actix-web-2-demo

## Setup

```
# initial setup:
docker-compose up -d
diesel setup --database-url 'postgres://demo:demo@localhost/demo'

# to create new migrations:
diesel migration generate my_migration
```
