Note: I moved these instructions outside of the main README file since the project isn't running in a kubernetes cluster on Google Cloud Platform at the moment.

## Kubernetes Local Setup
- Install kubectl
- Install minikube
- Follow the instructions in the comment at the top of the `kubernetes/secret_template.yaml` file
- Run these commands:
```bash
    eval $(minikube -p minikube docker-env) # To use local images
    docker build -t uvindex-backend services/backend/.
    kubectl apply -f kubernetes/.
    minikube service uvindex-load-balancer --url # Go to this url on a browser
```
