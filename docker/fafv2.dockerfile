FROM clearlinux-faf:v1

WORKDIR /faf
RUN cd /faf
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./merged.profdata ./merged.profdata
COPY ./benches ./benches
RUN export CC=/usr/bin/clang-12 && alias clang="/usr/bin/clang-12" && alias clang-format="clang-format-12"
RUN RUSTFLAGS="-Ctarget-cpu=native -Clinker=clang-12 -Clink-arg=-fuse-ld=lld -Clink-arg=-flto -Clto=thin -Cembed-bitcode=yes -Copt-level=3 -Ccodegen-units=1 -Cprofile-use=/faf/merged.profdata" cargo build --release
