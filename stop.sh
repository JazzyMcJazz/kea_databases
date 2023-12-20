docker stop rpg_server > /dev/null 2>&1
docker rm rpg_server
docker stop rpg_mariadb > /dev/null 2>&1
docker rm rpg_mariadb
docker stop rpg_surrealdb > /dev/null 2>&1
docker rm rpg_surrealdb
docker network rm rpg_network