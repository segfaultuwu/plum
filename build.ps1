cargo +nightly build `
  -p kernel `
  --target x86_64-unknown-none `
  "-Zbuild-std=core,alloc,compiler_builtins" `
  "-Zbuild-std-features=compiler-builtins-mem"
