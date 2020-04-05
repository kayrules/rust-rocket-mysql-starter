# rust-rocket-mysql-starter

## initial setup

###1. prepare environment file `.env`
```
cp ./env.example .env
```

###2. configure mysql connection string in both `.env` and `Rocket.toml` files.
Eg:
```
mysql://root:password@127.0.0.1:3306/rust-rocket-mysql-starter?serverTimezone=UTC
```

###3.1 to allow using the same struct for read and update table, we need to add `Nullable` flag on every auto increment `id` field.
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

###3.2 ...or on mac, just run the script to automate the step using `sed` command
```
sh ./scripts/nullable-id.sh
```

###4. run diesel command to auto create database and 
```
diesel setup
diesel migration run
```

## run service
```
cargo run
```