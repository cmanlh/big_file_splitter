# big_file_splitter

#### 介绍
一个用于将大文件切割成多份小文件的命令行工具

#### 下载

可以前往[发行页面](https://gitee.com/lifeonwalden/big_file_splitter/releases)下载最新版本，目前仅支持windows.

#### 使用说明

##### 切割文件

```
big_file_splitter --target=${path_to_file}
```
命令执行后，文件将按照默认设置，以行为单位，切割成每个文件包含1024行的多个小文件

##### 参数说明

```
merge : 加上该参数则为合并文件

--target : 切割或者合并的目标文件的路径

--type : 切割单位， 可选值为：line(按照行切割，默认)；size(按照数据大小)

--size : 每个切分后文件最大的大小。如切割单位为行，则该参数指行数；如切割单位为大小，则该参数指字节数。支持k,m,g后缀。k=1024, m=1024*1024, g=1024*1024*1024
```

#### 示例

##### 将拥有260行的目标文件(c:\test.txt)切割成最多包含128行的小文件
```
big_file_splitter --target=c:\test --size=128
```
命令执行后，原目录下将出现三个新文件：c:\test.txt_0 , c:\test.txt_1 , c:\test.txt_2


##### 将拥有260M的目标文件(c:\test.txt)切割成最多包含128M的小文件
```
big_file_splitter --target=c:\test --type=size --size=128M
```
命令执行后，原目录下将出现三个新文件：c:\test.txt_0 , c:\test.txt_1 , c:\test.txt_2

##### 合并文件（假设有切割后的三个子文件：c:\test.txt_0 , c:\test.txt_1 , c:\test.txt_2）
```
big_file_splitter --target=c:\test.txt_0
```
命令执行后，原目录将出现合并的新文件：c:\test.txt