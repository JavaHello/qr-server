# QR SERVER

带有签名的 `qr-server` 服务

- 签名结构:

```txt
URL: http://127.0.0.1:3000/qr?32c8f305919315a8ef5088d30369c733hello%20world

query[..32]: md5(qr内容)
query[32..]: qr内容
```

## 使用

1. 启动 `qr-server`

```sh
qr-server
server start port: 3000
```

2. 使用 `qr-cli` 生成链接

```sh
qr-cli "hello world"
http://127.0.0.1:3000/qr?32c8f305919315a8ef5088d30369c733hello%20world
```
