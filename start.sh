docker build -t rpg_server:latest .

docker network rm rpg_network > /dev/null 2>&1
docker network create rpg_net
docker network connect rpg_net nginx-proxy > /dev/null 2>&1

docker stop rpg_mariadb > /dev/null 2>&1
docker rm rpg_mariadb > /dev/null 2>&1
docker run --name rpg_mariadb \
    --net rpg_net \
    -v ./data/mariadb:/var/lib/mysql:rw \
    -e MARIADB_ROOT_PASSWORD=password \
    -e MARIADB_DATABASE=database \
    -e MARIADB_USER=user \
    -e MARIADB_PASSWORD=password \
    -d mariadb:latest

docker stop rpg_surrealdb > /dev/null 2>&1
docker rm rpg_surrealdb > /dev/null 2>&1
docker run --name rpg_surrealdb \
    --net rpg_net \
    -v ./data/surrealdb:/data:rw \
    -d surrealdb/surrealdb:latest start --user root --pass root file:/data/mydatabase.db

docker stop rpg_server > /dev/null 2>&1
docker rm rpg_server > /dev/null 2>&1
docker run -d --name rpg_server \
    --net rpg_net \
    -p 3000:3000 \
    -e "VIRTUAL_HOST=rpg.treeleaf.dev" \
    -e "VIRTUAL_PORT=3000" \
    -e "LETSENCRYPT_HOST=rpg.treeleaf.dev" \
    rpg_server:latest