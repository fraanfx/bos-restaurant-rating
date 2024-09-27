# Restaurant reviews Solana DAPP 

Descentralized application built with rust and running on Solana devnet 



Welcome to the **Restaurant review** project repository! This decentralized application (DApp) built with rust  leverages blockchain technology to implement a review platform on the Solana devnet network. Participants can place reviews about restaurants.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Smart Contracts](#smart-contracts)
- [Testing](#testing)
- [Frontend](#frontend)
- [Contributing](#contributing)
- [License](#license)

## Overview

The **Web3 Restaurant reviews DApp** provides a user-friendly interface to place restaurant reviews. This project ensures transparency and trust in the review process through the use of smart contracts. Users can place restaurant reviews on solana devnet.

## Features

- Place reviews for a restaurants.
- Real-time updates: Receive instant notifications for reviews.
- Decentralized: No central authority controls the data.
- Solana Wallet Integration: Connect your Solana wallet (e.g., Phantom, Solflare...) to participate directly.

## Getting Started

Follow these steps to set up the project locally and start participating in web3 auctions.

### Prerequisites

1. Node.js: Ensure Node.js is installed. Download it from [nodejs.org](https://nodejs.org/).

### Installation

1. Clone the repository:

```bash
  git clone https://github.com/fraanfx/bos-restaurant-rating.git
```

2. Navigate to the project frontend directory:

```bash
  cd frontend
```

3. Install required npm packages:

```bash
 npm install
```

## Usage

1. Start the development server:

```bash
 npm start
```

2. Open your web browser and navigate to `http://localhost:3000` to access the DApp.

3. Connect your Solana wallet (e.g., Phantom) to the DApp.


4. Browse ongoing reviews and place your own review.


## Smart Contracts

The smart contracts in this project facilitate the placement process. They handle review creation. These contracts are deployed on the Solana devnet blockchain.

- `lib.rs`: Responsible for creating new reviews.

Program deployed address: 8ewQYYxHYkpErrJApoY4vsaF8VvAdTt6gKwXyXvZMgiQ [Devnet explorer](https://explorer.solana.com/address/8ewQYYxHYkpErrJApoY4vsaF8VvAdTt6gKwXyXvZMgiQ?cluster=devnet)

## Testing

Smart contract tests are located in the `tests` folder inside the backend folder. These tests ensure the correct functioning of the smart contract. To run the tests, follow these steps:

1. Open a terminal in the project directory.

2. Navigate from  root to backend folder:

```bash
cd backend
```

3. Run the following command to execute the tests:

```bash
cargo test
```

This command will initiate the smart contract tests and display the results in the terminal.


### Setting up the environment

First,enter to the frontend folder and install required libraries:

```bash
npm install
```

Then, run the development server:

```bash
npm run dev
```


ProgramID: 8ewQYYxHYkpErrJApoY4vsaF8VvAdTt6gKwXyXvZMgiQ



## Frontend

The DApp frontend is built using modern web technologies including React.js. It provides an intuitive and interactive user interface for auction participation.

- **React.js**: Powers the DApp's user interface.
- **Web3.js**: The Solana JavaScript API for smart contract interaction.
- **Phantom**: A popular Solana wallet browser extension for secure transactions.


## Contributing

Contributions to this project are welcome! To contribute:

1. Fork the repository.
2. Create a new branch for your feature/bug fix.
3. Make changes and test thoroughly.
4. Commit with clear and concise messages.
5. Push changes to your fork.
6. Submit a pull request describing your changes.


## License

This project is licensed under the [MIT License](LICENSE).

---

Thank you for your interest in the Web3 Restaurant reviews DApp project! For questions or suggestions, reach out to us or open an issue on [GitHub](https://github.com/fraanfx/bos-restaurant-rating). Happy bidding on the blockchain! 🚀
