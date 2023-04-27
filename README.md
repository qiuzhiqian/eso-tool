# eso-tool
这是一个用rust编写的用来辅助解码eso的资源链接源的工具。主要处理策略是将源链接中的base64压缩码部分进行解码和解压缩。

通过分析链接码的生成规则可以发现，主要是采取了将json源进行了gzip压缩，然后将压缩后的数据通过base64进行编码，最后拼接成eso链接头。

如下简单描述了eso链接的压缩生成规则。
```
{“xxx”:xxxx} ==gzip compress=> binary code ===base64 encode===> base Ascii ===esc://xxx@===> esc://xxx@xxxx
```

按照这个规则进行反编码即可得到原始的json码了。

## eso
eso是一个开源的跨平台阅读类软件，支持支持自定义源。详细信息可以查看官网资料
- [eso_source](https://github.com/mabDc/eso_source)
- [eso](https://github.com/mabDc/eso)

## 编译
```
cd eso-tool
cargo build
```

## 运行
```
eso-tool decode --file /xxx/xxx.txt --prefix @
```