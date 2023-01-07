
################################
# oracle client
################################
FROM oraclelinux:8 as oracleDrivers

WORKDIR /tmp
RUN dnf -y install wget
RUN wget https://yum.oracle.com/repo/OracleLinux/OL7/oracle/instantclient21/x86_64/getPackage/oracle-instantclient-basiclite-21.4.0.0.0-1.x86_64.rpm
RUN dnf -y install oracle-instantclient-basiclite-21.4.0.0.0-1.x86_64.rpm


################################
# rust build
################################
FROM rust:buster as rustbuilder

WORKDIR /usr/src/app
COPY . ./
RUN cargo build --release


################################
# bring it all togther
################################
FROM debian:buster

WORKDIR /usr/src/app

COPY --from=oracleDrivers /usr/lib/oracle /usr/lib/oracle
COPY --from=oracleDrivers /usr/share/oracle /usr/share/oracle
COPY --from=oracleDrivers /etc/ld.so.conf.d/oracle-instantclient.conf /etc/ld.so.conf.d/oracle-instantclient.conf

RUN apt-get update -y && apt-get install -y ca-certificates
RUN ldconfig

RUN apt-get update && apt-get -y upgrade && apt-get -y dist-upgrade && apt-get install -y libaio1 && \
    apt-get -y autoremove && apt-get -y clean

COPY --from=rustbuilder /usr/src/app/target/release/*apitest* /usr/src/app

EXPOSE 1080
CMD ./apitest
