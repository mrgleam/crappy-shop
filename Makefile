generate-entity:
	sea-orm-cli generate entity -o entity/src/entities
migrate-down:
	sea-orm-cli migrate down
migrate-up:
	sea-orm-cli migrate up
run:
	cargo watch -x run
