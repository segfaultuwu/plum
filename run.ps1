./build.ps1

if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

cargo +nightly run -p plum_runner