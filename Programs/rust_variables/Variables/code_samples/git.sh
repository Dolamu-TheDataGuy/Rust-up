#!/bin/bash

echo "Please enter your commit message"
read COMMIT_MSG

git add .
git commit -m "$COMMIT_MSG"

echo "Please enter remote name"
read REMOTE

echo "Please enter branch name"
read Branch_name

git push $REMOTE $Branch_name


