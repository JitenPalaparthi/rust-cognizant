- To download minikube 

https://minikube.sigs.k8s.io/docs/start/?arch=%2Fmacos%2Farm64%2Fstable%2Fbinary+download

1. kubectl apply -f kube/1-namespace.yaml 
2. kubectl apply -f kube/2-deployment.yaml
3. kubectl apply -f kube/3-service.yaml

4. minikube service actix-web-app-service -n actix-web-app