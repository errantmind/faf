FROM debian:bullseye-slim

WORKDIR /faf
RUN cd /faf

# Set bash as default, instead of just /bin/sh
RUN /bin/sh -c set -xe && echo '#!/bin/sh' > /usr/sbin/policy-rc.d && echo 'exit 101' >> /usr/sbin/policy-rc.d && \
   chmod +x /usr/sbin/policy-rc.d && dpkg-divert --local --rename --add /sbin/initctl && cp -a /usr/sbin/policy-rc.d /sbin/initctl && \
   sed -i 's/^exit.*/exit 0/' /sbin/initctl

# Speed up apt
RUN echo 'force-unsafe-io' > /etc/dpkg/dpkg.cfg.d/docker-apt-speedup

# Add various apt configs
RUN echo 'DPkg::Post-Invoke { "rm -f /var/cache/apt/archives/*.deb /var/cache/apt/archives/partial/*.deb /var/cache/apt/*.bin || true"; };' > /etc/apt/apt.conf.d/docker-clean
RUN echo 'APT::Update::Post-Invoke { "rm -f /var/cache/apt/archives/*.deb /var/cache/apt/archives/partial/*.deb /var/cache/apt/*.bin || true"; };' >> /etc/apt/apt.conf.d/docker-clean
RUN echo 'Dir::Cache::pkgcache ""; Dir::Cache::srcpkgcache "";' >> /etc/apt/apt.conf.d/docker-clean
RUN echo 'Acquire::Languages "none";' > /etc/apt/apt.conf.d/docker-no-languages
RUN echo 'Acquire::GzipIndexes "true"; Acquire::CompressionTypes::Order:: "gz";' > /etc/apt/apt.conf.d/docker-gzip-indexes
RUN echo 'Apt::AutoRemove::SuggestsImportant "false";' > /etc/apt/apt.conf.d/docker-autoremove-suggests

# Get updates and basics needed for later installations
RUN apt update && apt upgrade -y
RUN apt install wget gnupg -y

# Install LLVM 12
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
RUN echo 'deb http://apt.llvm.org/bullseye/ llvm-toolchain-bullseye-12 main' >> /etc/apt/sources.list
RUN apt update
RUN apt install wget gnupg software-properties-common lsb-release nano -y
RUN wget https://apt.llvm.org/llvm.sh
RUN chmod +x llvm.sh
RUN ./llvm.sh 12
RUN apt install clang-tools-12

# Install Rust
RUN wget -O rustup-init.sh https://sh.rustup.rs
RUN chmod +x rustup-init.sh
RUN ./rustup-init.sh -y --default-toolchain nightly
RUN /root/.cargo/bin/rustup component add rust-src --toolchain nightly

# Set clang as default for `cc`
RUN update-alternatives --install /usr/bin/cc cc /usr/bin/clang-12 100

# Clean out apt to prevent unattended upgrades in the future.. all apt should be before this point
RUN rm -rf /var/cache/apt/archives/* && rm -rf /var/lib/apt/lists/*
CMD ["bash"]

