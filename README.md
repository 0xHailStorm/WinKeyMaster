# WinKeyMaster
This utility, built in Rust, automates the activation process for Windows 10 and 11. It auto-detects your OS version and applies the appropriate activation key.


# Windows Activation Utility in Rust

## Overview

This project is a Rust-based utility designed to automate the activation of Windows operating systems, specifically Windows 10 and Windows 11. The utility helps you identify the current platform and automatically finds and applies the appropriate activation key.

## Features

- Supports Windows 10 and Windows 11
- Checks current activation status
- Automatically identifies the Windows version and edition
- Applies the most appropriate product key
- Utilizes native WinAPI calls

## Prerequisites

- Rust Programming Language
- Windows 10/11 operating system
- Administrator privileges

## Installation

1. Clone the repository
   ```
   git clone https://github.com/0xHailStorm/WinKeyMaster.git
   ```
2. Change into the directory
   ```
   cd WinKeyMaster
   ```

## Usage

1. Open a terminal and navigate to the project folder.

2. Compile the project:
   ```
   cargo build --release
   ```

3. Run the compiled binary as administrator:
   ```
   .\target\release\WinKeyMaster.exe
   ```

> **Note**: Make sure to run the program as an administrator to successfully activate Windows.
