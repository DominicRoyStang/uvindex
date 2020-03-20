.PHONY: show-help start stop down nuke inspect-backend inspect-cli

.DEFAULT_GOAL := show-help

show-help:
	@echo 'Available Commands:'
	@echo '  start | Builds and starts all containers.'
	@echo '  stop | Stops all running containers.'
	@echo '  down | Stops containers, clears volumes & networks. If things are not working as expected, you likely want to run this command.'
	@echo '  nuke | Clears all images, containers, networks, and volumes. If all else fails, run this command.'
	@echo '  inspect-<SERVICE> | Runs bash inside the running service container.'

start:
	docker-compose --file services/docker-compose.yaml up --build --renew-anon-volumes

stop:
	docker stop `docker ps -aq`

down:
	docker-compose --file services/docker-compose.yaml down -v

nuke:
	- docker stop `docker ps -aq`
	- @echo 'Note: this may take a while'
	- docker system prune --all --volumes

inspect-backend:
	docker-compose --file services/docker-compose.yaml exec backend /bin/bash

inspect-cli:
	docker-compose --file services/docker-compose.yaml exec cli /bin/bash
