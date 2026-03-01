# syntax=docker/dockerfile:1

################################################################################
# Stage 1: Build the Vue application
FROM node:20-alpine AS build
WORKDIR /app

# Install dependencies first (better layer caching)
COPY package.json package-lock.json ./
RUN npm ci

# Copy source and build
COPY . .
RUN npm run build

################################################################################
# Stage 2: Serve with Nginx
FROM nginx:alpine AS final

# Copy custom nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copy built assets from build stage
COPY --from=build /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
