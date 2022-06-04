[![Issues][issues-shield]][issues-url]

<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/zrthxn/borshed">
    <img src="./docs/logo.png" alt="Logo" width="80">
  </a>

  <b><h2 align="center">BorshEd</h2></b>

  <p align="center">
    A command line editor for Borsh data.
Solana uses a data serialization system/algorithm called Borsh (Binary Object Representation Serializer for Hashing) which can be used to essentially write an in-program object to a "secure" binary format.
    <br />
    <a href="https://github.com/ashikka/golf-it"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/zrthxn/borshed/issues">Report Bug</a>
    ·
    <a href="https://github.com/zrthxn/borshed/issues">Request Feature</a>
  </p>
</p>



<!-- TABLE OF CONTENTS -->
## :dart: Table of Contents

* [About the Project](#about-the-project)
  * [Built With](#built-with)
* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
* [Usage](#usage)
* [Roadmap](#roadmap)
* [Contributing](#contributing)
* [License](#license)
* [Contributors](#contributors-)



<!-- ABOUT THE PROJECT -->
##  :open_book: About The Project

<p align="center">
  <a href="https://github.com/zrthxn/borshed">
    <img src="./docs/demo.gif" />
  </a>
</p>

## :gear: How we built it

## :rocket: What's next for Golf-it!

<!-- GETTING STARTED -->
## :airplane: Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.
* yarn
```sh
npm install -g yarn
```

### Installation
 
1. Clone the repo
```sh
git clone https://github.com/ashikka/golf-it.git
```
2. Install yarn packages
```sh
yarn
```
3. Add necessary environment variables to the project using: 
```sh
{
        echo 'API_KEY='
        echo 'AUTH_DOMAIN='
        echo 'PROJECT_ID='
        echo 'STORAGE_BUCKET='
        echo 'MESSAGING_SENDER_ID='
        echo 'APP_ID='

} >> .env
```
<!-- USAGE EXAMPLES -->
## :wrench: Usage

Start the project locally by following these steps. 

1. Run a Redis broker on your system

```sh
docker run -p6379:6379 redis
```

2. Start the worker which is going to interact with the library `code-executor`.
```sh
cd backend

ts-node worker.ts
```
3. Start the backend using:
```sh 
yarn run dev
```
4. Start the frontend using:
```sh
cd frontend

yarn start
```
## :triangular_flag_on_post: Routes
The backend of the project supports the following routes: 

### 1. Code submission
```http
POST /code/submission/:questionName
```

| Parameter | Type     | Description                     |
| :--------: | :-------: | :------------------------------: |
| `params`    | `string` | questionName |
| `body`    | `string` |  language |
| `body`    | `string` | code |
| `body`    | `Date` | submitTime |


### 2. Create new room
```http
POST /room/create
```
| Parameter | Type     | Description                     |
| :--------: | :-------: | :------------------------------: |
| `body`    | `string` | clientId|
| `body`    | `number` |  payload |

### 3. Get room
```http
GET /room/:roomId
```
| Parameter | Type     | Description                     |
| :--------: | :-------: | :------------------------------: |
| `params`    | `string` | roomId|

<!-- ROADMAP -->
## :world_map: Roadmap

See the [open issues](https://github.com/ashikka/golf-it/issues) for a list of proposed features (and known issues).



<!-- CONTRIBUTING -->
## :mechanical_arm: Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'feat: Add some AmazingFeature'`)
4. Push to the Branch (`git push -u origin feature/AmazingFeature`)
5. Open a Pull Request

You are requested to follow the contribution guidelines specified in [CONTRIBUTING.md](./CONTRIBUTING.md) while contributing to the project :smile:.

<!-- LICENSE -->
##  :page_facing_up: License

Distributed under the MIT License. See [`LICENSE`](./LICENSE) for more information.




<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[csivitu-shield]: https://img.shields.io/badge/csivitu-csivitu-blue
[csivitu-url]: https://csivit.com
[issues-shield]: https://img.shields.io/github/issues/csivitu/Template.svg?style=flat-square
[issues-url]: https://github.com/ashikka/golf-it/issuesa
