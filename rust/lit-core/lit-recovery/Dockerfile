FROM ubuntu:22.04

VOLUME [ "/data" ]

RUN rm -rf /tmp/lit_recovery
RUN mkdir -p /tmp/lit_recovery
COPY Cargo.* /tmp/lit_recovery
COPY src/ /tmp/lit_recovery/src/

COPY init.sh /tmp/lit_recovery
RUN chmod +x /tmp/lit_recovery/init.sh

RUN mkdir /root/.lit-recovery
RUN chmod 777 /root/.lit-recovery

COPY config.sample /root/.lit-recovery/config.json

WORKDIR /tmp/lit_recovery

RUN apt update
RUN apt upgrade -y
RUN apt install -y build-essential autoconf libtool curl git libssl-dev libudev-dev pkg-config ltrace strace libperl-dev libgtk-3-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev libwebkit2gtk-4.1-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
ENV PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
ENV LD_LIBRARY_PATH=/usr/local/lib/

RUN cargo build

CMD ["/tmp/lit_recovery/init.sh"]