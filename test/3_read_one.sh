#!/bin/bash
# 3. READ ONE (GET)

if [ -z "$1" ]; then
  read -p "Digite o id do registro a ser lido: " id
else
  id="$1"
fi

echo "Lendo o registro criado (id=$id)..."
curl -s http://127.0.0.1:8080/data/$id | jq 