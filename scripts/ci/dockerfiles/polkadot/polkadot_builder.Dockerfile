# This is the build stage for vine. Here we create the binary in a temporary image.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /vine
COPY . /vine

RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the vine binary."
FROM docker.io/library/ubuntu:20.04

LABEL description="Multistage Docker image for vine: a platform for web3" \
	io.parity.image.type="builder" \
	io.parity.image.authors="chevdor@gmail.com, devops-team@parity.io" \
	io.parity.image.vendor="Parity Technologies" \
	io.parity.image.description="vine: a platform for web3" \
	io.parity.image.source="https://github.com/paritytech/vine/blob/${VCS_REF}/scripts/ci/dockerfiles/vine/peer_builder.Dockerfile" \
	io.parity.image.documentation="https://github.com/paritytech/vine/"

COPY --from=builder /vine/target/release/vine /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /vine vine && \
	mkdir -p /data /vine/.local/share && \
	chown -R vine:vine /data && \
	ln -s /data /vine/.local/share/vine && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/vine --version

USER vine

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/vine"]
