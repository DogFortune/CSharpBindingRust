#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod fibo_bindgen;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod fibo_ffi;

#[no_mangle]
pub extern "C" fn calcpoint(points: usize) {
    let mut src: Vec<u8> = vec![0; points * 3];
    let mut elm = 0;

    for _i in 0..points {
        src[elm] = 1;
        src[elm + 1] = 2;
        src[elm + 2] = 3;
        elm += 3;
    }

    // srcの最初の10個の要素を表示して確認する
    for i in src.iter().take(9) {
        println!("{}", i);
    }
}
