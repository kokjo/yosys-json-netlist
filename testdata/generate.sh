#!/bin/sh
for testcase in *.v; do
    yosys -p "read_verilog ${testcase}; synth; write_json ${testcase%.v}.json" > ${testcase%.v}.log
done