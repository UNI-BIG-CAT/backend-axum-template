# 生成数据库entities
```
.env 
DATABASE_URL=postgres://radar:radar123@localhost/radar_db
```
```
运行 
sea-orm-cli generate entity --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' -o ./src/repository/entity
```

# 生成pcks8公私钥
```
openssl genpkey -algorithm RSA -out password-private-key.pem -pkeyopt rsa_keygen_bits:2048
openssl rsa -in password-private-key.pem -pubout -out password-public-key.pem

openssl genpkey -algorithm RSA -out jwt-private-key.pem -pkeyopt rsa_keygen_bits:2048
openssl rsa -pubout -in jwt-private-key.pem -out jwt-public-key.pem
```