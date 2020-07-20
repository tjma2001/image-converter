var { convert } = require('../native');
var fs = require('fs');

module.exports.upload = (req, res) => {
  if (!req.files || Object.keys(req.files).length === 0) {
    return res.status(400).send('No files were uploaded.');
  }

  const file = req.files.file;
  const message = convert(file.tempFilePath, file.md5, 800);

  res.status(200).send(message);
};

module.exports.download = (req, res) => {
  const fileName = req.params.id;
  const path = './output/' + fileName;
  const file = fs.createReadStream(path);

  file.on('end', function () {
    console.log('file end');
    const result = fs.unlinkSync(path);
    console.log('delete file result: ', result);
  });
  file.on('error', (error) => console.error(error));
  file.pipe(res);
};
