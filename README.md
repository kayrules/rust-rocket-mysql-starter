# rust-rocket-mysql-starter

## initial setup
```
diesel setup
diesel migration run
```

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

...or on mac, just run the script
```
sh ./scripts/nullable-id.sh
```

prepare environment file `.env`
```
cp ./env.example .env
```

configure mysql connection string in both `.env` and `Rocket.toml` files.
Eg:
```
mysql://root:password@127.0.0.1:3306/rust-rocket-mysql-starter?serverTimezone=UTC
```

## run service
```
cargo run
```