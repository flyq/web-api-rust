# tower-web-test

## how to use
terminal 0:
```shell
$ RUST_LOG=chinese_segmentation=info cargo run --release
```

terminal 1:
```shell
$ curl -H "Content-Type: application/json" -X POST -d '{"text":"智能指针是一类数据结构他们的表现类似指 针但是也拥有额外的元数据和功能"}'  0.0.0.0:8081/tokenize
```
the Log out:
```shell
{"words":["一类","也","他们","但是","元","功能","和","拥有","指针","数据","数据结构","是","智能","的","类似","表现","额外"]}
```