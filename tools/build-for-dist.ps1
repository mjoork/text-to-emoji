# This script should be run from the root of the project.

$OUT_DIR = ".\dist\win64\"

Clear-Host

# Make clean release build. Can afford! Small project :)
Write-Output "Building a fresh release build ..."
cargo clean
cargo build --release

# Make "dist" directory where the release will go.
# NOTE: Only for releasing and compilation from source, hence why it is in .gitignore together with "/target"
if ((Test-Path $OUT_DIR) -eq 0) {
    Write-Output "Directory '$OUT_DIR' doesn't exist; creating ..."
    New-Item -ItemType Directory $OUT_DIR | Out-Null
}
else {
    Write-Output "Directory '$OUT_DIR' exists; cleaning ..."
    Remove-Item -Recurse $OUT_DIR
    New-Item -ItemType Directory $OUT_DIR | Out-Null
}

# Self-explanatory
Write-Output "Copying release build to '$OUT_DIR' ..."
Copy-Item ./target/release/monke-text.exe ./dist/win64/monke-text-win-x64.exe

# We are done.
Write-Output "-------- Done. --------"