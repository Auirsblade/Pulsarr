#!/bin/bash
set -e

# Generate Swift client from Pulsarr OpenAPI
openapi-generator generate \
  -i https://dev.pulsarr-music.com/openapi.json \
  -g swift5 \
  -o GeneratedAPI

echo "PulsarrAPI regenerated in $OUTPUT_DIR"
