use super::*;
mod package_manifest {
    
    const P1: &str = r#"
    ---
    schema: 1
    name: mypackge
    version: 1.2.3
    description: this is the description
    
    targets:
        run:
            requires:
                maya-plugins: ^4.3
        build:
            include:
                - run
            requires:
                maya: 1.2.3+<4
    
    "#;
    use super::*;
    use PackageTarget;
    use pubgrub::version::SemanticVersion;
    use pubgrub::range::Range;

    #[test]
    fn can_deserialize_from_str() {
        let manifest = PackageManifest::from_str(P1);
        let mut run_target = PackageTarget::new();
        run_target.requires(
            "maya-plugins", 
            Range::between(
                SemanticVersion::new(4,3,0), 
                SemanticVersion::new(4,4,0)
            )
        );

        let mut build_target = PackageTarget::new();
        build_target.include("run");
        build_target.requires(
            "maya", 
            Range::between(
                SemanticVersion::new(1,2,3),
                SemanticVersion::new(4,0,0)
        ));
        let mut target_map = TargetMap::new();
        target_map.insert("run".into(), run_target);
        target_map.insert("build".into(), build_target);

        assert_eq!(manifest.unwrap(), 
            PackageManifest {
                schema: 1,
                name: "mypackage".into(),
                version: SemanticVersion::new(1,2,3),
                description: "this is the description".into(),
                targets: target_map,
            }
        );
    }
}