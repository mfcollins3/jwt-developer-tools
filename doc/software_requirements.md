# Software Requirements

## Introduction

JWT Developer Tools supports two ways of working with the source code:

1. [Development Container](#using-the-development-container)
1. [Local Development](#local-development)

The recommended and preferred way to develop and build JWT Developer Tools is to use the [development container](https://containers.dev). The development container is pre-configured with all of the tools and libraries necessary to build, run, and develop JWT Developer Tools.

## Using the Development Container

To work with the development container, you will need to install the following software packages:

1. [Docker Desktop](#docker-desktop)
1. [Visual Studio Code](#visual-studio-code)
1. [Remote Development extension](#remote-development-extension)
1. __OPTIONAL__: [RustRover](#rust-rover)

### Docker Desktop

[Docker Desktop](https://www.docker.com) provides containerization services for developers on their workstations. Docker Desktop installs the Docker command line tools and runtime components that allow developers to build and run containers locally on their machines. The development container for JWT Developer Tools will be built and run using Docker Desktop.

To install Docker Desktop, download and install it from the [Docker website](https://www.docker.com).

### Visual Studio Code

[Visual Studio Code](https://code.visualstudio.com) is a free text editor and lightweight development environment for Linux, Apple macOS, and Microsoft Windows. Visual Studio Code provides a large ecosystem of plugins that add support for developing and debugging programs and libraries written in many different programming languages. Along with the [Remote Development extension](#remote-development-extension), Visual Studio Code can be used to run the development container and work with the source code inside of the development container.

Visual Studio Code can be installed from the [website](https://code.visualstudio.com).

### Remote Development Extension

The [Remote Development extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack) for [Visual Studio Code](#visual-studio-code) provides the extensions and tools necessary to build, run, and work with development containers.

The Remote Development extension can be installed from the [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack).

### RustRover

[JetBrains RustRover](https://jetbrains.com/rust) is a professional integrated development environment for building, developing, and debugging rust programs and libraries. The use of RustRover is optional, but the development container can be run using RustRover instead of Visual Studio Code.

## Local Development

To develop JWT Developer Tools locally on your machine, you will need to install the [Rust](https://www.rust-lang.org) development tools. Rust is an open source programming language that has become very popular and is supported by tools such as Visual Studio Code. To install the latest version of Rust, follow the [installation instructions](https://www.rust-lang.org/tools/install).
