module adder(
    input [16:0] a,
    input [16:0] b,
    output [16:0] c,
);
    assign c = a + b;
endmodule