#!/bin/bash
_srcpath="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && cd ../ && pwd )"

if [ $# = 1 ]; then
  _name=$1
else
  echo "terminated."
  echo "Usage: $0 <model_name>"
  exit 0;
fi

capitalize() {
  len=`echo "$1" | awk '{print length($0)}'`
  first=`echo ${1:0:1}`
  therest=`echo ${1:1:$len}`
  capital=`uppercase $first`
  echo "$capital$therest"
}

uppercase() {
  echo "$1" | tr '[:lower:]' '[:upper:]'
}

_var=$_name
_vars="${_name}s"
_struct=`capitalize $_name`
_structs="${_struct}s"

# -- controller
_tplfile=$_srcpath/scripts/tpl/controller.tpl
_tmpfile=$_srcpath/src/controllers/$_var.rs
cp $_tplfile $_tmpfile

/usr/bin/sed -i .copy "s/_Templates_/$_structs/g" $_tmpfile
rm $_tmpfile.copy

/usr/bin/sed -i .copy "s/_Template_/$_struct/g" $_tmpfile
rm $_tmpfile.copy

/usr/bin/sed -i .copy "s/_templates_/$_vars/g" $_tmpfile
rm $_tmpfile.copy

/usr/bin/sed -i .copy "s/_template_/$_var/g" $_tmpfile
rm $_tmpfile.copy

# -- model
_tplmodel=$_srcpath/scripts/tpl/model.tpl
_tmpmodel=$_srcpath/src/models/$_var.rs
cp $_tplmodel $_tmpmodel

/usr/bin/sed -i .copy "s/_Templates_/$_structs/g" $_tmpmodel
rm $_tmpmodel.copy

/usr/bin/sed -i .copy "s/_Template_/$_struct/g" $_tmpmodel
rm $_tmpmodel.copy

/usr/bin/sed -i .copy "s/_templates_/$_vars/g" $_tmpmodel
rm $_tmpmodel.copy

/usr/bin/sed -i .copy "s/_template_/$_var/g" $_tmpmodel
rm $_tmpmodel.copy