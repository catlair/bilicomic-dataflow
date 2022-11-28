
use wasm_bindgen::prelude::*;

const I_ARR: [i32; 256] = [
  234, 212, 150, 168, 18, 44, 110, 80, 127, 65, 3, 61, 135, 185, 251, 197, 165,
155, 217, 231, 93, 99, 33, 31, 48, 14, 76, 114, 200, 246, 180, 138, 116, 74,
8, 54, 140, 178, 240, 206, 225, 223, 157, 163, 25, 39, 101, 91, 59, 5, 71,
121, 195, 253, 191, 129, 174, 144, 210, 236, 86, 104, 42, 20, 179, 141, 207,
241, 75, 117, 55, 9, 38, 24, 90, 100, 222, 224, 162, 156, 252, 194, 128, 190,
4, 58, 120, 70, 105, 87, 21, 43, 145, 175, 237, 211, 45, 19, 81, 111, 213,
235, 169, 151, 184, 134, 196, 250, 64, 126, 60, 2, 98, 92, 30, 32, 154, 164,
230, 216, 247, 201, 139, 181, 15, 49, 115, 77, 88, 102, 36, 26, 160, 158, 220,
226, 205, 243, 177, 143, 53, 11, 73, 119, 23, 41, 107, 85, 239, 209, 147, 173,
130, 188, 254, 192, 122, 68, 6, 56, 198, 248, 186, 132, 62, 0, 66, 124, 83,
109, 47, 17, 171, 149, 215, 233, 137, 183, 245, 203, 113, 79, 13, 51, 28, 34,
96, 94, 228, 218, 152, 166, 1, 63, 125, 67, 249, 199, 133, 187, 148, 170, 232,
214, 108, 82, 16, 46, 78, 112, 50, 12, 182, 136, 202, 244, 219, 229, 167, 153,
35, 29, 95, 97, 159, 161, 227, 221, 103, 89, 27, 37, 10, 52, 118, 72, 242,
204, 142, 176, 208, 238, 172, 146, 40, 22, 84, 106, 69, 123, 57, 7, 189, 131,
193, 255
];

fn get_check_num(i: i32) -> u8 {
  return (I_ARR[((i >> 24) & 255 ^ I_ARR[(I_ARR[(I_ARR[(i & 255) as usize] ^ (i >> 8) & 255) as usize] ^ (i >> 16) & 255) as usize]) as usize] & 255) as u8;
}

fn get_length(i: i32) -> i32 {
  return (1 << 31) | i;
}

fn to_varint(x: i64) -> Vec<u8> {
  let mut buf: Vec<u8> = Vec::new();
  let mut x = x;
  while x >= 0x80 {
      buf.push((x | 0x80) as u8);
      x >>= 7;
  }
  buf.push(x as u8);
  return buf;
}



