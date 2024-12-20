- To run postgres instance 

    

```
docker run -d --name=pg1 -p 55432:5432 -e POSTGRES_USER=admin -ePOSTGRES_PASSWORD=admin -e POSTGRES_DB=fooddb postgres
```

- To run database UI adminer

```
docker run --name dbui -p 8089:8080 -d adminer
```
 - Table script

 ```
 CREATE TABLE foods (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at BIGINT DEFAULT EXTRACT(EPOCH FROM CURRENT_TIMESTAMP)::BIGINT,
    updated_at BIGINT DEFAULT EXTRACT(EPOCH FROM CURRENT_TIMESTAMP)::BIGINT
);
```


-- To run docker command

```
docker build . -t jpalaparthi/my_app:v0.1
```

```
docker run -d --name my_app1 -p 58084:8084 -e DATABASE_URL=postgres://admin:admin@172.17.0.2:5432/fooddb jpalaparthi/my_app:v0.1
```

- To push to docker
- First get docker credentials (if don't have credentials , signup in docker hub(hub.docker.com))
- docker login (provide username and password)
- before going to push there must be an image with the name. Make sure the name of the image starts with your username.(jpalaparthi is my username)

```
docker push jpalaparthi/http-demo:v0.1
```

- minikube (need to install minikube, google and do it)

```
minikube start
```