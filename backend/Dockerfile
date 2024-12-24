# See https://www.debian.org/releases/
FROM rust:1.83-bookworm AS builder

ARG APP_NAME=backend

RUN apt-get update -y


# Start by fetching and building the rust dependencies.
# Create a dummy project so that the `cargo build` step doesn't complain about:
# Caused by:
#   no targets specified in the manifest
#   either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present
WORKDIR /src
RUN cargo new ${APP_NAME}
COPY Cargo.toml Cargo.lock /src/${APP_NAME}
WORKDIR /src/${APP_NAME}
RUN cargo build


# And finally copy the source for the Rust program.
COPY ./src /src/${APP_NAME}/src/
# Run all the build modes so that we have them available in the debug container.
RUN cargo build && cargo build --release

##############
# Distroless #
##############
# https://github.com/GoogleContainerTools/distroless
FROM gcr.io/distroless/cc-debian12 AS prod

# Create the same user in distroless.
ARG NB_USER="jovyan"
ARG NB_UID="1000"
ARG NB_GID="100"

# Copy from the exact path used in builder.
WORKDIR /app
COPY --from=builder --chown=${NB_UID}:${NB_GID} /src/backend/target/release/backend /app/backend

# Switch to non-root user.
USER ${NB_UID}

CMD ["/app/backend"]