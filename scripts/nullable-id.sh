#!/bin/bash

/usr/bin/sed -i .bak 's/id -> Integer,/id -> Nullable<Integer>,/' ./src/schema.rs
rm ./src/schema.rs.bak