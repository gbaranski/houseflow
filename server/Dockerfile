FROM node:12

WORKDIR /app

# Install app dependencies
COPY package*.json ./

# Note that, rather than copying the entire working directory, we are only copying the package.json file.
# This allows us to take advantage of cached Docker layers
RUN npm install

# Bundle app source
COPY . .

EXPOSE 8000
EXPOSE 8001
EXPOSE 8003

CMD [ "npm", "run", "start" ]
