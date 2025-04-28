#!/bin/bash

# create the target directory if it doesnt exist
TARGET_DIR="taillard-benchmark"

mkdir -p "$TARGET_DIR"

urls=(
  "http://mistic.heig-vd.ch/taillard/problemes.dir/ordonnancement.dir/flowshop.dir/best_lb_up.txt"
  "http://mistic.heig-vd.ch/taillard/problemes.dir/ordonnancement.dir/flowshop.dir/tai20_5.txt"
  "http://mistic.heig-vd.ch/taillard/problemes.dir/ordonnancement.dir/flowshop.dir/tai20_10.txt"
  "http://mistic.heig-vd.ch/taillard/problemes.dir/ordonnancement.dir/flowshop.dir/tai20_20.txt"
  "http://mistic.heig-vd.ch/taillard/problemes.dir/ordonnancement.dir/flowshop.dir/tai50_5.txt"
  "http://mistic.heig-vd.ch/taillard/problemes.dir/ordonnancement.dir/flowshop.dir/tai50_10.txt"
  "http://mistic.heig-vd.ch/taillard/problemes.dir/ordonnancement.dir/flowshop.dir/tai50_20.txt"
  "http://mistic.heig-vd.ch/taillard/problemes.dir/ordonnancement.dir/flowshop.dir/tai100_5.txt"
  "http://mistic.heig-vd.ch/taillard/problemes.dir/ordonnancement.dir/flowshop.dir/tai100_10.txt"
  "http://mistic.heig-vd.ch/taillard/problemes.dir/ordonnancement.dir/flowshop.dir/tai100_20.txt"
  "http://mistic.heig-vd.ch/taillard/problemes.dir/ordonnancement.dir/flowshop.dir/tai200_10.txt"
  "http://mistic.heig-vd.ch/taillard/problemes.dir/ordonnancement.dir/flowshop.dir/tai200_20.txt"
  "http://mistic.heig-vd.ch/taillard/problemes.dir/ordonnancement.dir/flowshop.dir/tai500_20.txt"
)

for url in "${urls[@]}"; do
    filename=$(basename "$url")
    # wget -q --no-clobber --timestamping -P "$TARGET_DIR" "$url"
    wget --show-progress -q --no-clobber -O "$TARGET_DIR/$filename" "$url"
done

echo "All files downloaded into '$TARGET_DIR' directory."