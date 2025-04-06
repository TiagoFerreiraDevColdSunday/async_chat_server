## Async Chat Server in Rust



## Create a password/private/public key step-by-step linux:

Must do this for the server to work (for now). On the file keys do the following steps:

Create a private key do:
```shell 
openssl genrsa -out private_key.pem 2048
```

Create a public key do:
```shell 
openssl rsa -in private_key.pem -pubout -out public_key.pem
```

Create a text file with your desired password.

Then encrypt it with:
```shell 
openssl rsautl -encrypt -inkey public_key.pem -pubin -in password.txt -out password.enc
```

At this point the server will start reading the private key and the encrypted password.

Whenever you want to decrypt it again use:
```shell 
openssl rsautl -decrypt -inkey private_key.pem -in password.enc -out decrypted_password.txt
```