#!/bin/sh
set -e

FOLDER=${OUTDIR:-"out"}
FORMAT=${FMT:-"png"}
MGK=${MAGICK:-"$HOME/branches/ImageMagick/utilities/magick"}
TILESIZE="256"
ZOOMLEVEL=${ZOOMLEVEL:-"5"}

mkdir -p ${FOLDER}
${MGK} -background none "$1" \
  -crop ${TILESIZE}x${TILESIZE} \
  -set filename:f "%[fx:page.x/${TILESIZE}]-%[fx:page.y/${TILESIZE}]" \
  +repage +adjoin "${FOLDER}/${ZOOMLEVEL}-%[filename:f].${FORMAT}"
