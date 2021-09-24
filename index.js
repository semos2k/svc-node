const http = require('http');
const url = require('url');

const API_HOSTNAME = process.env.SVC_API_HOSTNAME || 'node-app.api.svc.cluster.local';
const API_PORT = parseInt(process.env.SVC_API_PORT || '4000');

function options(url) {
    return {
        hostname: API_HOSTNAME,
        port: API_PORT,
        path: url,
        method: 'GET',
        agent: new http.Agent({ keepAlive: true, maxSockets: 2000, scheduling: 'fifo' })
    }
}

http.createServer(async function (req, res) {
    //console.log(req.headers);
    //console.log(req.method, req.url);

    let url_parts = url.parse(req.url, true);
    let query = url_parts.query;

    const request = http.request(options('/?number=' + query.number), response => {
        //console.log(`statusCode: ${response.statusCode}`)
        let data = '';

        response.on('data', chunk => {
            data += chunk;
        })

        response.on('end', () => {
            res.setHeader('Content-Type', 'application/json');
            res.write(data.replace('}', ', "test": "nuevo0" }'));
            res.end();
        });
    })

    request.on('error', error => {
        console.error(error)

        res.writeHead(500);
        res.write('Error request to api');
        res.end();
    })

    request.end();
    //end the response
}).listen(5000);

console.log(`Worker ${process.pid} started`);