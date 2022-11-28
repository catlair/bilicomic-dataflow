## manga dataflow

- 生成 Manga 数据流
- 使用 node https 发送数据流

## Types

```typescript
export class Bilicomic {
  /**
   * @param {string | number} mid - 用户 ID
   * @param {string | number} manga_id - 漫画 ID
   * @param {string | number} manga_num - 漫画集数 ID
   */
  constructor(
    mid: string | number,
    manga_id: string | number,
    manga_num: string | number
  );
  /**
   * 发送一次数据包
   * @param {number} n 数据大小，每两个增加一分钟阅读进度，默认 `1`
   * @description 该函数只有两种情况，返回 true 或者 抛出异常。希望由调用者自行处理异常。
   * @returns {Promise<true>} 在没有异常的情况下始终返回 true，即使没有增加时间（因为原本接口就是如此）
   */
  read(n?: number): Promise<true>;
  /**
   * 用于发送的数据
   */
  readonly dataflow: Uint8Array;
}
```

## 使用方式

```js
const { Bilicomic } = require('@catlair/bilicomic-dataflow');

// 自行获取 manga_id, manga_num，如果 mid 太长，可以使用字符串
const bilicomic = new Bilicomic(114514, 123456, 12231);

async function run1() {
  // 发送两次增加 1 分钟，至于检查需要多少时间，以及是否完成，不在本项目的范畴。
  for (let index = 0; index < 32; index++) {
    // 异常处理略
    try {
      await bilicomic.read();
    } catch {}
    // 间隔 1 分钟
    await sleep(1000);
  }
}

async function run2() {
  // 直接一次性完成 30 分钟
  await bilicomic.read(60);
}

run1().catch(console.error);

function sleep(time) {
  return new Promise((resolve) => setTimeout(resolve, time));
}
```

mangaId 和 mangaNum 如何获取？

你可以参考 snippets 目录下定义的代码，其中有获取漫画 ID 和漫画集数 ID 的方法。它们没有被项目用到，但是我将其作为测试用例的一部分，所以保留了。

## 说明

- 本项目仅用于学习交流，不得用于商业用途，下载后请在 24 小时内删除仓库及相关文件。
- 暂时没有完全开放源码的计划（毕竟这些不是光明正大的）。
- 本项目仅使用到 mid，不包含任何其它用户信息，可以放心使用。
