# Qpipe

Qpipe 是一个AI工作流工具，目前集成了免费的智谱开放平台-GLM-4模型，用户可以通过自定义YAML格式的文件来设置定时任务工作流组。

## Quickstart

**参数介绍**

```bash
$ Qpipe -h
Usage: Qpipe [OPTIONS]

Options:
  -c, --config <FILE>  Sets a custom config file
  -d, --debug          
  -h, --help           Print help
  -V, --version        Print version
$ Qpipe -c /path/to/config.yaml
```
通过指定YAML文件来声明工作流配置，就可以与AI交互了。

**执行流程**

启动Qpipe后，它将会解析配置文件启动多线程处理任务组，通过每个任务组中的`prompt`对AI进行预设，再由对应的`stream`脚本与本地的AI中转服务进行交互。

**配置文件结构**

```yaml
# model 默认不需要更改 | glm-4-flash 是免费模型
model: "glm-4-flash"
# api_key 通过链接获取： https://open.bigmodel.cn/usercenter/apikeys
api_key: "" 
url: "https://open.bigmodel.cn/api/paas/v4/chat/completions" # 默认不需要更改
server: "127.0.0.1:3000" # AI中转服务监听地址

process_group:
  # 每一个 Group 视为一个任务组，包含了 任务名称、定时任务表达式、Prompt、Stream
  - name: "history_analysis"
    cron: "0/60 * * * * *"
    prompt: >
      你现在是一个Linux操作系统专家，请帮我分析执行的历史命令，给出一段话总结。
    stream: "/path/to/script/history.sh"

  - name: "document_search"
    cron: "now"
    prompt: >
      请在我提供的文档中找出 Fofa/fofa 搜索语句，并返回 搜索语句，按照如下格式输出：
      <Query>{语句}</Query>
      # 举例
      app="abc"
      body="def"
    stream: "/path/to/script/request_doc.py"

```

## 中转服务的交互流程

举例，我有一个配置文件如下：

```yaml
model: "glm-4-flash"
api_key: "secrets" 
url: "https://open.bigmodel.cn/api/paas/v4/chat/completions"
# AI中转服务监听地址
server: "127.0.0.1:3000"

process_group:
  - name: "history_analysis"
    cron: "0/60 * * * * *"
    prompt: >
      你现在是一个Linux操作系统专家，请帮我分析执行的历史命令，给出一段话总结。
    stream: "/path/to/script/history.sh"
```
在Bash脚本中可以先POST请求`127.0.0.1:3000/{name}`，请求体中发送要喂给AI要处理的数据，服务端会在`Header`头中返回`任务ID`，然后再通过`GET`请求读取AI处理的结果（GET请求的Header头中需要携带`任务ID`）。

```bash
#!/bin/zsh
# /path/to/script/history.sh
# 获取当前日期
today=$(date +%Y-%m-%d)

# 执行 atuin history list 命令并过滤出当天的记录，然后保存到一个变量中
today_commands=$(atuin history list | while read -r line; do
    # 从每行中提取日期
    date_in_line=$(echo "$line" | awk '{print $1}')

    # 比较日期是否为今天
    if [[ "$date_in_line" == "$today" ]]; then
        echo "$line"
    fi
done)

# 打印保存的命令历史
echo "$today_commands"

# 使用 curl 发送今天命令历史到服务器，并提取响应头中的 Process-ID
response=$(curl -X POST http://127.0.0.1:3000/history_analysis -d "$today_commands" -i)

# 提取 Process-ID 响应头的值
process_id=$(echo "$response" | grep "Process-ID" | awk '{print $2}' | tr -d '\r')

# 打印 Process-ID
echo "Process ID: $process_id"

curl -X GET http://127.0.0.1:3000/history_analysis -H "Process-ID: $process_id"
```

## 案例

- 数据分析与总结：分析当日系统上执行的命令，并给出文字总结
- 数据提取：提取网络空间搜索引擎语句
- ...

## TODO

-[ ] 支持OpenAI