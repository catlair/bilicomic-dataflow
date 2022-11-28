const fs = require('fs');
const path = require('path');

// 读取 package.json 文件
const pkg = require('../pkg/package.json');

const name = pkg.name,
  _name = name.replace(/-/g, '_');

// name 添加前缀 @catlair/
pkg.name = `@catlair/${name}`;
pkg.files.push('snippets/**/*.js');
// @ts-ignore
pkg.publishConfig = {
  access: 'public',
  registry: 'https://registry.npmjs.org/',
};

// 写入 package.json
fs.writeFileSync(
  path.join(__dirname, '../pkg/package.json'),
  JSON.stringify(pkg, null, 2)
);

// 读取 name .js 文件
const jsPath = path.resolve(__dirname, `../pkg/${_name}.js`);
let js = fs.readFileSync(jsPath, 'utf8');

js = js.replace(
  `    /**
    * @param {string} mid
    * @param {string} manga_id
    * @param {string} manga_num
    */
    constructor(mid, manga_id, manga_num) {`,
  `    /**
    * @param {number | string} mid
    * @param {number | string} manga_id
    * @param {number | string} manga_num
    */
    constructor(mid, manga_id, manga_num) {
        mid = String(mid);
        manga_id = String(manga_id);
        manga_num = String(manga_num);`
);

js = js.replace(`read(n)`, `read(n = 1)`);
// 写入 .js
fs.writeFileSync(jsPath, js);
