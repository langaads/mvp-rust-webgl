ARG VARIANT="3.13"
FROM alpine:${VARIANT}

# [Option] Install zsh
ARG INSTALL_ZSH="true"

# Install needed packages and setup non-root user. Use a separate RUN statement to add your own dependencies.
ARG USER=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN apk add --update sudo
RUN apk add --update git

RUN adduser -D $USER \
        && echo "$USER ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/$USER \
        && chmod 0440 /etc/sudoers.d/$USER

USER $USER