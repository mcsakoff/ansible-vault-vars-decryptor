# ansible-vault-vars-decryptor

Quickly decrypt and print out ansible vault variables from within a file.

- Assumes you have `ansible-vault` installed (in your path/in an activated virtual environment)

## Keywords:

- "Allow decrypting of files with vaulted variables" (https://github.com/ansible/ansible/issues/26190)
- "Ansible-vault is not able to decrypt files with individual variable encryption" (https://github.com/ansible/ansible/issues/78124)
  - > "While decrypt could be used to decrypt a string, it would have to appear as the 'whole document' to work, which you can do with editor that let you pass a 'selection' as the input to external calls, which vim can do (I'm not sure the extension does, but could be modified to do so)." (https://github.com/ansible/ansible/issues/78124#issuecomment-1167348692)
    
    The above is a faff. Hence this tool makes quickly viewing the decrypted values easier for me.
- https://newbit.ch/ansible-vault-encrypted-variables/

## ⚠️ Requirements

**Important**: This tool requires `ansible-vault` to be installed and available in your PATH. Please ensure you have Ansible installed before using this tool.


## Download

See https://github.com/chrisjsimpson/ansible-vault-vars-decryptor/actions/workflows/ci.yml artifacts and download & extract the binary for your os.
TODO: Tie release from auto rc to ci.yml `build` job.

## Usage

1. Download the latest release for your platform from the [releases page](https://github.com/chrisjsimpson/ansible-vault-vars-decryptor/releases/latest).

2. Extract the binary:
```bash
tar -xzf ansible-decryptor-Linux-x86_64.tar.gz  # Linux
tar -xzf ansible-decryptor-macOS-x86_64.tar.gz  # macOS
tar -xzf ansible-decryptor-Windows-x86_64.tar.gz # Windows
```

3. Set your vault password (choose one method):
```bash
# Method 1: Use environment variable
export ANSIBLE_VAULT_PASSWORD=your_password

# Method 2: Use password file
echo "your_password" > ~/.vault_pass
# or
export ANSIBLE_VAULT_PASSWORD_FILE=path/to/your/vault_password_file
```

4. Run the decryptor:
```bash
./ansible-decryptor path/to/your/encrypted_vars.yml
```

## Installation

### Download Pre-built Binary

Pre-built binaries are available for multiple platforms in the [Releases](https://github.com/YOUR_USERNAME/ansible-decryptor/releases) section. Choose the appropriate version for your system:

- Linux (x86_64, aarch64, riscv64)
- macOS (x86_64)
- Windows (x86_64)
- FreeBSD (x86_64)

Each release includes:
- Compressed binary (`.tar.gz`)
- SHA256 checksums for verification

To install:

1. Download the appropriate binary for your system from the [latest release](https://github.com/YOUR_USERNAME/ansible-decryptor/releases/latest)
2. Verify the checksum (recommended):
   ```bash
   sha256sum -c ansible-decryptor-<platform>.sha256
   ```
3. Extract the binary:
   ```bash
   tar xzf ansible-decryptor-<platform>.tar.gz
   ```
4. Move the binary to a directory in your PATH:
   ```bash
   sudo mv ansible-decryptor /usr/local/bin/
   ```

### Building from Source

If you prefer to build from source, you'll need Rust installed. Then:

```bash
git clone https://github.com/YOUR_USERNAME/ansible-decryptor.git
cd ansible-decryptor
cargo build --release
```

The binary will be available at `target/release/ansible-decryptor`

## Development Requirements

To run the tests, you'll need:
1. Rust toolchain (cargo, rustc)
2. Python and ansible-vault:
   ```bash
   # Install ansible-vault (Ubuntu/Debian)
   sudo apt-get install ansible

   # Or with pip
   pip install ansible-core
   ```

## Testing

The project includes integration tests that verify the decryption functionality. To run the tests:

```bash
# Run all tests
cargo test

# Run with output (including println! statements)
cargo test -- --nocapture

# Run a specific test
cargo test test_decrypt_all_yml

# Run tests with logging
RUST_BACKTRACE=1 cargo test
```

The tests verify:
- Decryption of single and multiple vault variables
- Handling of invalid vault passwords
- Error cases (nonexistent files)
- Preservation of non-vault content

Test fixtures are located in `tests/fixtures/`:
- `all.yml`: Example YAML with encrypted vault variables
- `vault_pass.txt`: Test vault password file

# Disclaimer

This was written by a mix of LLM and manual tweaks.
