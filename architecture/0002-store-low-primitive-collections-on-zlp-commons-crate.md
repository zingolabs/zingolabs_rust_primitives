# 2. store low-primitive collections in zlp_common package

Date: 2025-09-02

## Status

Accepted

## Context

Interdependencies among various crates, especially in the zingolib and infrastructure workspaces, have complicated upgrades.   We need to support NU6.1 which is defined in zcash_protocol, but multiple versions of that crate are used in one or both of zingolib's crates and infrastructure's crates.

## Decision

I've decided to move types that are (a) used in both workspaces, and (b) are themselves independent of zingolabs types into a new workspace (this one), and package (this one).  This package does not depend on any other zingolabs package.

## Consequences

This package does not include a Cargo.lock, this allows consumers to contrain versions according to their requirements.

Consumers will have to add this package as a dependencies.

Consumers will remove dependencies formerly required for these abstractions.

That is:

As Rust abstractions with the properties listed in "Decision" are migrated into this package, other zingolabs packages can depend on them here, removing interdepdendencies between those packages, e.g. zingolib/zingolib and infrastructure/infrastructure.
