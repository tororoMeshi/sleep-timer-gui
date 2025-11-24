#!/bin/bash
set -euxo pipefail

cd "$(dirname "$0")"

FILE=appimagetool-x86_64.AppImage
DIR=appimagetool.AppDir

if [ ! -f "$FILE" ]; then
    wget -q "https://github.com/AppImage/AppImageKit/releases/download/continuous/$FILE"
    chmod +x "$FILE"
fi

if [ ! -d "$DIR" ]; then
    rm -rf squashfs-root
    "./$FILE" --appimage-extract
    mv squashfs-root "$DIR"
fi

cat > appimagetool <<'EOF'
#!/bin/bash
exec "$(dirname "$0")/appimagetool.AppDir/AppRun" "$@"
EOF

chmod +x appimagetool
export PATH="$PWD:$PATH"

cargo appimage

rm -f appimagetool
