
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
FROM rust:buster

WORKDIR /usr/src/app
COPY . ./
RUN cargo build --release


################################
# bring it all togther
################################
#FROM debian:buster

#WORKDIR /usr/src/app

COPY --from=oracleDrivers /usr/lib/oracle /usr/lib/oracle
COPY --from=oracleDrivers /usr/share/oracle /usr/share/oracle
COPY --from=oracleDrivers /etc/ld.so.conf.d/oracle-instantclient.conf /etc/ld.so.conf.d/oracle-instantclient.conf


RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get install -y ca-certificates \
   && apt-get install -y libaio1 \
   && apt-get autoremove -y && apt-get clean -y

RUN ldconfig


#COPY --from=rustbuilder /usr/src/app/target/release /usr/src/app

EXPOSE 1080
CMD ./target/release/apitest
