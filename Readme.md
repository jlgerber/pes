# PES The Package Environment System

## TODOs
- [ ] define a trait to convert from a string to a PubGrub::range
- [ ] implement said trait for Range<SemanticVersion>
- [ ] implement said trait for Range<Number>
- [ ] design a manifest for a pes package
- [ ] implement a reader for the manifest 
- [ ] define a package repository to store manifests 
- [ ] write a caching package version provider 


# Design

## package manifest
```yaml

schema: 1

name: mypackge

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