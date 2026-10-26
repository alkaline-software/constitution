#!/bin/sh
# A new product from template/: copied to a folder of its own, with every
# name Starter has made the product's.
#
#   scripts/new-product.sh <folder> <Name> <bundle id> <owner/repo>
#   scripts/new-product.sh ../tabflip Tabflip com.alkalinesoftware.tabflip alkaline-software/tabflip
#
# <Name> is the app's name as a person reads it (one word, a capital
# first). The crates become <name>-core and <name>-app, the binary <Name>,
# the home folder ~/.<name>, the environment's variables <NAME>_*, the
# Velopack pack id AlkalineSoftware.<Name>. The vendored crates are copied
# as they are. The folder must not exist yet.
set -eu

[ $# -eq 4 ] || { sed -n '2,13p' "$0" | sed 's/^# \{0,1\}//' >&2; exit 2; }
dest=$1 Name=$2 bundle=$3 repo=$4
case $Name in [A-Z][A-Za-z0-9]*) ;; *) echo "new-product: the name is one word with a capital first: $Name" >&2; exit 2;; esac
[ -e "$dest" ] && { echo "new-product: $dest exists already" >&2; exit 1; }

name=$(printf %s "$Name" | tr '[:upper:]' '[:lower:]')
NAME=$(printf %s "$Name" | tr '[:lower:]' '[:upper:]')
here=$(cd "$(dirname "$0")/.." && pwd)

mkdir -p "$dest"
(cd "$here/template" && tar cf - --exclude ./target --exclude ./dist --exclude .DS_Store .) | (cd "$dest" && tar xf -)
cd "$dest"

# Names in paths first, deepest first, then in the text of every file but
# the vendored crates and pictures.
find . -depth -name '*starter*' ! -path './vendor/*' | while read -r p; do
    mv "$p" "$(dirname "$p")/$(basename "$p" | sed "s/starter/$name/g")"
done
find . -type f ! -path './vendor/*' ! -name '*.png' ! -name '*.ico' ! -name '*.icns' ! -name '*.ttf' ! -name '*.otf' | while read -r f; do
    grep -Iq . "$f" 2>/dev/null || continue
    LC_ALL=C sed -i.bak \
        -e "s#com\.alkalinesoftware\.starter#$bundle#g" \
        -e "s#alkaline-software/starter#$repo#g" \
        -e "s/AlkalineSoftware\.Starter/AlkalineSoftware.$Name/g" \
        -e "s/Starter/$Name/g" -e "s/starter/$name/g" -e "s/STARTER/$NAME/g" "$f"
    rm -f "$f.bak"
done

git init -q
echo "made $dest: $Name ($bundle, $repo)"
echo "next: cargo test -p $name-core -p $name-app, then draw the icon (scripts/icon/icon.html) and run scripts/make-icon.sh"
