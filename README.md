## 机核RSS加速

机核电台rss生成

1. 云函数请求机核api获取json
2. json转xml
3. 字符串存储到静态存储

#### Build 函数
AWS
```
make upload_aws
```
Aliyun
```
make upload_ali
```
#### 测试
AWS
```
make invoke_aws
```
Aliyun
```
make invoke_ali
```

#### Resource
机核api：https://www.gcores.com/gapi/v1/radios
RSS feed: http://feed.tangsuanradio.com/gadio.xml
