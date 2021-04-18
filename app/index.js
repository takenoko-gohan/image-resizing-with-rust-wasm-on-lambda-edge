'use strict';

const http = require('http');
const https = require('https');
const querystring = require('querystring');
const wasm = require("./wasm/image_resize");

const AWS = require('aws-sdk');
const S3 = new AWS.S3({
    signatureVersion: 'v4',
});

// 自環境の S3 バケットを指定してください
const BUCKET = '';

exports.handler = (event, context, callback) => {
    let response = event.Records[0].cf.response;
 
    let request = event.Records[0].cf.request;
    let params = querystring.parse(request.querystring);

    console.log(JSON.stringify(params));

    // パラメーター d の指定がない場合、そのまま返す
    if (!params.d) {
        callback(null, response);
        return;
    }

    // リサイズ後の幅と高さを取得する
    let dimension = params.d.split("x");
    let width = parseInt(dimension[0], 10);
    let height = parseInt(dimension[1], 10);
  
    // URI から必要な情報を取得
    let path = request.uri;

    let key = path.substring(1);

    let prefix, originalKey, match, imageName, requiredFormat;

    try {
        match = key.match(/(.*)\/(.*)/);
        prefix = match[1];

        imageName = match[2];
        requiredFormat = imageName.split(".")[1];
        
        originalKey = prefix + "/" + imageName;
    }
    catch (err) {
        console.log("no prefix present..");
        match = key.match(/(.*)/);

        imageName = match[1];

        requiredFormat = imageName.split(".")[1];
        
        originalKey = imageName;
    }

    // 画像を取得し、リサイズ
    console.log("start getObject");
    S3.getObject({ Bucket: BUCKET, Key: originalKey }).promise()
        .then(data => wasm.resize(data.Body, width, height, requiredFormat))
        .then(result => {
            // generate a binary response with resized image
            response.status = 200;
            response.body = result;
            response.bodyEncoding = 'base64';
            response.headers['content-type'] = [{ key: 'Content-Type', value: 'image/' + requiredFormat }];
            console.log("response :", JSON.stringify(response));
            callback(null, response);
        })
    .catch( err => {
        console.log("Exception while reading source image :%j",err);
    });
};