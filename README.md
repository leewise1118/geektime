## 2024.4.29
- [x] text verify失败，但是单元测试 `test_ed25519_sign_verify`可以成功，目前不知道问题出在哪，需要debug.

- [ ] 作业1：

  - [ ] 阅读 rust-cc/awesome-cryptography-rust
  - [ ] 了解 rust cryptography 生态

- [ ] 作业2：阅读 chacha20poly1305文档，了解其使用方法，并构建 CLI 对输入文本进行加密/解密。要求：

  - [ ] ``````shell
    rcli text encrypt -key "xxx" => 加密并输出 base64
    rcli text decrypt -key "xxx" => base64 -> binary -> 解密成原始文本
    ``````





# 目前已经完成的功能
1. Support CSV convert.
2. Support Generate safe password, use 'zxcvbn' test strength of password.
3. Support Base64 encode/decode, you can choose between standard and urlsafe.
4. Support text encryption using Blake3 and ed25519_dalek.
