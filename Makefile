

help: ## Display help
	@awk 'BEGIN {FS = ":.*##"; printf "Usage:\n  make <target>\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  %-15s %s\n", $$1, $$2 } /^##@/ { printf "\n%s\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

docker-image: ## Build a docker image with the 'es-mcp' tag
	docker build -t es-mcp .

docker-multiarch-image: ## Build an amd64+arm64 docker image with the 'es-mcp' tag
	docker buildx build --platform linux/amd64,linux/arm64 --tag es-mcp .
