#!/bin/bash
msg=$(cargo run -- --version)
cp PKGBUILD ../packages/fmnl/
cd ../packages/fmnl
makepkg --printsrcinfo > .SRCINFO
makepkg -g >> PKGBUILD
git add PKGBUILD .SRCINFO
git commit -m "$msg"
git push
