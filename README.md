# jwtool

Super basic CLI tool for decoding (but not verifying) JWTs.  This will print a single JSON blob
having the form `{"header":<header>,"payload":<payload>,"signature":<signature>}`, which can
be formatted using `jq .`.

## Installation

From the project root, run:

    cargo install --path .

## Example Usage

For nice, formatted output, pipe the output through `jq .` as in the following.

    echo "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c" | jwtool | jq .
