from rust

RUN apt-get update && apt-get -y install clang golang gcc-mingw-w64 jsonnet

RUN rustup target add x86_64-pc-windows-gnu
