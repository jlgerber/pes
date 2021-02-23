# PES The Package Environment System

## TODOs
- [x] implement parser from str to  Range<SemanticVersion>
- [ ] implement parser from str to  Range<Number>
- [ ] design a manifest for a pes package
- [x] implement a reader for the manifest 
- [ ] define a package repository to store manifests 
- [ ] write a caching package version provider 


# Design

## package manifest
```yaml

schema: 1

name: mypackage

version: 1.2.3

description: >
    this is the description

targets:
    run:
        requires:
            maya-plugins: ^4.3
            gcc: 4.3.2
    build:
        include-target:
            - run
        requires:
            maya: 1.2.3+<4
environment:
    LD_LIBRARY_PATH: "{root}/foo/bar:@"

```

## toml version

```toml
schema = 1

name = "mypackage"

version = "1.2.3"

description  = """
The best table ever \
if you know what I mean"""

[target.run]

include-targets = [
    "run"
]

[target.run.requires]
maya-plugins = "^4.3"

[environment]
LD_LIBRARY_PATH = '${root}/foo/bar:@'

```

### Rust
```rust
struct PackageManifest {
    schema: u32,
    name: String,
    version: SemanticVersion,
    description: String,
    targets: HashMap<String, PackageTarget>
}

struct PackageRange {
    package: String,
    range: Range<SemanticVersion>
}

struct PackageTarget {
    name: String,
    include: Option<String>,
    requires: IndexMap<String, Range<SemanticVersion>>
}
```

## thoughts

How will the system work in practice?

There are a number of things that one needs to know when extending a manifest to act as a context:

1 - version - trivial as the manifest already has the info
2 - package name - trivial as the manifest already has the info
3 - package root - how do we record this? Do we brand the manifest at install time?
4 - base environment - what is the set of environment variables that are needed? How is this communicated? (trait?)