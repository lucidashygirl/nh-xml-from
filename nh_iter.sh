#!/bin/bash

for f in $1; 
do 
  nh-xml-from $f $2 
done
