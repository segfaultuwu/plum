cargo +nightly build `
  -p kernel `
  --target x86_64-unknown-none `
  "-Zbuild-std=core,alloc,compiler_builtins" `
  "-Zbuild-std-features=compiler-builtins-mem"

if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

cargo +nightly run -p plum_runner