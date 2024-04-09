setup:
# install brew if not installed
ifeq (, $(shell which brew))
	$(info "brew not found, install...")
	/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
endif
# install docker if not installed
ifeq (, $(shell which docker))
	$(info "docker not found, install...")
	brew install --cask docker
endif
# install helm if not installed
ifeq (, $(shell which helm))
	$(info "helm not found, install...")
	brew install helm
endif
# install rust if not installed
ifeq (, $(shell which rustc))
	$(info "rust not found, install...")
	curl https://sh.rustup.rs -sSf | sh
endif
# update /etc/hosts (required for deploy with helm)
ifeq (, $(shell cat /etc/hosts | grep dev-wep-api.com))
	$(info "update /etc/hosts...")
	echo "127.0.0.1  dev-wep-api.com" | sudo tee -a /etc/hosts
endif
	rustup component add rustfmt
	rustup component add clippy
	rustup component add llvm-tools-preview
	cargo install grcov

deploy:
	openssl req \
	-x509 -newkey rsa:4096 -sha256 -nodes \
	-keyout tls.key -out tls.crt \
	-subj "/CN=ingress.local" -days 365

	kubectl create secret tls api-secret \
  	--cert=tls.crt \
  	--key=tls.key

	helm upgrade --install --atomic --timeout 300s --wait web-api-dev helm

delete:
	helm delete web-api-dev
	kubectl delete secret api-secret --ignore-not-found