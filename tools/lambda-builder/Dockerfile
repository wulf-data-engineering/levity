FROM public.ecr.aws/amazonlinux/amazonlinux:2023

# Configure dnf to be more robust against network flakes
RUN echo -e "[main]\n\
gpgcheck=1\n\
installonly_limit=3\n\
clean_requirements_on_remove=True\n\
best=True\n\
skip_if_unavailable=False\n\
retries=10\n\
timeout=30\n\
max_parallel_downloads=5" > /etc/dnf/dnf.conf

# Install system dependencies + Clang/LLD for cross-compilation
RUN dnf install -y --nodocs --setopt=install_weak_deps=False \
    gcc \
    clang \
    lld \
    openssl-devel \
    protobuf-compiler \
    cmake \
    tar \
    gzip \
    file && \
    dnf clean all && \
    rm -rf /var/cache/dnf

# Set internal "immutable" paths for the binaries
ENV PATH="/opt/rust-bin/bin:${PATH}"

# Install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    --no-modify-path \
    --default-toolchain none

# Move the binaries to our persistent location
RUN mkdir -p /opt && \
    mv /root/.cargo /opt/rust-bin && \
    rm -rf /root/.rustup

# Set the default RUSTUP_HOME/CARGO_HOME/CARGO_TARGET_DIR to the mount points
# These are where the toolchains and crates will be stored/cached
ENV RUSTUP_HOME=/var/cargo/rustup \
    CARGO_TARGET_DIR=/var/cargo/target \
    CARGO_HOME=/var/cargo/registry    

# Add a helper script to collect the binaries
COPY collect-binaries.sh /usr/local/bin/collect-binaries
RUN chmod +x /usr/local/bin/collect-binaries

# Add a helper script to act as the linker
RUN echo '#!/bin/bash\nclang -fuse-ld=lld --target=aarch64-unknown-linux-gnu "$@"' > /usr/local/bin/aarch64-linker && \
    chmod +x /usr/local/bin/aarch64-linker

WORKDIR /app