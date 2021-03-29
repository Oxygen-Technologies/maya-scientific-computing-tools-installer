# Maya Scientific Computing Tools Installer
自动帮助你在 maya 中安装 python 的科学计算库。

![avatar](./resource/maya-scientific-computing-tools.png)

## 包含的库
- numpy ```1.16.6```
- scipy ```1.2.3```
- pandas ```0.24.2```
- matplotlib ```2.2.5```

## 测试环境
- Maya 2018

## Usage
> 建议: 请依次先装好```pip``` ```cython``` ```numpy```.
```shell
install.exe MAYA_LOCATION LIBRARY_NAME
```
eg:
```shell
install.exe C:\Maya\Maya2018 numpy
```