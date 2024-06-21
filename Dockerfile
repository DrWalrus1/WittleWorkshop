# FROM rust:latest

# COPY ./ /api

# WORKDIR /api

# RUN cargo build

# ENV ROCKET_PROFILE=release

# EXPOSE 8000

# CMD ["cargo", "run"]

# syntax=docker/dockerfile:1
FROM mcr.microsoft.com/mssql/server:2022-latest
ENV ACCEPT_EULA=Y

USER root

RUN apt-get update
RUN apt-get install -y dotnet-sdk-8.0
RUN apt-get install -y aspnetcore-runtime-8.0
RUN apt-get install -y dotnet-runtime-8.0

RUN apt-get install unzip libunwind8 libicu55 -y

# Install SQLPackage for Linux and make it executable
RUN wget -progress=bar:force -q -O sqlpackage.zip https://go.microsoft.com/fwlink/?linkid=2113331 \
&& unzip -qq sqlpackage.zip -d /opt/sqlpackage \
&& chmod +x /opt/sqlpackage/sqlpackage \
&& chown -R mssql /opt/sqlpackage \
&& mkdir /tmp/db \
&& chown -R mssql /tmp/db

RUN dotnet tool update -g linux-dev-certs
RUN dotnet dev-certs https
# RUN dotnet linux-dev-certs install

RUN dotnet tool install -g microsoft.sqlpackage