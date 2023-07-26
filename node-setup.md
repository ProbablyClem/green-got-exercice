# Node setup
How to setup a new node in aws

## Prerequisites
AWS account

## Create a new EC2 instance
1. Go to EC2 service
2. Click on "Launch instance"
3. Select "Ubuntu Server 20.04 LTS (HVM), SSD Volume Type" AMI
4. Select "t2.micro" instance type
10. Click on "Review and Launch"

## Instance Setup
1. Select the instance you just created
2. Click on "Connect"
3. Follow the instructions to connect to the instance
4. [Setup docker](https://docs.docker.com/engine/install/ubuntu/)
5. [Setup docker-compose](https://docs.docker.com/compose/install/linux/#install-using-the-repository)
6. run `git clone https://github.com/ProbablyClem/green-got-exercice.git`
7. run `cd green-got-exercice`
8. run `docker-compose up -d --build` to start the service  

## Assign a public IP to the instance
1. Go to EC2 service
2. Select the instance you just created
3. Click on "Actions"
4. Click on "Networking"
5. Click on "Manage IP addresses"
6. Click on "Allocate Elastic IP address"
7. Click on "Allocate"
8. Select the newly created IP address
9. Click on "Actions"
10. Click on "Associate Elastic IP address"
11. Select the instance you just created
12. Click on "Associate"

## Add to load balancer
1. Go to EC2 service
2. Click on "Target groups"
3. Select `target-green-got`
4. Click on "Targets"
5. Select the new instance
6. Click register

## Add to CD pipeline
1. open `.github/workflows/ci_cd.yml`
2. duplicate the `deploy` job
3. change the secrets variables names
4. in the github repo go to `Settings > Secrets`
5. add the secrets variables
6. push to main branch

### Now the new node is ready and will be automatically updated on push to main branch

