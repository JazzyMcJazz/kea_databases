# Artifacts
There are no pure SQL scripts with this project. Everything is handled programattically in Rust by the SeaORM library. 

This has the benefit that everything is handlede through migration files that can be individually applied and rolled back.

The migration files can be found in the `src` subfolder in this directory ( `migration/` ).

# Installation for Development

This assumes that the project has already been cloned to a local machine.

1. Run a new database in docker (you can use MySQL or MariaDB. I am using MariaDB).

```shell
docker run -d \
--name mariadb \
-p 3306:3306 \
-e MARIADB_USER=my_user \
-e MARIADB_PASSWORD=my_cool_secret \
-e MARIADB_RANDOM_ROOT_PASSWORD=yes \
-e MARIADB_DATABASE=my_db \
mariadb:latest
```

2. Create a file `.env` in the project root with the following variable:
```.env
DATABASE_URL="mysql://my_user:my_cool_secret@localhost:3306/my_db"
```

3. Install Rust (if it is not already installed)
    - https://www.rust-lang.org/tools/install 

4. Install the SeaORM CLI tool (this is required to run migrations from the terminal)
```shell
cargo install sea-orm-cli
```

5. Make sure your terminal is in the project root
6. Use the migration tool to run apply all migrations
```shell
sea-orm-cli migrate up
# or for short:
sea migrate up
```
This will install necessary dependencies, compile the code and apply the migrations

7. (optional) Rollback migrations
```shell
# rollback one migration
sea migrate down
# rollback all migrations
sea migrate down -n 6
```

### Note:
The migrations currently also have scripts to insert test data into the database. This is not good practice, and rolling back those specific migrations and re-applying them will not work due to expected foreign keys that will no longer exist.

These migrations are only there for convenvience during early development.