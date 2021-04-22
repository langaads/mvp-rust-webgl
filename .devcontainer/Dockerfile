ARG VARIANT="3.13"
FROM alpine:${VARIANT}

LABEL MAINTAINER="DevHero"

ARG USERNAME="vscode"
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN apk add build-base
RUN apk add sudo
RUN apk add curl
RUN apk add git
RUN apk add npm

RUN adduser -D $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
    # RUST_VERSION=1.51.0

# # Install rust
RUN curl https://sh.rustup.rs --silent | sh -s -- -y --default-toolchain none

# # Install rust-wasm
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh --silent | sh

# # Install rust
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 

# ENV PATH=/root/.cargo/bin:$PATH

# # Install rust-wasm
# RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN chmod -R a+w $RUSTUP_HOME $CARGO_HOME;
USER $USERNAME
RUN rustup default nightly


# LABEL MAINTAINER="DevHero"

# ARG VARIANT="3.13"
# FROM alpine:${VARIANT}

# ARG USERNAME="vscode"
# ARG USER_UID=1000
# ARG USER_GID=$USER_UID

# # build-base is necessary to build wasm
# RUN apk add --no-cache npm curl build-base

# # Install rust
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 

# ENV PATH="/root/.cargo/bin:$PATH"

# # Install rust-wasm
# RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh