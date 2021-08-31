#!/bin/bash

readonly TEXMFDIST=/usr/share/texmf-dist

pacman -Sy
pacman --noconfirm -S texlive-core ed diff

ed $TEXMFDIST/scripts/texlive/tlmgr.pl < changes.txt
export tlmgr='$TEXMFDIST/scripts/texlive/tlmgr.pl --usermode'

$tlmgr init-usertree
$tlmgr option repository http://mirrors.rit.edu/CTAN/systems/texlive/tlnet
$tlmgr install enumitem
$tlmgr install resume.tex
$tlmgr install xifthen
$tlmgr install texliveonfly
$tlmgr install ifmtarg
$tlmgr install luatex
$tlmgr install fontawesome
$tlmgr install  sourcesanspro
$tlmgr install  tcolorbox
$tlmgr install  environ
$tlmgr install  trimspaces
