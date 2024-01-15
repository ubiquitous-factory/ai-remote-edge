FROM registry.access.redhat.com/ubi8/ubi as rhel8builder

RUN yum install -y gcc openssl-devel libudev-devel && \
    rm -rf /var/cache/dnf && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH=/root/.cargo/bin:${PATH}

COPY . /app-build

WORKDIR "/app-build"

RUN cargo build --release


FROM registry.access.redhat.com/ubi8/ubi-minimal

RUN  microdnf update && microdnf install -y procps-ng

WORKDIR "/app"
COPY --from=rhel8builder /app-build/target/release/serial-reader ./
CMD ["./serial-reader"]
