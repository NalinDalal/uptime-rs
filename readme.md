do start postgres via docker:
`docker run -p 5432:5432 -e POSTGRES_PASSWORD=password -d postgres          
b3b1a3084b7517a8f129274149cd33ab5d74d683ca5d70e5cf20781fd58644e9`

so first we created the routes, then we created 2 files to handle with the
request and handle them via serde to serialise and deserialise the json data

then we added a store workspace to handle with the db,
initialiose diesel in it

create the migration folder in it, then run the migration cmd:
`diesel migration generate create_user_website`

Creating migrations/2025-06-09-094606_create_user_website/up.sql
Creating migrations/2025-06-09-094606_create_user_website/down.sql

so sql files generated-> now need to write the up.sql file, leave down.sql for
now

setup the db via:
`diesel setup`

now run the migration cmd: `diesel migration run`

check if the tables exists or not, we asked gpt to create the cmd to directly
show the cmd:
`docker exec -it sleepy_kilby psql -U postgres -d db -c "\dt"`

it gave back:

```sql
                   List of relations
 Schema |            Name            | Type  |  Owner
--------+----------------------------+-------+----------
 public | __diesel_schema_migrations | table | postgres
 public | region                     | table | postgres
 public | website                    | table | postgres
 public | website_tick               | table | postgres
(4 rows)

```

so we have now src/Schema.rs file, auto-genrated file-> call it in lib.rs file
for store submodule particularly

now defining the connection object and running the queries

now add the sign_up and sign_in endpoint

create a new migration and update the sql files
remigrate -> `diesel migration run`

now go to user model file for store and try to put something in db
then handle with models with various functions

now move the logic to same db endpoint store,

writing test, middlewares and jwt in rust
