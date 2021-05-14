FROM rustlang/rust:nightly
RUN mkdir /rust_learn
WORKDIR /rust_learn
ADD . /rust_learn