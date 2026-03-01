# syntax=docker/dockerfile:1

################################################################################
# Stage 1: Build the Vue application
FROM node:20-alpine AS build
WORKDIR /app

# Install dependencies first (better layer caching)
COPY package.json package-lock.json ./
RUN npm ci

# Copy source and build with placeholder for runtime substitution
COPY . .
ENV VITE_API_URL=__VITE_API_URL_PLACEHOLDER__
ARG VITE_APP_TAG=local
ENV VITE_APP_TAG=$VITE_APP_TAG
RUN npm run build

################################################################################
# Stage 2: Serve with Nginx
FROM nginx:alpine AS final

# Copy custom nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copy built assets from build stage
COPY --from=build /app/dist /usr/share/nginx/html

# Copy entrypoint that replaces env var placeholder at startup
COPY docker-entrypoint.sh /docker-entrypoint.sh

EXPOSE 80

ENTRYPOINT ["/docker-entrypoint.sh"]
CMD ["nginx", "-g", "daemon off;"]
