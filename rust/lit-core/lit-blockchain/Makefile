ROOT_DIR := $$(git rev-parse --show-toplevel)

update:
	@@./scripts/update_abis.sh
	@@./scripts/generate_contracts.sh
	
pretty:
	cd $(ROOT_DIR)/blockchain/contracts && pwd && npx prettier --config .prettierrc.json --write .