# `sadness-generator`

A native node module to generate various types of native crashes

It's just an [`napi-rs`](https://github.com/napi-rs/napi-rs) wrapper for [`sadness-generator`](https://github.com/EmbarkStudios/crash-handling/tree/main/sadness-generator).


```ts
const { raiseAbort, raiseSegfault, raiseIllegalInstruction } = require('sadness-generator');

// Crash!
raiseSegfault();
```

