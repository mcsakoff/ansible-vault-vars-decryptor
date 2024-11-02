# ansible-vault-vars-decryptor

Quickly decrypt and print out ansible vault variables from within a file.

- Assumes you have `ansible-vault` installed (in your path/in an activated virtual environment)

## Keywords:

- "Allow decrypting of files with vaulted variables" (https://github.com/ansible/ansible/issues/26190)
- "Ansible-vault is not able to decrypt files with individual variable encryption" (https://github.com/ansible/ansible/issues/78124)
  - > "While decrypt could be used to decrypt a string, it would have to appear as the 'whole document' to work, which you can do with editor that let you pass a 'selection' as the input to external calls, which vim can do (I'm not sure the extension does, but could be modified to do so)." (https://github.com/ansible/ansible/issues/78124#issuecomment-1167348692)
    
    The above is a faff. Hence this tool makes quickly viewing the decrypted values easier for me.
- https://newbit.ch/ansible-vault-encrypted-variables/

## Download

See https://github.com/chrisjsimpson/ansible-vault-vars-decryptor/actions/workflows/ci.yml artifacts and download & extract the binary for your os.
TODO: Tie release from auto rc to ci.yml `build` job.

## Usage:

Give `ansible-vault-decryptor` a path to your ansible file with vars (e.g. `all.yml`) and
it will attempt to decrypt and print them to standard out.

`ansible-vault-decryptor ./src/group_vars/all.yml`

> This tool assumes you have `~/.ansible.cfg` configured with contents pointing to your ansible vault password e.g.:
  ```
  [defaults]
  vault_password_file=~/ansible-vault-password
  ```


Example run:

> All `!vault` encrypted variables are printed in plain text, already plaintext variables are printed verbatim.
```
ansible-vault-decryptor ./src/vpn/group_vars/all.yml
username_plain: alice
ipmi_password: !vault |

|
  calvin
another_secret:
  - !vault |
|
    9j546efj-563b-49a7-af9a-71d018d7312k
```


# Disclaimer

This was written by a mix of LLM and manual tweaks.
