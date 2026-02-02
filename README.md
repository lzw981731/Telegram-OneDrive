
# telegram-onedrive

* 一个用于将文件传输到 OneDrive 的 Telegram 机器人。
* 源代码https://github.com/hlf20010508/telegram-onedrive
## 这是一个汉化版本
* 增加OpenList下载地址功能

## 注意事项 (Attention)

* **请仔细阅读 [准备工作](https://www.google.com/search?q=%23%E5%87%86%E5%A4%87%E5%B7%A5%E4%BD%9C-preparation)，不要遗漏任何步骤。**
* **请阅读 [使用方法 - 开始之前](https://www.google.com/search?q=%23%E5%BC%80%E5%A7%8B%E4%B9%8B%E5%89%8D---%E9%87%8D%E8%A6%81-before-start-important)，否则机器人可能无法工作。**

## 简介 (Introductions)

* 基于 [gramme.rs](https://github.com/Lonami/grammers) 开发。
* 仅支持在 **群组** 中工作。
* 传输你发送或转发的文件。
* 支持传输 **受限内容**（禁止转发/保存的内容）。
* 支持通过 URL 链接传输文件。
* **无文件大小限制**。
* **不占用本地空间**，通过分片传输（multipart transfer）完全在内存中运行。
* 支持多个 OneDrive 账户。
* 支持更改 OneDrive 存储目录。
* 支持多任务并行处理。

## 演示 (Demos)

<details>
<summary>文件传输</summary>
<img src="https://github.com/user-attachments/assets/e3f62c2e-c562-4018-84bd-235f351d6da7" alt="files">
</details>
<details>
<summary>消息链接传输</summary>
<img src="https://github.com/user-attachments/assets/e1a00bde-9fdb-42ec-b020-dc7ee1cd9c91" alt="link">
</details>
<details>
<summary>URL 传输</summary>
<img src="https://github.com/user-attachments/assets/74c8f895-8768-437d-ad09-f0d5b2e8783d" alt="url">
</details>

## 账户类型支持 (Account Types)

### 支持

* 个人账户 (Personal account)。
* 所有类型的商业账户 (Business accounts)，[详情见此](https://learn.microsoft.com/en-us/office365/servicedescriptions/office-365-platform-service-description/office-365-platform-service-description#feature-availability-across-some-plans)。
* 存在域管理员的各类教育账户 (Educational accounts)。

### 不支持

* **不存在** 域管理员的各类教育账户。

### 暂不支持

* 由世纪互联 (21Vianet) 运营的 Microsoft 365。

---

## 准备工作 (Preparation)

请按照以下顺序依次获取凭证并配置环境。

### 第一步：获取 Telegram 凭证

1. **创建机器人**：通过 [BotFather](https://t.me/BotFather) 创建一个新的 Telegram 机器人。
* 记录获得的 `token`。


2. **创建应用**：在 [my.telegram.org](https://my.telegram.org) 创建一个 Telegram 应用（详见 [教程](https://docs.telethon.dev/en/stable/basic/signing-in.html)）。
* 记录 `api_id`。
* 记录 `api_hash`。
* 确认你登录该网站使用的手机号码（国际格式，如 `+8613800000000`）。



### 第二步：确定域名与网络配置

本服务需要一个 **HTTPS** 地址来接收回调。

1. **确定 `server_uri**`：
* 这是你的访问域名，例如 `https://example.com`。
* 如果你没有域名，也可以使用 IP，例如 `https://127.0.0.1:xxxx`。
* **注意**：协议必须是 `https`。


2. **SSL 证书配置**（二选一）：
* **方案 A：使用反向代理（推荐）**
* 如果你使用 Nginx 等反向代理，请在代理层配置 SSL，并将流量转发给机器人。
* Nginx 配置示例：
```nginx
server {
    listen 443 ssl;
    server_name example.com;

    ssl_certificate path/to/public.pem;
    ssl_certificate_key path/to/private.key;

    location / {
        proxy_pass http://127.0.0.1:8080/; # 转发到容器端口
    }
}

```




* **方案 B：直接挂载证书**
* 如果你没有 Web 服务器，需要将证书直接挂载到容器中（见下文 `docker-compose.yml` 配置）。
* 注意：自签名证书可能会被部分浏览器拦截。





### 第三步：获取 OneDrive (Azure) 凭证

1. 前往 [portal.azure.com](https://portal.azure.com/#view/Microsoft_AAD_RegisteredApps/ApplicationsListBlade) 的“应用注册 (App registrations)”。
2. 点击 **新注册 (New registrations)**。
3. **配置应用**：
* **名称 (Name)**：随意填写。
* **受支持的账户类型**：选择 `任何组织目录(任何 Microsoft Entra ID 租户 - 多租户)中的账户和个人 Microsoft 账户`。
* **重定向 URI (Redirect URI)**：
* 平台选择 `Web`。
* 地址填写：`你的域名/auth`（例如 `https://example.com/auth`）。**必须与上一步确定的 `server_uri` 域名一致。**




4. 点击 **注册 (Register)**。
5. **获取 ID**：在应用“概览”页，记录 `应用程序(客户端) ID (Application (client) ID)`。
6. **获取密钥**：
* 前往 `证书和密码 (Certificates & secrets)` -> `客户端密码 (Client secrets)` -> `新客户端密码 (New client secret)`。
* 填写说明并选择有效期，点击添加。
* **重要**：立即记录 `值 (Value)`（注意不是 ID，是那一串密钥值）。



### 第四步：编辑配置文件

打开 `docker-compose.yml` 并填入上述步骤获取的信息。

```yaml
version: "3"
services:
  telegram-onedrive:
    image: lzw981731/telegram-onedrive
    container_name: telegram-onedrive
    restart: always
    ports:
      - "8080:8080" # 左侧端口可修改，需与反代配置一致
    volumes:
      - ./config:/config
      - ./logs:/logs
      # 仅在使用“方案B：直接挂载证书”时取消以下注释
      # - /path/to/*.crt:/ssl/server.crt
      # - /path/to/*.key:/ssl/server.key
    environment:
      # --- 必需配置 ---
      - server_uri=https://example.com      # 你的域名，必须以 https 开头
      - tg_bot_token=123456:ABC-DEF...      # 步骤一获取的 Bot Token
      - tg_api_id=1234567                   # 步骤一获取的 API ID
      - tg_api_hash=abcdef123456...         # 步骤一获取的 API Hash
      - tg_user_phone=+8613800000000        # 步骤一用的手机号
      - od_client_id=xxxx-xxxx-xxxx         # 步骤三获取的 Client ID
      - od_client_secret=xxxx-xxxx-xxxx     # 步骤三获取的 Client Secret Value

      # --- 推荐配置 ---
      - tg_user_name=user1,user2            # 你的 Telegram 用户名（不带@），逗号分隔。
                                            # 如果不设置，任何人都能控制你的机器人！

      # --- 可选配置 ---
      - od_root_path=/                      # OneDrive 存储根目录，默认为 /
      - auto_delete=false                   # 是否自动删除消息，默认为 false
      - reverse_proxy=true                  # 如果使用 Nginx 反代（方案A），设为 true
      - tg_user_password=                   # 如果开启了 2FA，在此填入两步验证密码
      - OpenList=https://xxxxxx  #配置OpenList下载地址，如果不设置机器人不会返回下载信息

      # --- 开发调试配置 (一般无需修改) ---
      - port=8080
      - trace_level=info
      - worker_num=5

```

---

## 使用方法 (Usage)

### 开始之前 - 重要！ (Before Start (Important!))

1. 创建一个 Telegram 群组。
2. 在机器人的资料页，点击 `添加到群组或频道 (Add to Group or Channel)`。
3. 将此机器人添加到你的群组中。
4. **关键步骤**：将此机器人设为 **管理员 (Admin)**，并给予下图所示的所有权限：
<img width="330" alt="image" src="https://github.com/hlf20010508/telegram-onedrive/assets/76218469/d5fc1130-493e-47fb-9c45-67c328470692">

如果不遵循这些步骤，机器人可能无法正常工作。

### 授权步骤 (Authorization Steps)

1. 在群组中发送 `/auth`。
2. 等待片刻，你将收到来自 Telegram 官方账号的登录验证码。
3. 访问机器人回复的链接，输入收到的验证码。
4. 提交后，系统会自动跳转或发送 OneDrive 的授权链接。点击并登录微软账号进行授权。
5. 如果机器人提示 `Onedrive authorization successful!`（OneDrive 授权成功！），则一切准备就绪。

### 开始传输 (Start)

* **普通文件**：直接在群组中转发或上传文件（视频、照片、GIF、贴纸、语音）。
* **受限内容**：如果想传输禁止转发/保存的内容，请右键点击该消息，复制 **消息链接 (Message Link)**，然后发送该链接给机器人。
* **查看进度**：机器人会回复最新消息以显示进度。
* **更多指令**：使用 `/help` 获取完整指令列表。

## 机器人指令 (Bot Command)

* `/start` 启动机器人。
* `/auth` 授权 Telegram 和 OneDrive。
* `/clear` 清除历史记录。
* `/autoDelete` 切换机器人是否自动删除消息。
* `/drive` 列出所有 OneDrive 账户。
* `/drive add` 添加一个 OneDrive 账户。
* `/drive $index` 切换当前的 OneDrive 账户（$index 为账户序号）。
* `/drive logout` 登出当前 OneDrive 账户。
* `/drive logout $index` 登出指定的 OneDrive 账户。
* `/links $message_link $range` 传输连续的受限内容（$range 为数量）。
* `/url $file_url` 通过 URL 上传文件。
* `/logs` 发送日志文件。
* `/logs clear` 清除日志。
* `/dir` 显示当前 OneDrive 目录。
* `/dir $path` 设置 OneDrive 目录。
* `/dir temp $path` 设置临时 OneDrive 目录。
* `/dir temp cancel` 将 OneDrive 目录恢复为上一个。
* `/dir reset` 重置 OneDrive 目录为默认值。
* `/version` 显示版本信息。
* `/help` 获取帮助。

### 实验性功能 (Experimental Features)

* **批处理脚本**：支持扩展名为 `.t2o` 的文件。你可以使用它们来自动化机器人操作。
* **取消任务**：删除机器人回复的那条进度消息即可取消任务。
* **取消批处理/链接任务**：删除你发送的那条指令消息。

### 示例 (Example)

* `/links https://t.me/c/xxxxxxx/100 2`
* 将传输 `.../100` 和 `.../101` 两条消息的内容。


* `/url https://example.com/file.txt`
* 将上传 `file.txt`。**注意：** 目标服务器必须在响应 Header 中包含 `Content-Length`。


* **脚本示例** (`example.t2o` 内容)：
```text
https://t.me/xxxx/100
/links https://t.me/yyyy/200 2
/autoDelete
/dir temp /files
/url https://example.com/file.txt

```



## 通过 Docker 启动 (Launch Through Docker)

配置完成后，使用以下命令启动：

```sh
sudo docker compose up -d

```

## 链接 (Links)

* [Docker Hub](https://hub.docker.com/repository/docker/hlf01/telegram-onedrive)
