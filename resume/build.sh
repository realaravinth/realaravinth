#!/bin/bash

docker run --volume="$(pwd):/src"  --rm -it realaravinth/resume:latest lualatex resume.tex
cp ./resume.pdf ../static/realaravinth-resume.pdf
