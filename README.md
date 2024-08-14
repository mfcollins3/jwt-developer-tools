# JWT Developer Tools

## About JWT Developer Tools

JWT Developer Tools are a suite of tools that help developers to build distributed applications that use [JSON Web Tokens](https://jwt.io/introduction) to authorize client application requests sent to APIs on behalf of users. JSON Web Tokens are cryptographically signed containers containing values known as claims that provide information about the client application and user that are sending a request to APIs. APIs use JSON Web Tokens to authorize that the client application and user have permission to perform a specific action using the API. JWT Developer Tools helps developers while building applications. Using JWT Developer Tools, developers can generate JSON Web Tokens for testing, generate cryptographic keys used to sign and verify JSON Web Tokens, verify the signature and contents of JSON Web Tokens, or query the contents of a JSON Web Token. JWT Developer Tools can help developers build their API infrastructure before other authentication and authorization infrastructure is ready, or to help in the configuration of authorization services by supporting the creation of cryptographic signing keys.

## Contributing to JWT Developer Tools

Before beginning to work with the JWT Developer Tools, please review the [Software Requirements](doc/software_requirements.md) document. JWT Developer Tools supports two ways of development: using a development container or developing locally on your machine. The preferred and recommended way to work with the source code is to use the development container. When using a development container, the Git repository can either be stored locally or cloned from GitHub.

### Local Source Code, Work in Development Container using Visual Studio Code

To keep the source code on your local machine but work in a development container, first you will need to clone the repository to a location in your machine's file system:

    git clone https://github.com/mfcollins3/jwt-developer-tools.git

You can also use the [GitHub CLI](https://cli.github.com):

    gh repo clone mfcollins3/jwt-developer-tools

After cloning the repository, open the `jwt-developer-tools` directory in Visual Studio Code. Visual Studio Code should recognize the development container configuration and will prompt you to re-open the repository in a development container.

### Open the GitHub Repository in a Development Container using Visual Studio Code

Visual Studio Code can also build the development container directly from the GitHub repository:

1. Open Visual Studio Code
1. Press CMD+SHIFT+P on macOS or CTRL+SHIFT+P on Microsoft Windows or Linux.
1. Search for the `Dev Container: Clone Repository in Container Volume...` command.
1. Select the `Clone a repository from GitHub in a Container Volume` option in the dropdown list.
1. Enter `mfcollins3/jwt-developer-tools` as the repository name.

Visual Studio Code will clone the repository and open it in a development container.
