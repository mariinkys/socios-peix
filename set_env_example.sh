#!/bin/bash

# Set environment variables
export DATABASE_URL="sqlite:socios.db"
export FROM_NAME="From Email Name"
export SMTP_PASSWORD="SMPTPassword"
export SMTP_USERNAME="myemail@gmail.com"

echo "Environment variables set:"
echo "DATABASE_URL=$DATABASE_URL"
echo "FROM_NAME=$FROM_NAME"
echo "SMTP_PASSWORD=$SMTP_PASSWORD"
echo "SMTP_USERNAME=$SMTP_USERNAME"
