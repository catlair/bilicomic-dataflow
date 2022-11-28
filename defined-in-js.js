const https = require('https');
const { gunzipSync } = require('zlib');

const ua =
  'Dalvik/2.1.0 (Linux; U; Android 7.1.2; vivo50 Build/NZH54D) 4.18.1 os/android model/vivo50 mobi_app/android_comic build/36418010 channel/bilicomic innerVer/36418010 osVer/7.1.2 network/2';

function request(host, path, data, headers = {}) {
  return new Promise((resolve, reject) => {
    const req = https.request(
      {
        method: 'POST',
        host,
        path,
        port: 443,
        headers: {
          'User-Agent': ua,
          'Accept-Encoding': 'gzip',
          ...headers,
          'Content-Length': Buffer.from(data).byteLength,
        },
      },
      (res) => {
        if (res.statusCode !== 200) {
          reject(new Error(`${res.statusMessage}: ${res.statusCode}`));
          return;
        }
        let data = Buffer.alloc(0);
        res.on('data', (d) => {
          data = Buffer.concat([data, d]);
        });
        res.on('end', () => {
          if (res.headers['content-encoding'] === 'gzip') {
            data = gunzipSync(data);
          }
          if (res.headers['content-type'] === 'application/json') {
            resolve(JSON.parse(data.toString()));
            return;
          }
          resolve(data);
        });
        res.on('error', (e) => {
          console.log('error: ' + e);
          reject(e);
        });
      }
    );
    req.write(data);
    req.end();
  });
}

function stringify(entries) {
  const searchParams = new URLSearchParams();
  if (!Array.isArray(entries)) {
    entries = Object.entries(entries);
  }
  entries.forEach(([key, value]) => {
    if (Object.prototype.toString.call(value) === '[object Object]') {
      searchParams.append(key, JSON.stringify(value));
      return;
    }
    searchParams.append(key, String(value));
  });
  return searchParams.toString();
}

module.exports.fetch = async function fetch(host, path, data, contentType) {
  const buf = Buffer.from(data);
  const res = await request(host, path, buf, {
    'Content-Type': contentType,
  });
  if (res?.toString() === 'ok') {
    return true;
  }
  return false;
};

module.exports.getMangaId = async function getMangaId() {
  try {
    const res = await request(
      'manga.bilibili.com',
      '/twirp/user.v1.Season/GetSeasonInfo',
      stringify({
        is_teenager: 0,
        no_recommend: 0,
        take_type: 1,
        mobi_app: 'android_comic',
        ts: new Date().getTime(),
      }),
      {
        'Content-Type': 'application/x-www-form-urlencoded',
        'User-Agent': ua,
        Referer: 'https://manga.bilibili.com/',
        'Accept-Encoding': 'gzip',
      }
    );
    if (!res || res.code !== 0) {
      return;
    }
    return res.data?.today_tasks?.find((task) => task && task.comics?.length)
      ?.comics?.[0]?.comic_id;
  } catch {
    return 26536;
  }
};

module.exports.getMangaNum = async function getMangaNum(comic_id) {
  try {
    const res = await request(
      'manga.bilibili.com',
      '/twirp/comic.v1.Comic/ComicDetail',
      stringify({
        device: 'android',
        version: '4.16.0',
        comic_id,
      }),
      {
        'Content-Type': 'application/x-www-form-urlencoded',
        'User-Agent': ua,
        Referer: 'https://manga.bilibili.com/',
        'Accept-Encoding': 'gzip',
      }
    );
    if (!res || res.code !== 0) {
      return;
    }
    return res.data?.ep_list?.[0]?.id;
  } catch {
    return 114514;
  }
};
