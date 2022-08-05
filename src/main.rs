fn main() {
    let xor = "
    3 5
    1 >
    < < n

    1 >
    < < n

    3 >
    %
    0 >
    < % < n

    3 >
    %
    2 >
    < % < n

    n

    0 > * * * * * * * <";

    let small_xor = "
    3
    5

    1 > % 0 >
    < n n < %
    < < n n n

    15 5

    1 > % 0 >
    < n n < %
    < < n n n

    ";

    /*
     * XOR logic is defined from counter @ 3
     * program starts and jumps to counter @31 where our code is
     *
     * push the counter where we want to go after running the "function"
     *
     * push the arguments for the function (15 and 4)
     *
     * jump to counter @ 3 (xor function)
     *
     * function then calculates 15 xor 5
     *
     * function swaps the top 2 values so that it can get the return address
     *
     * function then jumps back to the return address.
     *
     * we've jumped to the last 2 instructions to end up with 420, 69 and 15 ^ 5 on the stack.
     */
    let functions = "
    31 1 !

    1 > % 0 >
    < n n < %
    < < n n n

    0 > * %
    0 > * %
    < % < 1 !

    37
    15 5 3 1 !

    69 420
    .
    ";

    // Calling convention
    /*
     * Return Address
     * Operand A, Operand B
     * Address of function, true, jump
     */

    let function_with_halt = "
    6
    15 5
    7 1 !
    .

    1 > % 0 >
    < n n < %
    < < n n n
    0 > * %
    0 > * %
    < % < 1 !
    .
    ";

    nand_lang::start(function_with_halt.to_string()).unwrap();
}
/*
 *
 * a, b (0/0)
 * a, b, (0/b)
 * b (a / b)
 * b, b (a/b)
 * !b&a (a/b)
 */

// a nand a = !a
// a, b, !a

// b nand b = !b
// a, b, !a, !b

// a nand !b = c
// a, b, !a, !b, c

// b nand !a = d
// a, b, !a, !b, c, d

// c nand d = a^b
// a, b, !a, !b, c, d, a^b

// 5 ^ 3
// 0101 ^ 0011 = 0110
// 0101 & 0011 = 0001
// 0101 | 0011 = 0111
// 0110 ?  ?  = 1000
