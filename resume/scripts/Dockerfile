from archlinux:latest

LABEL org.opencontainers.image.source https://github.com/realaravinth/realaravinth/resume/

RUN pacman -Sy
RUN pacman --noconfirm -S texlive-core ed

ENV TEXMFDIST=/usr/share/texmf-dist
COPY ./config/ /usr/share/texmf-dist/scripts/texlive/
#RUN ed $TEXMFDIST/scripts/texlive/tlmgr.pl < ./changes.txt
#COPY ./changes.txt .
#
#RUN ed $TEXMFDIST/scripts/texlive/tlmgr.pl < ./changes.txt
ENV tlmgr="$TEXMFDIST/scripts/texlive/tlmgr.pl --usermode"

RUN $tlmgr init-usertree
RUN $tlmgr option repository http://mirrors.rit.edu/CTAN/systems/texlive/tlnet

RUN $tlmgr install enumitem
RUN $tlmgr install xifthen
RUN $tlmgr install texliveonfly
RUN $tlmgr install ifmtarg
RUN $tlmgr install luatex
RUN $tlmgr install fontawesome
RUN $tlmgr install sourcesanspro
RUN $tlmgr install tcolorbox
RUN $tlmgr install environ
RUN $tlmgr install trimspaces

RUN pacman --noconfirm -Sc

WORKDIR /src
CMD ["luatex"]
#ENTRYPOINT ["lualatex"]
