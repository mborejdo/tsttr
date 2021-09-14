FROM rust

RUN dpkg --add-architecture i386 && apt-get update && apt-get install -y mingw-w64 wine64 wine32
RUN rustup target add x86_64-pc-windows-gnu
RUN rustup target add i686-pc-windows-gnu

RUN mkdir /root/.cargo

RUN echo '[target.x86_64-pc-windows-gnu]' > /root/.cargo &&\
    echo 'linker = "x86_64-w64-mingw32-gcc"'>> /root/.cargo &&\
    echo 'ar = "x86_64-w64-mingw32-ar"'>> /root/.cargo &&\
    echo 'runner = "wine"'>> /root/.cargo &&\
    echo '[target.i686-pc-windows-gnu]'>> /root/.cargo &&\
    echo 'linker = "i686-w64-mingw32-gcc"'>> /root/.cargo &&\
    echo 'ar = "i686-w64-mingw32-ar"'>> /root/.cargo &&\
    echo 'runner = "wine"'>> /root/.cargo