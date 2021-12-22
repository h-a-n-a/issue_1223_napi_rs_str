# Issue

## Description

This issue is mainly about the unmatched behavior between `&str` and `String`.

From the perspective of `&str`, it should not be compiled for a single string type, since NAPI-RS is not supported(you may try to use `&str` on a function, it would not be compiled).
In this case, it compiled successfully but the `name` is not emitted as intended.



## Reproduction


```bash
$ pnpm install
$ pnpm build
$ pnpm test
```