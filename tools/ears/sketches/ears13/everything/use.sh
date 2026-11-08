#!/bin/bash
# use.sh VARIANT: the sampler block into core/sketch with the parts VARIANT
# names (b lowBell, c colorTone, w padWow; "bcw" all three), its three parts
# beside it. Run from the repository's root. Put core/sketch back after
# (git checkout core/sketch && rm core/sketch/{lowbell,colortone,padwow}.rs).
set -e
v=$1; D=tools/ears/sketches/ears13
cp $D/lowBell/block.rs core/sketch/lowbell.rs
cp $D/colorTone/block.rs core/sketch/colortone.rs
cp $D/padWow/block.rs core/sketch/padwow.rs
on() { case "$v" in *$1*) echo true;; *) echo false;; esac; }
sed -e "s/^const BELL: bool = true;/const BELL: bool = $(on b);/" \
    -e "s/^const COLOR: bool = true;/const COLOR: bool = $(on c);/" \
    -e "s/^const WOW: bool = true;/const WOW: bool = $(on w);/" $D/everything/block.rs > core/sketch/block.rs
printf '# Ears 13 sampler: the chords channel.\nchannel chords\n' > core/sketch/where.txt
