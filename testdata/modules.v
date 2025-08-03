module test_and(input a, input b, output c);
    assign c = a & b;
endmodule

module test_or(input a, input b, output c);
    assign c = a | b;
endmodule

module test_xor(input a, input b, output c);
    assign c = a ^ b;
endmodule