# aliyun-ddns

[![Build status](https://ci.appveyor.com/api/projects/status/clt0992pxvu1kveo?svg=true)](https://ci.appveyor.com/project/Devifish/aliyun-ddns)
[![Docker status](https://img.shields.io/docker/stars/Devifish/aliyun-ddns.svg?logo=docker)](https://hub.docker.com/r/Devifish/aliyun-ddns)
[![GitHub license](https://img.shields.io/github/license/Devifish/aliyun-ddns.svg)](https://github.com/Devifish/aliyun-ddns/blob/master/LICENSE)
[![GitHub Release](https://img.shields.io/github/release/Devifish/aliyun-ddns.svg)](https://github.com/Devifish/aliyun-ddns/releases)

阿里云DDNS动态域名工具，支持IPv6解析、Docker环境运行。

使用Rust构建，媲美 C/C++ 的运行效率和内存利用率。低至1MB内存占用，意味着可在路由器等IoT设备上流畅运行

## 使用方法
### 命令行
```
aliyun-ddns --help
```
### Docker
```
docker pull devifish/aliyun-ddns
```