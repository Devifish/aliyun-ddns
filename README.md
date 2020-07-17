# aliyun-ddns

[![Build status](https://ci.appveyor.com/api/projects/status/clt0992pxvu1kveo?svg=true)](https://ci.appveyor.com/project/Devifish/aliyun-ddns)
[![Docker status](https://img.shields.io/docker/stars/devifish/aliyun-ddns.svg?logo=docker)](https://hub.docker.com/r/devifish/aliyun-ddns)
[![GitHub license](https://img.shields.io/github/license/Devifish/aliyun-ddns.svg)](https://github.com/Devifish/aliyun-ddns/blob/master/LICENSE)
[![GitHub Release](https://img.shields.io/github/release/Devifish/aliyun-ddns.svg)](https://github.com/Devifish/aliyun-ddns/releases)

阿里云DDNS动态域名工具，支持IPv6解析、Docker环境运行

使用Rust构建，媲美 C/C++ 的运行效率和内存利用率

仅占用低至1MB内存，意味着可在路由器,IoT等嵌入式设备上高效使用

## 使用方法
### 命令行
```
aliyun-ddns --help
```
### Docker
```
docker pull devifish/aliyun-ddns
```

### 说明

1. 如果指定的域名在阿里云上没有做解析，并且本地支持IPv6，则将基于当前域名新增A(IPv4)与AAAA(IPv6)解析
2. 如果阿里云上已经存在一条解析，则只会更新这条解析记录，当前解析是否是A或AAAA取决于当前已经存在阿里云上的解析类型
3. 如果当前值与阿里云解析的IP值不匹配时，且当前解析为disable状态，则会更新解析对应的IP地址，并设置状态为enable
4. 如果解析的IP地址相同，且解析状态为disable时，不做任何更改