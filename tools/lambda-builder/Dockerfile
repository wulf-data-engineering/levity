FROM public.ecr.aws/amazonlinux/amazonlinux:2023

RUN dnf install -y --nodocs --setopt=install_weak_deps=False \
    gcc \
    openssl-devel \
    protobuf-compiler \
    cmake \
    tar \
    gzip \
    file \
    python3-pip && \
    dnf clean all && \
    rm -rf /var/cache/dnf

RUN pip3 install ziglang==0.15.2

RUN CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    --no-modify-path --default-toolchain none && \
    mkdir -p /opt/rust-bin && \
    mv /root/.cargo/bin /opt/rust-bin/bin && \
    rm -rf /root/.cargo /root/.rustup

ENV RUSTUP_HOME=/var/cargo/rustup \
    CARGO_HOME=/var/cargo/registry \
    PATH="/opt/rust-bin/bin:${PATH}"

COPY collect-binaries.sh /usr/local/bin/collect-binaries
RUN chmod +x /usr/local/bin/collect-binaries

WORKDIR /app