<div align="center">
  <h1>test-group</h1>
  <p>
    <strong>Organize Rust tests into groups for filtering in nextest</strong>
  </p>
  <p>

[![Crates.io](https://img.shields.io/crates/v/test-group.svg?style=for-the-badge)](https://crates.io/crates/test-group)
[![CI](https://img.shields.io/github/actions/workflow/status/sergiimk/test-group/build.yaml?logo=githubactions&label=CI&logoColor=white&style=for-the-badge&branch=master)](https://github.com/sergiimk/test-group/actions)
[![Dependencies](https://deps.rs/repo/github/sergiimk/test-group/status.svg?&style=for-the-badge)](https://deps.rs/repo/github/sergiimk/test-group)

  </p>
</div>

Cargo's built-in test runner does not support grouping tests, which can be a problem when some of your tests are reasource-intensive and you don't want to run them all at once. [Nextest](https://github.com/nextest-rs/nextest) solves this problem by allowing you to define test groups using filter expressions in a config file and limit concurrently. 

This works, but you may find yourself:
- Having very complex filter expressions that often go out-of-sync with code
- Or having a special module hierarchy for your tests, separating tests into modules like `heavy`, `dockerized` etc. - this is not ideal because tests for the same component may now end up in different files, based on how "heavy" they are.

This tiny crate lets you group tests in code like so:

```rust
#[test_group::group(heavy)]
#[test]
async fn test_foo() {
  // ...
}

#[test_group::group(heavy, flaky)]
#[test]
fn test_bar() {
  // ...
}
```

This will result in a code like this:

```rust
mod g64dc {
  use super::*;
  mod heavy {
    use super::*;
    #[test]
    async fn test_foo() {
      // ...
    }
  }
}

mod g3d6f {
  use super::*;
  mod heavy {
    use super::*;
    mod flaky {
      use super::*;
      #[test]
      fn test_bar() {
        // ...
      }
    }
  }
}
```

So that these tests will appear in `nextest` as:
- `g3d6f::heavy::test_foo`
- `g64dc::heavy::flaky::test_bar`

Allowing you to filter tests by group in your `.config/nextest.toml` as such:

```toml
[[profile.default.overrides]]
filter = 'test(::heavy::)'
threads-required = 4

[[profile.default.overrides]]
filter = 'test(::flaky::)'
retries = 3
```

Or when running as:

```sh
cargo nextest run -E 'test(::flaky::)'
```

> Note: The extra modules like `g3d6f` are generated by the macro to avoid module name collisions, as unlike `impl` blocks you cannot have more than one `mod X {}` in the same file. The names are based on a SHA hash of the test content to be stable but unique-ish.
