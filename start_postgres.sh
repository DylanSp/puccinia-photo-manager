#!/usr/bin/env bash

sudo docker run --name photo-manager-postgres -e POSTGRES_PASSWORD=dbpass -e POSTGRES_DB="puccinia_photo_manager_dev" -d --rm -p 5432:5432 postgres:13
