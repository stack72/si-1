# A list of available rules and their signatures can be found here: https://buck2.build/docs/api/rules/


# buildifier: disable=no-effect
rust_binary(
    name = "council",
    srcs = glob(
        ["bin/council/src/**/*.rs"],
    ),
    crate_root = "bin/council/src/main.rs",
    deps = [":council-server"],
) if not host_info().os.is_windows else None
