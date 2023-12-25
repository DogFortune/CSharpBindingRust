fn main() {
    // C#→Rust
    // Rustで書いたコードをC#から呼び出せるコードを生成
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("calc")
        .generate_csharp_file("../NativeMethods.cs")
        .unwrap();

    // C#→C
    // fibonacciのヘッダーファイルを解析してFFIバインディングを生成
    bindgen::Builder::default()
        .header("c/fibonacci.h")
        .generate()
        .unwrap()
        .write_to_file("src/fibo_bindgen.rs")
        .unwrap();

    // fibonacciのコードをccクレートでコンパイル
    cc::Build::new()
        .warnings(true)
        .file("c/fibonacci.c")
        .compile("fibo");

    //出力されたFFIバインディングからC#連携コード生成
    csbindgen::Builder::new()
        .input_bindgen_file("src/fibo_bindgen.rs")
        // rust_method_prefixを設定しないとcsbindgen_になる。
        .rust_method_prefix("fibo_")
        .rust_file_header("use super::fibo_bindgen::*;")
        .csharp_class_name("Libfibo")
        // ただしこっちは指定しないと何もつかない。
        .csharp_entry_point_prefix("fibo_")
        // ここはcargo.tomlのpackageと同じにしないといけない。
        .csharp_dll_name("calc")
        .generate_to_file("src/fibo_ffi.rs", "../fibo_bindgen.cs")
        .unwrap();
}
