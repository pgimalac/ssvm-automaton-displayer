const { regex_to_svg } = require('../pkg/ssvm_nodejs_starter_lib.js');

const http = require('http');
const url = require('url');
const hostname = '0.0.0.0';
const port = 3000;

const server = http.createServer((req, res) => {
    const queryObject = url.parse(req.url,true).query;
    res.end(regex_to_svg("a.+b*+cd?", 0, 0) + '\n');
});

server.listen(port, hostname, () => {
    console.log(`Server running at http://${hostname}:${port}/`);
});
