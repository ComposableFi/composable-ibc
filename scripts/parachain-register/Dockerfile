FROM node:18

COPY * /build/
WORKDIR /build

RUN yarn build

ENTRYPOINT ["yarn", "start"]
