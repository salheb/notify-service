####################################################################################################
# Many thanks for @DriLLFreAK100 (github user) for his example of 
#   how to create a dockerfile for Rust Lang which works 
#   (https://github.com/DriLLFreAK100/codefee-works-api)
####################################################################################################
# distroless docker file
# Build: docker build -t token-service . 
#         && docker images
# Run: docker run -p 80:80 token-service // docker run -p <host-port>:<container-port> <image-name>
# Test: curl http://localhost/health
# If running under wsl, get the linux machine IP and replace localhost above

# Use rust-based image for container; rustc version 1.70.0
FROM rust:1.70 AS builder
LABEL Author="Julio Nogueira <julio.salheb@gmail.com>"

RUN update-ca-certificates

# Set working directory in container
RUN mkdir /usr/src/notify-service
WORKDIR /usr/src/notify-service

# Copy all source code file from local computer to container
COPY src src
COPY Cargo.toml .
COPY LICENSE .
COPY .env.docker .env

# install some dependencies needed at build time
RUN apt-get update && apt-get install libpq5 libsasl2-dev zlib1g -y 

# Build release application
RUN cargo install --path .
RUN strip -s /usr/src/notify-service/target/release/notify-service

###################
## Runtime image ##
###################
FROM gcr.io/distroless/cc-debian11 AS runtime

# Set the architecture argument (arm64, i.e. aarch64 as default)
# For x86_64, i.e. amd64, you can append a flag when invoking the build `... --build-arg "ARCH=aarch64"`
ARG ARCH=x86_64

# libpq related (required by diesel)
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libpq.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libgssapi_krb5.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libldap_r-2.4.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libkrb5.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libk5crypto.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libkrb5support.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/liblber-2.4.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libsasl2.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libgnutls.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libp11-kit.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libidn2.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libunistring.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libtasn1.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libnettle.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libhogweed.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libgmp.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libffi.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /lib/${ARCH}-linux-gnu/libcom_err.so* /lib/${ARCH}-linux-gnu/
COPY --from=builder /lib/${ARCH}-linux-gnu/libkeyutils.so* /lib/${ARCH}-linux-gnu/
COPY --from=builder /lib/${ARCH}-linux-gnu/libz.so.1* /lib/${ARCH}-linux-gnu/

# Copy app binary from builder image
COPY --from=builder /usr/local/cargo/bin/notify-service /usr/local/bin/notify-service
COPY .env.docker ./.env

# Expose listening port for application
EXPOSE 8080

# Run the application
CMD ["notify-service"]