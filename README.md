# rust-tokio-postgres
This is an example of postgres query example using Rust + tokio postgres

## Prepare environment
Before running the app, we need a postgres instance running. I set one locally with docker by running the following command:
```sh
docker run --name pgsql-dev -e POSTGRES_PASSWORD=test1234 -p 5432:5432 postgres
```

After that, I've created a test table using the follow SQL:
```sql
CREATE TABLE public.test (
	"name" varchar NULL,
	description varchar NULL
);
```
To insert data, I've executed the following script:
```sql
INSERT INTO public.test
("name", description)
VALUES('Test 1', 'some description');
INSERT INTO public.test
("name", description)
VALUES('Test 2', 'some other description');
```

## Build
To build the application, run:
```sh
cargo build
```

## Run
To execute the application, run:
```sh
sudo cargo run
```

It should get the following result:

```
name='Test 1' description='some description'
name='Test 2' description='some other description'
```