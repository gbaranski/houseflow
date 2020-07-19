FROM node:12

# Create app directory
WORKDIR /server

# Install app dependencies
COPY package*.json ./

# Note that, rather than copying the entire working directory, we are only copying the package.json file.
# This allows us to take advantage of cached Docker layers
RUN npm install

# Bundle app source
COPY . .
<<<<<<< HEAD
||||||| 9d54780
RUN touch /server/src/firebaseConfig.json
=======
RUN touch ./src/firebaseConfig.json
>>>>>>> 7231723443664089c265e36c0b053d8f543eb4e8

<<<<<<< HEAD
||||||| 9d54780
FROM arm32v7/node:12-alpine
COPY --from=build /server .
=======
EXPOSE 8000
>>>>>>> 7231723443664089c265e36c0b053d8f543eb4e8
CMD [ "npm", "run", "start" ]
