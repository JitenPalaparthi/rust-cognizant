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