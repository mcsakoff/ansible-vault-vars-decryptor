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

Give `ansible-decryptor` a path to your ansible file with vars (e.g. `all.yml`) and
it will attempt to decrypt and print them to standard out.

`ansible-decryptor ./src/group_vars/all.yml`

> This tool assumes you have `~/.ansible.cfg` configured with contents pointing to your ansible vault password e.g.:
  ```
  [defaults]
  vault_password_file=~/ansible-vault-password
  ```


Example run:

> All `!vault` encrypted variables are printed in plain text, already plaintext variables are printed verbatim.

Given a vars file (e.g. `all.yml`) with some vault encrypted variables:

```
$ cat all.yml 
username_plain: alice
ipmi_password: !vault |
          $ANSIBLE_VAULT;1.1;AES256
          65653566363161306561663830356630363032336338346438346135653638633563366334313636
          3563306530376631373936643137353565376465326235640a383136323663663938346439653432
          30353330353462653365343933663933366234336234303365626263363236393862613338313439
          3162393165663433660a306633333731663766643561663364386136333165303236393836326434
          65613537346236363233633439343832636537316335626163396138356436666230303639623833
          34643332653665383231373462316463613036646466363434366466656437373866313739336538
          37356463633861353938636561313138383939323736636361363630323631373466353666663765
          34653461316535363434356363306564376163336463333936396566326238613765663965363066
          63383765666539363666633838643266373932386433383233386138666233363239623337323238
          36363961333533386362666235366438316237336361336336396564313036303233303462366632
          62363339636365633236386537613735383063383434653362303865373435623636386338663139
          65623263393064323537643634353938653461643637646462376539343461366465643161666233
          34346161313662383665616437343330666563313263323333333264663830646163326364643265
          65333736333139333133313865353235623862313233666639633365326538663762396433363439
          64376434336266336536386264373464656237613264633630373362393133373138343932386632
          62636433333331626236306337636566343538383761326266666634333630363630303638656338
          34313638356536363239623530643836373733653130333263336639623763663134356664623764
          30653265623564653165613061353337306261366433326130306466363837396463623638323534
          64643632663237643165333030343332383364656333386331343337633561616366626633656431
          65643230663132373462626266626361353762353539656261313066313135626339313861653165
          33366439383234623237633533353135363033613263383838316561313161663036376435316663
          33373866636232373662383432646562616130633363393461386164346634353630376131303331
          61336235303331353338626131363162363163353661346531646539306337356166396433636565
          30666266356365316430343331663663353461316232386239316434383539656661326261373063
          6531

another_secret:
  - !vault |
          $ANSIBLE_VAULT;1.1;AES256
          63663937333161623734333431666164663661343634613165663330323963306666663862316364
          6435333863343734346634343132383961353739356464620a366139373065323762386437346234
          30333261636238396535623936303935353866383863666438373431636531636364353836656661
          6636666533306666330a376337343365363436616564643239363333656236663733613832643535
          62643664633162643037666530376330633235323738303237323665343932346538626662616134
          6430393832303362333432333165363434396636313739653735

```

Pass your ansible vars file to `ansible-decryptor` and it will display the plain text of
those encrypted vars:

```
ansible-decryptor ./src/vpn/group_vars/all.yml
username_plain: alice
ipmi_password: !vault |

|
  calvin
another_secret:
  - !vault |
|
    9j546efj-563b-49a7-af9a-71d018d7312k
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


# Disclaimer

This was written by a mix of LLM and manual tweaks.
