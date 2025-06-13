#!/bin/bash


echo "Generating largefile.txt using Rust in ./GenerateFile ..."
cd GenerateFile || { echo "Directory GenerateFile not found!"; exit 1; }

# Compile and run the Rust file generator
rustc generate_20gb_text.rs -o generate_20gb_text
./generate_20gb_text
echo "-----------------------"

# Move the generated file to the root Benchmark directory if it's not already there
if [ -f largefile_rust.txt ]; then
    mv largefile_rust.txt ../largefile.txt
fi

# Cleanup
rm -f generate_20gb_text
cd ..


# Check input parameter
if [[ "$1" == "singlethread" ]]; then
    DIR="SingleThread"
elif [[ "$1" == "multithread" ]]; then
    DIR="MultiThread"
else
    echo "Usage: $0 [singlethread|multithread]"
    exit 1
fi

echo "Running benchmarks in ./$DIR ..."
echo -e "\n"

cd "$DIR" || { echo "Directory $DIR not found!"; exit 1; }

# Go
if [ -f wordcount.go ]; then
    echo "-----------------------"
    echo "|Benchmarking using Go|"
    echo "-----------------------"
    go build -o wordcount_go wordcount.go
    ./wordcount_go
    echo "-----------------------"
    echo -e "\n"
    rm -f wordcount_go
fi

# Rust
if [ -f wordcount.rs ]; then
    echo "-------------------------"
    echo "|Benchmarking using Rust|"
    echo "-------------------------"
    rustc wordcount.rs -o wordcount_rust
    ./wordcount_rust
    echo "-------------------------"
    echo -e "\n"
    rm -f wordcount_rust
fi


# Java
if [ -f WordCount.java ]; then
    echo "-------------------------"
    echo "|Benchmarking using Java|"
    echo "-------------------------"
    javac WordCount.java
    java -Xmx2g WordCount
    echo "-------------------------"
    echo -e "\n"
    rm -f *.class
fi

# Python
if [ -f wordcount.py ]; then
    echo "---------------------------"
    echo "|Benchmarking using Python|"
    echo "---------------------------"
    python3 wordcount.py
    echo "---------------------------"
    echo -e "\n"
fi

cd ..

echo "Deleting the generated source file"
rm -f largefile.txt
echo -e "\n"

echo "benchmarking done successfully!"
