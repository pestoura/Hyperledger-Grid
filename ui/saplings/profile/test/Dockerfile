# Copyright 2018-2021 Cargill Incorporated
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Dockerfile for running unit tests and lint on the profile sapling
FROM node:lts-alpine

WORKDIR /saplings/profile

COPY package*.json ./

RUN apk add --no-cache git

# Gives npm permission to run the prepare script in splinter-canopyjs as root
RUN npm config set unsafe-perm true && npm install

COPY . .
