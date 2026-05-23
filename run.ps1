cargo +nightly build `
  -p kernel `
  --target x86_64-unknown-none `
  "-Zbuild-std=core,compiler_builtins" `
  "-Zbuild-std-features=compiler-builtins-mem"

cargo +nightly run -p plum_runner