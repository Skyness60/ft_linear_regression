
# Simple Makefile pour débuter sur le projet ft_linear_regression

IMAGE=linear
DOCKER_RUN=docker run --rm -v $(PWD):/usr/src/app $(IMAGE)

.PHONY: help build train predict plot clean

help:
	@echo "Commandes disponibles :"
	@echo "  make build    : Build l'image Docker"
	@echo "  make train    : Entraîne le modèle (train.rs)"
	@echo "  make predict  : Lance la prédiction (predict.rs)"
	@echo "  make plot     : Affiche le graphique et le score R² (plot.rs)"
	@echo "  make clean    : Supprime les fichiers générés (target/, plot.png, theta.txt)"

build:
	docker build -t $(IMAGE) .

train: build
	$(DOCKER_RUN)

predict: build
	$(DOCKER_RUN) cargo run --bin predict

plot: build
	$(DOCKER_RUN) cargo run --bin plot

clean:
	rm -rf target plot.png theta.txt
