#!/bin/bash

# To run on the current session use: source set_env.sh

# Set environment variables
export DATABASE_URL="sqlite:socios.db"
export FROM_NAME="From Email Name"
export SMTP_PASSWORD="SMPTPassword"
export SMTP_USERNAME="myemail@gmail.com"
# Please change the secret key don't leave the default one (on prod, this is just an example)
export SECRET_KEY="da5ff4ed8bd415a6a99a313d54de591503b762ed33a10c98b3d8d09293ed0adaabd70e9da8721e3c4b4e1a0ec842790e159e5cd14485811945030c58c5de2467"

echo "Environment variables set:"
echo "DATABASE_URL=$DATABASE_URL"
echo "FROM_NAME=$FROM_NAME"
echo "SMTP_PASSWORD=$SMTP_PASSWORD"
echo "SMTP_USERNAME=$SMTP_USERNAME"
echo "SECRET_KEY=$SECRET_KEY"
