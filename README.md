# BARK - Blink Payments

Welcome to **BARK - Blink Payments**! This project is a web application for handling payments with BARK, SEND, SOL, USDC, and SPL tokens using Next.js. It includes functionalities for receiving and sending payments, as well as managing transaction interactions.

## Table of Contents

- [BARK - Blink Payments](#bark---blink-payments)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Installation](#installation)
  - [Development](#development)
  - [Usage](#usage)
    - [API Routes](#api-routes)
    - [Components](#components)
    - [Utilities](#utilities)
  - [Scripts](#scripts)
  - [Contributing](#contributing)
  - [License](#license)

## Introduction

**BARK - Blink Payments** is designed to facilitate payments using various tokens on the Solana blockchain, including BARK, SEND, SOL, USDC, and SPL tokens. This project showcases how to integrate multiple token payments into a web application, with a focus on efficient transaction handling and user interactions.

## Installation

To set up the project on your local machine, follow these steps:

1. **Clone the repository:**

   ```sh
   git clone https://github.com/barkprotocol/bark-blink-payments.git
   ```

2. **Navigate to the project directory:**

   ```sh
   cd bark-blink-payments
   ```

3. **Install dependencies:**

   ```sh
   npm install
   ```

   If using TypeScript, you may also need:

   ```sh
   npm install --save-dev typescript @types/react @types/node
   ```

## Development

To start the development server, use:

```sh
npm run dev
```

This command will start the server on [http://localhost:3000](http://localhost:3000), and you can view changes in real-time as you develop.

## Usage

### API Routes

The project includes API routes to handle payments:

- **`/api/receive-payment`**: Endpoint to receive BARK tokens.
- **`/api/send-token`**: Endpoint to send tokens.
- **`/api/transfer`**: Endpoint to handle transfers of BARK, SOL, USDC, and other SPL tokens.

You can test these endpoints using tools like Postman or directly through the provided web interface.

### Components

Components for the application are located in the `components/` directory:

- **`components/Header.tsx`**: The header component for navigating the application.
- **`components/Footer.tsx`**: The footer component displayed at the bottom of each page.

### Utilities

Utility functions are in the `utils/` directory:

- **`utils/utils.ts`**: General utility functions, including URL and fetch handling.
- **`utils/helpers.ts`**: Functions for specific tasks like interacting with the Solana blockchain.

## Scripts

Here are some common scripts you can use:

- `dev` - Start the development server.
- `build` - Build the application for production.
- `start` - Run the application in production mode.
- `lint` - Lint the codebase.

For example, to build the application for production, use:

```sh
npm run build
```

## Contributing

Contributions are welcome! To contribute:

1. **Fork the repository.**
2. **Create a new branch:**

   ```sh
   git checkout -b feature/your-feature-name
   ```

3. **Commit your changes:**

   ```sh
   git commit -m "Describe your changes"
   ```

4. **Push to the branch:**

   ```sh
   git push origin feature/your-feature-name
   ```

5. **Create a Pull Request** on GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
