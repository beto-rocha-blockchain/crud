#!/bin/bash
# 2. READ ALL (GET)
echo "Lendo todos os registros..."
curl -s http://127.0.0.1:8080/data | jq 