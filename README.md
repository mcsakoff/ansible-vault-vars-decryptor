# ansible-vault-vars-decryptor

Quickly decrypt and print out ansible vault variables from within a file.

## Features

- Native Rust implementation - no external dependencies required
- Fast and efficient decryption
- Supports both environment variables and password files for vault passwords
- Handles inline vault variables within YAML files

## Keywords:

- "Allow decrypting of files with vaulted variables" (https://github.com/ansible/ansible/issues/26190)
- "Ansible-vault is not able to decrypt files with individual variable encryption" (https://github.com/ansible/ansible/issues/78124)
  - > "While decrypt could be used to decrypt a string, it would have to appear as the 'whole document' to work, which you can do with editor that let you pass a 'selection' as the input to external calls, which vim can do (I'm not sure the extension does, but could be modified to do so)." (https://github.com/ansible/ansible/issues/78124#issuecomment-1167348692)
    
    The above is a faff. Hence this tool makes quickly viewing the decrypted values easier for me.
- https://newbit.ch/ansible-vault-encrypted-variables/

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

## Example Usage

Consider a YAML file with encrypted values (`secrets.yml`):

```yaml
username: bob
password: !vault |
          $ANSIBLE_VAULT;1.1;AES256
          66666561343034643465376137363938656161346430353236643834313131653630663331626531
          3330623833393338663930323633303566366134643131360a623862653133323732383166656366
          31333631663861386136346432373438653735346436633566333562656630613135303262383164
          3631616264633838350a616562376332356565316234643563383935393864353633393338646234
          3532
nested_secret:
  api_key: !vault |
          $ANSIBLE_VAULT;1.1;AES256
          34623737646430613735633664323934306138616266646532396434303661666166366237353862
          6331613966396639373939376633393033386239633138650a336338656235613363386364353564
          30636138393666306663383761313738653463326264646634353136663065666337623838306666
          6362333237616537630a613330613336653338313131633134623661373939663263303265656437
          3636
```

Running the decryptor:
```bash
export ANSIBLE_VAULT_PASSWORD=your_password
./ansible-decryptor secrets.yml
```

Output:
```yaml
username: bob
password: supersecret123
nested_secret:
  api_key: abcdef123456
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
