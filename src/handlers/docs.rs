/*
:project: telegram-onedrive
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

const GREETING: &str = "\
将文件传输到 OneDrive。

转发或上传文件给我，或发送消息链接以从群组或频道传输受限内容。

- /help: 获取帮助。
";

const HELP_BASE: &str = "\
<pre><code>/auth</code></pre>
授权 Telegram 和 OneDrive。
<pre><code>/clear</code></pre>
清除所有历史记录。
<pre><code>/autoDelete</code></pre>
切换机器人是否自动删除消息。
<pre><code>/version</code></pre>
显示版本信息。
";

const HELP_LINKS: &str = "\
<pre><code>/links $message_link $num</code></pre>
传输连续的受限内容。
<pre><code>/links help</code></pre>
显示命令帮助。
";

const HELP_URL: &str = "\
<pre><code>/url $url</code></pre>
通过 URL 上传文件。
<pre><code>/url help</code></pre>
显示命令帮助。
";

const HELP_LOGS: &str = "\
<pre><code>/logs</code></pre>
发送日志打包文件。
<pre><code>/logs clear</code></pre>
清除日志。
<pre><code>/logs help</code></pre>
显示命令帮助。
";

const HELP_DRIVE: &str = "\
<pre><code>/drive</code></pre>
列出所有 OneDrive 账户。
<pre><code>/drive add</code></pre>
添加一个 OneDrive 账户。
<pre><code>/drive $index</code></pre>
切换 OneDrive 账户。
<pre><code>/drive logout</code></pre>
登出当前 OneDrive 账户。
<pre><code>/drive logout $index</code></pre>
登出指定的 OneDrive 账户。
<pre><code>/drive help</code></pre>
显示命令帮助。
";

const HELP_DIR: &str = "\
<pre><code>/dir</code></pre>
显示当前 OneDrive 目录。
<pre><code>/dir $path</code></pre>
设置 OneDrive 目录。
<pre><code>/dir temp $path</code></pre>
设置临时 OneDrive 目录。
<pre><code>/dir temp cancel</code></pre>
将 OneDrive 目录恢复为上一个。
<pre><code>/dir reset</code></pre>
将 OneDrive 目录重置为默认值。
<pre><code>/dir help</code></pre>
显示命令帮助。
";

const INSTRUCTION: &str = "\
- 要传输文件，请转发或上传给我。
- 要传输受限内容，请右键点击内容，复制消息链接，然后发送给我。
- 点击进度消息上的文件名以定位任务。
- 要通过 URL 上传文件，文件响应头必须包含 Content-Length。
- 要取消任务，请删除回复的消息。
- 要取消批量或链接任务，请删除你发送的消息。
- 支持扩展名为 .t2o 的文件作为脚本。

查看 <a href=\"https://github.com/hlf20010508/telegram-onedrive#example\">示例</a>。
";

pub fn format_unknown_command_help(name: &str) -> String {
    format!(
        "{} 的未知命令\n\n用法：\n{}",
        name,
        format_help(name)
    )
}

pub fn format_help(name: &str) -> String {
    match name {
        "/help" => {
            format!(
                "{}{}{}{}{}{}\n{}",
                HELP_BASE, HELP_LINKS, HELP_URL, HELP_LOGS, HELP_DRIVE, HELP_DIR, INSTRUCTION
            )
        }
        "/start" => GREETING.to_string(),
        "/links" => HELP_LINKS.to_string(),
        "/url" => HELP_URL.to_string(),
        "/logs" => HELP_LOGS.to_string(),
        "/drive" => HELP_DRIVE.to_string(),
        "/dir" => HELP_DIR.to_string(),
        _ => String::new(),
    }
}
