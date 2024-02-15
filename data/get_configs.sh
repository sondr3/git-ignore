#!/usr/bin/env bash

rm -f config-schema.json
wget https://raw.githubusercontent.com/starship/starship/master/.github/config-schema.json
jq '.properties | map_values({ detect_files: (.default.detect_files? // []), detect_extensions: (.default.detect_extensions? // []), detect_folders: (.default.detect_folders? // []) })' config-schema.json > parsed.json