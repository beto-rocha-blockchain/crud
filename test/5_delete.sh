#!/bin/bash
# 5. DELETE (DELETE)

if [ -z "$1" ]; then
  read -p "Digite o id do registro a ser deletado: " id
else
  id="$1"
fi

echo "Deletando o registro (id=$id) com outro usuário (não autorizado)..."
curl -i -X DELETE http://127.0.0.1:8080/data/$id \
  -H "Authorization: invasor" \
  -w "\nStatus: %{http_code}\n"
echo

echo "Tentando ler o registro (id=$id)..."
curl -s http://127.0.0.1:8080/data/$id -w "\nStatus: %{http_code}\n"
