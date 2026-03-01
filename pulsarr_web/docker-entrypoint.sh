#!/bin/sh
set -e

# Replace the build-time placeholder with the runtime VITE_API_URL env var
# in all JS files under the nginx html directory
if [ -n "$VITE_API_URL" ]; then
  find /usr/share/nginx/html/assets -name '*.js' -exec \
    sed -i "s|__VITE_API_URL_PLACEHOLDER__|${VITE_API_URL}|g" {} +
fi

exec "$@"
