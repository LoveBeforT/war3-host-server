cd `dirname $0` && pwd

rm -rf output/
mkdir output/

if [ ! -n "$1" ] ;then
    cargo build
    mv target/debug/war3-host-server output/server
else
    cargo build --$1
    mv target/$1/war3-host-server output/server
fi

cp config/* output/
cp script/* output/

# cargo clean
