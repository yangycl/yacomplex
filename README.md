# yacomplex

`yacomplex` 是一個完全由 Rust 純手寫實現的精密複數（Complex Number）數學庫。本專案採用了高度模組化的多層級架構，專為需要高精度、輕量化以及清晰架構的數學、物理計算與工程模擬場景設計。

## ✨ 特色
- **獨立無依賴**：100% 純手寫，完全不依賴任何第三方庫（`[dependencies]` 欄位保持純淨）。
- **進階模組架構**：採用兩層級的精密路徑 `crate::types::types::Complex`，完美實現檔案與目錄的解耦管理。
- **支援全面衍生特徵**：結構體原生內建 `PartialEq`、`Debug`、`Clone` 與 `Copy` 特徵，完美支援 `==` 比較運算，且數值會自發複製，徹底解決 Rust 所有權（Move）的頭痛問題。

## 📦 檔案結構
```text
src/
├── lib.rs              # 專案大門
├── functions/
│   └── functions.rs    # 核心數學運算呼叫點
└── types/
    ├── mod.rs          # types 模組入口
    └── types.rs        # 存放 Complex 結構體本體與衍生特徵
```

## 🚀 快速上手與呼叫示範

在你的專案中，你可以透過雙冒號（`::`）的關聯函數方式，輕鬆呼叫複數的四則運算：

```rust
use crate::types::types::Complex;

fn main() {
    // 1. 建立兩個複數：c1 = 3.0 + 4.0i, c2 = 1.0 + 2.0i
    let c1 = Complex::new(3.0, 4.0); 
    let c2 = Complex::new(1.0, 2.0); 

    // 2. 複數加法 (add) -> 4.0 + 6.0i
    let sum = Complex::add(c1, c2);
    println!("加法結果: {:?}", sum);

    // 3. 複數減法 (sub) -> 2.0 + 2.0i
    let diff = Complex::sub(c1, c2);
    println!("減法結果: {:?}", diff);

    // 4. 複數乘法 (mul) -> -5.0 + 10.0i
    let prod = Complex::mul(c1, c2);
    println!("乘法結果: {:?}", prod);

    // 5. 比較運算 (==) -> 支援 PartialEq
    if sum == Complex::new(4.0, 6.0) {
        println!("數學驗證完全正確！");
    }
}
```

## 🛠️ 開發與發布
本專案已完全通過 `cargo check` 的編譯大考。若要在本地測試打包或上傳至 crates.io，請遵循以下步驟：

```bash
# 測試專案編譯與引用路徑
cargo check

# 執行 crates.io 發布模擬考（測試 token 與 package 欄位設定）
cargo publish --dry-run

# 正式發布至全球開源社群
cargo publish --token <你的_API_TOKEN>
```
