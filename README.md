# IMAGE CONVERT

This is a small webservice that converts an image from whatever format it is, into `webp` format. Safari on mobile and desktop below version 14 do not like this format.

`Imagemagik` needs to be installed.

The service uses nodejs to provide the webserver and does all the processing via imagemagik using a `rust` submodule.

- Still needs to be dockerised ad made more `production` ready.
- Still needs to figure out how to build this while keeping its size as small as possible.
