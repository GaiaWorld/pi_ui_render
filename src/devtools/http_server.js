const http = require('http');
const fs = require('fs');
const path = require('path'); // 引入路径模块

const hostname = '0.0.0.0';
const port = 3000;
 
const server = http.createServer((_req, res) => {
    console.log("_req===", _req.url);

    let url = "src/devtools/web";
    if (_req.url == "/") {
        url += "/" + "index.html"
    } else {
        url += _req.url;
    }
   
    // 读取并返回HTML文件
    fs.readFile(url, (err, data) => {
        if (err) {
            res.writeHead(404);
            res.end('Server error' + err);
        } else {
            if (url.endsWith(".html")) {
                // 设置响应头为HTML类型
                res.writeHead(200, {'Content-Type': 'text/html'});
            } else if (url.endsWith(".js")) {
                res.writeHead(200, {'Content-Type': 'application/javascript'});
            }
            
            res.end(data); // 发送内容到客户端
        }
    });
});
 
server.listen(port, hostname, () => {
    console.log(`Server running at http://${hostname}:${port}/`);
});