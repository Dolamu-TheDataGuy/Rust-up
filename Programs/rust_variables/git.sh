#!/bin/bash

git add .

echo "Please enter commit message:"

read MSG

git commit -m $MSG