FROM node:alpine AS install

WORKDIR /app

COPY package-lock.json .
COPY package.json .

RUN npm i

FROM install AS compile

WORKDIR /app

COPY src src
RUN mkdir dist
RUN npx handlebars src/table.handlebars -f dist/table.handlebars.compiled.js

COPY --from=install /app/node_modules/handlebars/dist/handlebars.min.js dist
COPY --from=install /app/node_modules/bootstrap/dist/js/bootstrap.min.js dist
COPY --from=install /app/node_modules/bootstrap/dist/js/bootstrap.min.js.map dist
COPY --from=install /app/node_modules/bootstrap/dist/css/bootstrap.min.css dist
COPY --from=install /app/node_modules/bootstrap/dist/css/bootstrap.min.css.map dist
COPY src/index.html dist
COPY src/main.js dist
COPY src/style.css dist

FROM nginx:alpine

COPY --from=compile /app/dist /usr/share/nginx/html

CMD ["nginx", "-g", "daemon off;"]