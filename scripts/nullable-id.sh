#!/bin/bash
_srcpath="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && cd ../ && pwd )"

diesel print-schema > $_srcpath/src/schema.rs
/usr/bin/sed -i .bak 's/id -> Integer,/id -> Nullable<Integer>,/' $_srcpath/src/schema.rs
rm $_srcpath/src/schema.rs.bak