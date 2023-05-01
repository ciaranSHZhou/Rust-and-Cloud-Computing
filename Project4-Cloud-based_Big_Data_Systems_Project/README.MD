# Project4-Cloud-based Big Data Systems Project

## Overview
Use cloud-based tool, specifically AWS, to build a machine learning model to predict diabetes. I used a [diabetes dataset from Kaggle](https://www.kaggle.com/datasets/akshaydattatraykhare/diabetes-dataset)  

## 1. Data Processing
* Upload the csv file to S3 Bucket
<img width="642" src="/S4 Upload.jpg">

* Split the data into 70% training set, 20% validation set, and 10% testing set



### 2. Test the project locally.

* To compile the rust project, type:
  `cargo build`

* To run the project, type:
  `cargo run` 
  
  <img width="642" src="screenshots/1.jpg">
  
  add /kanye to the address for the actual recommendation  
  
  <img width="642" src="screenshots/2.jpg">

## 3. Deploy the project on the cloud via AWS App Runner

* 1. Deploy the codebase on AWS Cloud9.

* 2. Create a repository on AWS Amazon Elastic Container Registry.

* 3. Push the image by running commands according to "View push commands".

* 4. Create a new service in AWS App Runner.
<img width="642" src="screenshots/3.jpg">


## 4. Deploy the project via Minikube

* 1.  Push the docker to the Dockerhub, i.e. 
`docker build -t <hub-user>/<repo-name>[:<tag>]` and `docker push <hub-user>/<repo-name>:<tag>`. For example, my repo: https://hub.docker.com/repository/docker/ciaranshz/ids721_project2/  
<img width="642" src="screenshots/6.jpg">

* 2.  `minikube start`  

* 3.  `minikube dashboard --url` for the Kubernetes Dashboard  
<img width="642" src="screenshots/4.jpg">

* 4.  Create a deployment: `kubectl create deployment kanye --image=registry.hub.docker.com/ciaranshz/ids721_project2`  

* 5.  View deployment: `kubectl get deployments`  

* 6.  Create service and expose it: `kubectl expose deployment kanye --type=LoadBalancer --port=8080`  

* 7.  View services: `kubectl get service kanye`  

* 8.  `minikube service kanye --url`  

* 9.  Curl web service `curl [url get from last step]`  
<img width="642" src="screenshots/5.jpg">


* 10. Cleanup

`kubectl delete service kanye`
  
`kubectl delete deployment kanye`
  
`minikube stop`
