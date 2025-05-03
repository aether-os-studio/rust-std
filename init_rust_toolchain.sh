AETHEROS_UNKNOWN_ELF_PATH=$(rustc --print sysroot)/lib/rustlib/x86_64-unknown-aether
mkdir -p ${AETHEROS_UNKNOWN_ELF_PATH}/lib
cp -r target-x86_64.json ${AETHEROS_UNKNOWN_ELF_PATH}/target.json

CARGO_PROFILE_RELEASE_DEBUG=0 \
    CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=true \
    RUSTC_BOOTSTRAP=1 \
    RUSTFLAGS="-Cforce-unwind-tables=yes -Cembed-bitcode=yes -Zforce-unstable-if-unmarked" \
    __CARGO_DEFAULT_LIB_METADATA="stablestd" \
    cargo build \
    --target x86_64-unknown-aether \
    -Zbinary-dep-depinfo \
    --manifest-path "library/test/Cargo.toml"

rm -rf ${AETHEROS_UNKNOWN_ELF_PATH}/lib/*
cp library/target/x86_64-unknown-aether/debug/deps/*.rlib ${AETHEROS_UNKNOWN_ELF_PATH}/lib
