import re
import sys
import subprocess


# As scrappy python implementation of the same tool for finding and decrypting ansible vault vars
# and printing to stdout.
def decrypt_data(encrypted_data):
    try:
        process = subprocess.Popen(
            ["ansible-vault", "decrypt", "--output=-"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        stdout, stderr = process.communicate(input=encrypted_data)

        if process.returncode == 0:
            return stdout
        else:
            raise RuntimeError(f"Failed to decrypt data: {stderr}")
    except Exception as e:
        sys.stderr.write(f"Error during decryption: {str(e)}\n")
        sys.exit(1)


def main():
    # Ensure a filename is provided as an argument
    if len(sys.argv) != 2:
        sys.stderr.write(f"Usage: {sys.argv[0]} <vault_file.yml>\n")
        sys.exit(1)

    filename = sys.argv[1]

    # Read the file into a list of lines
    try:
        with open(filename, "r") as f:
            lines = f.readlines()
    except Exception as e:
        sys.stderr.write(f"Failed to read the file: {str(e)}\n")
        sys.exit(1)

    # Regex to match lines with '!vault |', capturing the indentation
    vault_re = re.compile(r"^(\s*)(-?\s*.*?:)?\s*!vault\s*\|")
    vault_re = re.compile(r"^(\s*)(-?\s*.*?:?)?\s*!vault\s*\|")

    output_lines = []
    i = 0

    while i < len(lines):
        line = lines[i]

        match = vault_re.match(line)
        if match:
            base_indent = len(match.group(1))
            encrypted_data = ""

            output_lines.append(line)  # Keep the '!vault |' line

            i += 1
            # Collect encrypted data lines
            while i < len(lines):
                next_line = lines[i]
                next_line_indent = len(next_line) - len(next_line.lstrip())

                if next_line.strip() == "":
                    output_lines.append(next_line)
                    i += 1
                    continue

                if next_line_indent > base_indent:
                    # Remove base indentation and collect encrypted data
                    data_line = next_line[base_indent:]
                    encrypted_data += data_line.lstrip() + "\n"
                    i += 1
                else:
                    breakpoint()
                    break

            # Decrypt the encrypted data
            decrypted_data = decrypt_data(encrypted_data)

            # Indent decrypted data
            decrypted_lines = decrypted_data.splitlines()
            encrypted_line_indent = " " * (
                base_indent + 2
            )  # Increase indent for decrypted lines

            output_lines.append(f"{encrypted_line_indent.strip()}|")

            for decrypted_line in decrypted_lines:
                indented_line = f"{encrypted_line_indent}{decrypted_line}"
                output_lines.append(indented_line + "\n")
        else:
            output_lines.append(line)
            i += 1

    # Output the result
    for line in output_lines:
        sys.stdout.write(line)


if __name__ == "__main__":
    main()
