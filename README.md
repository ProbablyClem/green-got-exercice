# Data transformation service
asynchrounous service that transform data from a kafka topic and send it to a webhook

## Documentation
### API
Api documentation is available at http://green-got-load-balancer-1867801757.eu-west-3.elb.amazonaws.com/swagger-ui/#/

# Run
start app with `docker-compose up --build`
this will start the service listening on port 80 and the kafka broker

## Developpement environnement
### Prerequisites
    - docker
    - docker-compose
    - rust
    - cmake (needed for the rdkafka client)
### Run
- run `docker-compose -f docker-compose-kafka.yaml up` to start kafka broker
- run `cargo run` to start the service
### Test
run `cargo test` to run the tests locally  
run `docker build -f Dockerfile.test .` to run the tests in a docker container (usefull for CI)

### Environnement variables
This service use the following environnement variables :
- KAFKA_HOST : kafka broker host
- PORT : service port 
- WEBHOOK_URL : webhook url  
Thoses can be set in the .env file for local development and in the docker-compose file for production

## Deployment
A pipeline is configured to deploy the service on AWS EC2 instance on push to main branch

[Click here to learn how to replicate the service](node-setup.md)
# Consignes
Implémentation d’un service de transformation de données.


L’objectif métier est de recevoir des transactions bancaires et de les enrichir / transformer.


Le flux de données est le suivant :


Le service reçoit une transaction de la forme :
```json
{
    "clientId": "1234567890",

    "amount": {

        "value": -10.22,

        "currency": "euros",

    },

    "counterpart": "SCNF VA122345 dt: 01/01/2020",
}
```

```json
{

    "clientId": "1234567890",

    "amount": {

        "value": 150,

        "currency": "euros",

    },

    "counterpart": "papa",
}
```



Les  transactions doivent être envoyée vers un event bus au moment de leur réception puis traitées en mode FIFO et renvoyées vers un webhook une fois la transaction modifiée pour donner ceci :

```json
{

    "clientId": "1234567890",

    "amount": {

        "value": 1022,

        "currency": "EUR",

    },

    "counterpart": "SCNF",

    "rawCounterpart": "SCNF VA122345 dt: 01/01/2020",

    "logo": "/companies/logo-sncf.svg"

    "direction": "DEBIT"

}
```
```json
{
    "clientId": "1234567890",

    "amount": {

        "value": 15000,

        "currency": "EUR",

    },

    "counterpart": "papa",
    "direction": "CREDIT"
}
```

Notes :

    La valeur est maintenant en centimes et toujours positive
    La monnaie est normalisée
    On a ajouté la direction sur crédit / débit basée sur le signe de l’amount initial
    Le logo et la cible de la première transaction on été modifiée. Il n’y a probablement pas de façon élégante de faire ça.


Le code à écrire est simple, ce n’est pas ce sur quoi porte le test. Il faut livrer un repo GitHub public et un déploiement (sur une EC2 du free tier AWS).

Archi : Message queue FIFO  
Kafka  
ressources bonus :   
- https://model-checking.github.io/kani/
- https://github.com/verus-lang/verus
  
Les éléments évalués :


    La documentation du repo est claire
    On peut lancer le projet en local en une commande
    On peut déployer le projet en une commande
    On peut tester le projet en une commande
    On a des TU et des tests d’intégration
    Tout est dans la CI via des actions GitHub
    L’application est dockerisée, cloud agnostic
    L’organisation du repo est claire
    On comprend vite comment il sera possible d’ajouter des règles métier et de use cases
    Bonus : si le code génère de la documentation pour chaque règle métier implémentée (avec rustdoc possiblement)
    Bonus : on a un process pour répliquer le service sur plusieurs datacenters
    Bonus : le serveur est bien configuré ; un nginx avec plusieurs node qui tournent pour l’event bus et le serveur applicatif
