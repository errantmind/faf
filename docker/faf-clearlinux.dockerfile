FROM clearlinux:latest

WORKDIR /faf
#COPY ./Cargo.toml ./Cargo.toml
#COPY ./src ./src
#COPY ./merged.profdata ./merged.profdata
#RUN swupd autoupdate --disable
#RUN swupd bundle-add user-basic-dev
RUN swupd bundle-add git
RUN swupd bundle-add xz
RUN swupd bundle-add c-basic
#RUN swupd bundle-add which
#RUN swupd bundle-add python3-basic
RUN swupd bundle-add wget
#RUN swupd bundle-add os-core-dev
# RUN git clone git://github.com/ninja-build/ninja.git && cd ninja && git checkout release && cmake -Bbuild-cmake -H. \
# && cmake --build build-cmake -j $(nproc --all) && cd build-cmake && cp ninja /usr/bin
RUN cd /faf
RUN wget --quiet -O llvm.tar.xz https://github.com/llvm/llvm-project/releases/download/llvmorg-12.0.0-rc3/clang+llvm-12.0.0-rc3-x86_64-linux-gnu-ubuntu-16.04.tar.xz
RUN tar -xf llvm.tar.xz clang+llvm-12.0.0-rc3-x86_64-linux-gnu-ubuntu-/bin
RUN /bin/cp -f clang+llvm-12.0.0-rc3-x86_64-linux-gnu-ubuntu-/bin/* /usr/bin
RUN rm -rf clang+llvm-12.0.0-rc3-x86_64-linux-gnu-ubuntu-
RUN rm -rf llvm.tar.xz
RUN cd /faf
RUN wget -O rustup-init.sh https://sh.rustup.rs
RUN chmod +x rustup-init.sh
RUN ./rustup-init.sh -y --default-toolchain nightly
RUN source $HOME/.cargo/env
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./merged.profdata ./merged.profdata
COPY ./benches ./benches
RUN /bin/cp -f /.cargo/bin/* /usr/bin
RUN export CC=/usr/bin/clang-12 && alias clang="/usr/bin/clang-12" && alias clang-format="clang-format-12"
#RUN cargo build --release

# RUN git clone --single-branch --branch release/12.x https://github.com/llvm/llvm-project.git && cd llvm-project && git checkout release/12.x \
# && mkdir build && cd build && \
# cmake -G Ninja -DLLVM_ENABLE_ASSERTIONS=No -DLLVM_ENABLE_PROJECTS="clang;lld;clang-tools-extra" -DCMAKE_BUILD_TYPE=Release ../llvm \
# && cmake --build . -j $(nproc --all) --clean-first
