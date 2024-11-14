#!/usr/bin/env bash

set -xeo pipefail

readonly live_run=${LIVE_RUN:-false}
# Release number
version=${VERSION:?input VERSION is required}
# Dependencies' pattern
readonly bump_deps_pattern=${BUMP_DEPS_PATTERN:-''}
# Dependencies' version
readonly bump_deps_version=${BUMP_DEPS_VERSION:-''}
# Dependencies' git branch
readonly bump_deps_branch=${BUMP_DEPS_BRANCH:-''}
# Git actor name
readonly git_user_name=${GIT_USER_NAME:?input GIT_USER_NAME is required}
# Git actor email
readonly git_user_email=${GIT_USER_EMAIL:?input GIT_USER_EMAIL is required}

cargo +stable install toml-cli

# NOTE(fuzzypixelz): toml-cli doesn't yet support in-place modification
# See: https://github.com/gnprice/toml-cli?tab=readme-ov-file#writing-ish-toml-set
function toml_set_in_place() {
  local tmp=$(mktemp)
  toml set "$1" "$2" "$3" > "$tmp"
  mv "$tmp" "$1"
}

export GIT_AUTHOR_NAME=$git_user_name
export GIT_AUTHOR_EMAIL=$git_user_email
export GIT_COMMITTER_NAME=$git_user_name
export GIT_COMMITTER_EMAIL=$git_user_email

# For development releases (e.g. 1.0.0-dev-21-g2ca8632), transform the version
# into a PEP-440 compatible version, since maturin>1 requires strict version compliance.
if [[ "${version}" =~ [0-9]+\.[0-9]+\.[0-9]+-dev-[0-9]+-g[a-f0-9]+ ]]; then
  version=$(echo $version | sed 's/dev-/dev+/')
fi

# Bump Cargo version
toml_set_in_place Cargo.toml "package.version" "$version"
# Propagate version change to pyproject.toml
toml_set_in_place pyproject.toml "project.version" "$version"

git commit Cargo.toml pyproject.toml -m "chore: Bump version to $version"

# Select all package dependencies that match $bump_deps_pattern and bump them to $bump_deps_version
if [[ "$bump_deps_pattern" != '' ]]; then
  deps=$(toml get Cargo.toml dependencies | jq -r "keys[] | select(test(\"$bump_deps_pattern\"))")
  for dep in $deps; do
    if [[ -n $bump_deps_version ]]; then
      toml_set_in_place Cargo.toml "dependencies.$dep.version" "$bump_deps_version"
    fi

    if [[ -n $bump_deps_branch ]]; then
      toml_set_in_place Cargo.toml "dependencies.$dep.branch" "$bump_deps_branch"
    fi
  done
  # Update lockfile
  cargo check

  if [[ -n $bump_deps_version || -n $bump_deps_branch ]]; then
    git commit Cargo.toml Cargo.lock -m "chore: Bump $bump_deps_pattern version to $bump_deps_version"
  else
    echo "warn: no changes have been made to any dependencies matching $bump_deps_pattern"
  fi
fi

if [[ ${live_run} ]]; then
  git tag --force "$version" -m "v$version"
fi
git log -10
git show-ref --tags
git push origin
git push --force origin "$version"
