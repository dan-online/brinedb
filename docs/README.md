@brine-db/brine / [Exports](modules.md)

![npm version](https://img.shields.io/npm/v/@brine-db/brine)

# BrineDB

[Diesel](https://diesel.rs/) wrapper for NodeJS, providing basic key/value storage in SQLite/Postgres/MySQL/MariaDB.

## Table of contents

- [BrineDB](#brinedb)
  - [Table of contents](#table-of-contents)
  - [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Quick Start](#quick-start)
  - [Typescript](#typescript)
  - [Development](#development)
    - [Prerequisites](#prerequisites)
    - [Building the entire package](#building-the-entire-package)
  - [Contributing](#contributing)
  - [Built With](#built-with)
  - [Versioning](#versioning)
  - [Authors](#authors)
  - [License](#license)

## Getting Started

## Installation

To install and set up the library, run:

```sh
$ yarn add @brine-db/brine
```

Or if you prefer npm:

```sh
$ npm i @brine-db/brine
```

## Quick Start

```js
const { Brine } = require('@brine-db/brine');

// SQLite
const brinedb = new Brine('sqlite://:memory:');
const brinedb = new Brine('sqlite://database.sqlite');

// Postgres
const brinedb = new Brine('postgres://user:pass@localhost:5432/dbname');

// MySQL/MariaDB
const brinedb = new Brine('mysql://user:pass@localhost:3306/dbname');

// Initialize the database (also runs migrations)
await brinedb.init();

// Set a value
await brinedb.set('key', { hello: 'world' });

// Get a value
const value = await brinedb.get('key');
```

## Typescript

This library is written in Typescript and includes type definitions. Here is an example that will be typed correctly:

```ts
import { Brine, BrineDatabases } from '@brine-db/brine';

type Value = { hello: string }

const brinedb = new Brine<Value>(BrineDatabases.sqlite.memory);

await brinedb.set('key', { hello: 'world' });

const decoded = brinedb.get('key');

typeof decoded.hello; // string
```

## Development

### Prerequisites

This project requires NodeJS (version 18 or later) and yarn.
[Node](http://nodejs.org) and [Yarn](https://yarnpkg.com/) are really easy to install.
To make sure you have them available on your machine,
try running the following command.

```sh
$ yarn -v && node -v && rustc --version # Example output
3.6.3
v20.11.1
rustc 1.78.0-nightly (4a0cc881d 2024-03-11)
```

### Building the entire package

_Requirement: Rust is installed on your machine._

```sh
$ yarn build
```

This task will create a distribution version of the project
inside your local `dist/` folder and output a binary in `native/`

## Contributing

1.  Fork it!
2.  Create your feature branch: `git checkout -b my-new-feature`
3.  Add your changes: `git add .`
4.  Commit your changes: `git commit -am 'Add some feature'`
5.  Push to the branch: `git push origin my-new-feature`
6.  Submit a pull request :sunglasses:

## Built With

- [Napi-RS](https://napi.rs/)
- [Diesel](https://diesel.rs/)
- VSCode
- TypeScript
- Rust

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/dan-online/brinedb/tags).

## Authors

- **DanCodes** - [@dan-online](https://github.com/dan-online) - <dan@dancodes.online>

## License

[MIT License](https://dancodes.mit-license.org/2024) © DanCodes
