mod helper;

use wasm_bindgen::prelude::*;
use helper::{create_read_dataflow,read_manga};

#[wasm_bindgen(typescript_custom_section)]
const TS_BILICOMIC: &'static str = r#"
export class Bilicomic {
    /**
     * @param {string | number} mid - 用户 ID
     * @param {string | number} manga_id - 漫画 ID
     * @param {string | number} manga_num - 漫画集数 ID
     */
    constructor(mid: string | number, manga_id: string | number, manga_num: string | number);
    /**
     * 发送一次数据包
     * @param {number} 数据大小，每两个增加一分钟阅读进度，默认 1
     * @description 该函数只有两种情况，返回 true 或者 抛出异常。希望由调用者自行处理异常。
     * @returns {Promise<true>} 在没有异常的情况下始终返回 true，即使没有增加时间（因为原本接口就是如此）
     */
    read(n?: number): Promise<true>;
    /**
     * 用于发送的数据
     */
    readonly dataflow: Uint8Array;
  }
"#;

#[wasm_bindgen(skip_typescript)]
pub struct Bilicomic {
    dataflow: Vec<u8>,
}

#[wasm_bindgen]
impl Bilicomic {
    #[wasm_bindgen(constructor)]
    pub fn new(mid: String, manga_id: String, manga_num: String) -> Bilicomic {
        let now = js_sys::Date::now() as i64;
        let dataflow = create_read_dataflow(&manga_id, &manga_num, &mid, now);
        Bilicomic {
            dataflow,
        }
    }

    #[wasm_bindgen(js_name = "read")]
    pub async fn read_manga(&self, n:i32) -> Result<JsValue, JsValue> {
        let mut dataflow = Vec::new();
        for _ in 0..n {
            dataflow.extend_from_slice(&self.dataflow);
        }
        read_manga(dataflow).await
    }

    #[wasm_bindgen(getter = dataflow)]
    pub fn get_dataflow(&self) -> Vec<u8> {
        self.dataflow.clone()
    }
}
