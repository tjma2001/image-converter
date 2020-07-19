const express = require('express');
const fileUpload = require('express-fileupload');
const image = require('./lib/image');

const PORT = process.env.PORT_NUMBER || 9014;
const server = express();

server.use(
  fileUpload({
    limits: 10 * 1024 * 1024,
    useTempFiles: true,
    tempFileDir: './tmp/',
  })
);

server.post('/image', image.upload);
server.get('/image/:id', image.download);
server.use('/', express.static('static'));

server.listen(PORT, () => {
  console.log('server listening on port ', PORT);
});