# rust-rocket-mysql-starter

## Initial setup

#### 1. Prepare environment file `.env`
```
cp ./env.example .env
```

#### 2. Configure mysql connection string in both `.env` and `Rocket.toml` files.
Eg:
```
mysql://root:password@127.0.0.1:3306/rust-rocket-mysql-starter?serverTimezone=UTC
```

#### 3. To allow using the same struct for read and update table, we need to add `Nullable` flag on every auto increment `id` field
##### 3.1 Manual 
```
table! {
    users (id) {
        id -> Nullable<Integer>, //<-- Nullable added manually for auto-increment field
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        name -> Varchar,
        email -> Varchar,
        latitude -> Nullable<Decimal>,
        longitude -> Nullable<Decimal>,
        language -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        region -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        gender -> Nullable<Varchar>,
        age -> Nullable<Integer>,
    }
}
```

#### 3.2 Automate the step using `sed` command (Mac / *nix platform)
```
sh ./scripts/nullable-id.sh
```

#### 4. Run diesel command to auto create database and 
```
diesel setup
diesel migration run
```

## Run service
```
cargo run
```

## Extras

### CRUD generator script
```
sh ./scripts/crud-generator.sh <model_name>

example:
sh ./scripts/crud-generator.sh newcrud
```

this script will auto generate model and controller files,
but need to register as modules manually under `controllers/mod.rs` and `models/mod.rs`
```
pub mod newcrud;
```

and mount the endpoints under `main.rs`
```
.mount("/newcrud", controllers::newcrud::routes())
```