pub fn create_read_dataflow(manga_id: &str, manga_num: &str, mid: &str, now: i64) -> Vec<u8> {
  let int_array1 = [ 7, 101, 118, 101, 110, 116, 73, 100, 128,
  0, 0, 39, 98, 105, 108, 105, 98, 105, 108, 105, 45, 109, 97, 110, 103, 97, 46,
  109, 97, 110, 103, 97, 45, 114, 101, 97, 100, 46, 114, 101, 97, 100, 46, 118,
  46, 112, 108, 97, 121, 101, 114, 5, 108, 111, 103, 73, 100, 0, 0, 0, 6, 48,
  48, 49, 53, 51, 56, 10, 39, 98, 105, 108, 105, 98, 105, 108, 105, 45, 109, 97,
  110, 103, 97, 46, 109, 97, 110, 103, 97, 45, 114, 101, 97, 100, 46, 114, 101,
  97, 100, 46, 118, 46, 112, 108, 97, 121, 101, 114, 18, 177, 1, 8, 17, 16, 3,
  26, 37, 88, 89, 49, 48, 55, 53, 51, 65, 52, 53, 54, 55, 51, 67, 66, 68, 50,
  52, 53, 51, 50, 51, 68, 53, 52, 51, 48, 55, 53, 56, 68, 67, 56, 52, 69, 66,
  66, 34, 12, 112, 99, 95, 98, 105, 108, 105, 99, 111, 109, 105, 99, 42, 7, 115,
  97, 109, 115, 117, 110, 103, 50, 58, 71, 121, 115, 97, 76, 120, 112, 45, 82,
  51, 89, 81, 74, 120, 69, 108, 87, 87, 116, 90, 97, 49, 107, 95, 67, 68, 111,
  73, 98, 65, 65, 65, 65, 67, 66, 99, 79, 81, 65, 50, 65, 122, 99, 68, 77, 119,
  65, 50, 85, 68, 77, 67, 78, 70, 86, 105, 85, 119, 58, 8, 83, 77, 45, 71, 57,
  55, 55, 78, 66, 5, 55, 46, 49, 46, 50, 72, 159, 221, 239, 154, 6, 88, 185, 78,
  96, 25, 106, 11, 97, 114, 109, 101, 97, 98, 105, 45, 118, 55, 97, 122, 8, 54,
  97, 97, 97, 97, 97, 54, 49, 26, 48, 8, 1, 18, 5, 51, 49, 48, 49, 54, 42, 6,
  52, 46, 50, 50, 46, 48, 50, 8, 51, 54, 52, 50, 50, 48, 48, 48, 58, 12, 57, 46,
  48, 46, 52, 50, 45, 98, 101, 116, 97, 50, 74, 5, 50, 48, 48, 54, 50, 34];
  let int_array3 = [50, 6, 48, 48, 49, 53, 51, 56, 64, 134, 130, 1, 72, 9, 106];
  let int_array4 = [10, 8, 109, 97, 110, 103, 97, 95, 105, 100, 18];
  let int_array5 = [106, 12, 10, 7, 102,
  108, 117, 116, 116, 101, 114, 18, 1, 49, 106, 43, 10, 7, 114, 101, 97, 100,
  95, 105, 100, 18, 32, 49, 50, 55, 97, 50, 97, 98, 52, 50, 99, 53, 52, 56, 101,
  53, 57, 56, 56, 51, 57, 57, 102, 53, 55, 52, 53, 55, 48, 57, 57, 54, 56, 106,
  13, 10, 8, 102, 114, 101, 101, 102, 108, 111, 119, 18, 1, 48, 106,];
  let int_array6 = [10, 9, 109, 97, 110, 103, 97, 95, 110, 117, 109, 18];
 
  let manga_id_len = manga_id.len();
  let manga_num_len = manga_num.len();

  // 定义一个数组，用于存放最终的数据
  let mut data = Vec::new();
  // 将数组1的数据添加到 data 中
  data.extend_from_slice(&int_array1);
  // 将 mid len 添加到 data 中
  data.push(mid.len() as u8);
  // 将 mid 添加到 data 中
  data.extend_from_slice(mid.as_bytes());
  data.push(40);
  // 将 now to varint 添加到 data 中
  data.extend_from_slice(&to_varint(now - 2000));
  // 将数组3的数据添加到 data 中
  data.extend_from_slice(&int_array3);
  // manga_id_len 添加到 data 中
  data.push((manga_id_len + 12) as u8);
  // 将数组4的数据添加到 data 中
  data.extend_from_slice(&int_array4);
  // 将 manga_id value len 添加到 data 中
  data.push(manga_id_len as u8);
  // 将 manga_id value 添加到 data 中
  data.extend_from_slice(manga_id.as_bytes());
  // 将数组5的数据添加到 data 中
  data.extend_from_slice(&int_array5);
  // manga_num_len 添加到 data 中
  data.push((manga_num_len + 13) as u8);
  // 将数组6的数据添加到 data 中
  data.extend_from_slice(&int_array6);
  // 将 manga_num value len 添加到 data 中
  data.push(manga_num_len as u8);
  // 将 manga_num value 添加到 data 中
  data.extend_from_slice(manga_num.as_bytes());
  // 将数组7的数据添加到 data 中
  data.extend_from_slice(&[112, 1, 120]);
  // 将 sn_gen_time 添加到 data 中
  data.extend_from_slice(&to_varint(now - 1000));
  // 将数组8的数据添加到 data 中
  data.extend_from_slice(&[128, 1]);
  // 将 upload_time 添加到 data 中
  data.extend_from_slice(&to_varint(now));

  let data_len: i32 = get_length(data.len() as i32);
  let check_num = get_check_num(get_length(data_len));

  let mut result = Vec::new();
  result.extend_from_slice(&[82, 68, 73, 79]);
  result.extend_from_slice(&data_len.to_be_bytes());
  result.extend_from_slice(&check_num.to_be_bytes());
  result.extend_from_slice(&data);

  return result;
}


#[wasm_bindgen(module = "/defined-in-js.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn fetch(host: &str, path: &str, data: Vec<u8>, content_type: &str) -> Result<JsValue, JsValue>;
}


pub async fn read_manga(data: Vec<u8>) -> Result<JsValue, JsValue> {
  let resp_value = fetch("dataflow.biliapi.com", "/log/pbmobile/realtime?android", data, "application/octet-stream").await?;

  Ok(resp_value)
}
