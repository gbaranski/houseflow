FROM node:12-alpine

# Create app directory
WORKDIR /server

# Install app dependencies
COPY package*.json ./

# Note that, rather than copying the entire working directory, we are only copying the package.json file.
# This allows us to take advantage of cached Docker layers
RUN npm install

# Bundle app source
COPY . .

CMD [ "npm", "run", "start" ]
