const { Bilicomic } = require('../pkg/bilicomic_dataflow');
const { getMangaId, getMangaNum } = require('../defined-in-js');

(async () => {
  try {
    const mangaId = await getMangaId();
    const mangaNum = await getMangaNum(mangaId);
    const comic = new Bilicomic('3461581908216494', mangaId, mangaNum);
    const dataflow = comic.dataflow;
    console.log(dataflow.slice(dataflow.length - 6, dataflow.length));

    const ok = await comic.read();
    console.log(ok);
  } catch (error) {
    console.log(error);
  }
})();
