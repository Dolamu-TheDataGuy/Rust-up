#!/bin/bash

echo "What is the name of your rust file"
read CODE_NAME

rustc $CODE_NAME
