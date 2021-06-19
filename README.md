# ghti

GitHub Terminal Interface

## Proof-of-concept

This is a simple command-line tool that for now allows cloning an issue.

## Examples

For all of the metadata to be properly assigned to the new issue, a [personal access token][pat] is
required. The token can be set in the `GITHUB_TOKEN` environment variable.

```
# Configure the personal access token
 export GITHUB_TOKEN="my_personal_token"

# Clone issue #101 in repository jvff/ghti
cargo run -- -o jvff -r ghti 101
```

The repository and the owner can also be set using a `GITHUB_REPO` environment variable. The example
above is equivalent to:

```
# Configure the personal access token
 export GITHUB_TOKEN="my_personal_token"

# Clone issue #101 in repository jvff/ghti
export GITHUB_REPO="jvff/ghti"
cargo run 101
```

[pat]: https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token
