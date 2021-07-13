cd examples
for chip in * ; do
    cd $chip
    for example in * ; do
        cd $example
        echo "Testing $example for $chip"
        if [[ $chip == "atmega328p" ]];
        then
            AVR_CPU_FREQUENCY_HZ=16000000 cargo +nightly-2021-01-07 build -Z build-std=core --release --target ../../../avr-chips/avr-atmega328p.json >> /dev/null   || exit 1
        else 
            AVR_CPU_FREQUENCY_HZ=16000000 cargo +nightly-2021-01-07 build -Z build-std=core --release --target ../../../avr-chips/avr-atmega2560.json >> /dev/null   || exit 1
        fi
        cd ..
    done
    cd ..
done
