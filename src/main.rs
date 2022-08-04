fn main() {
    nand_lang::start(
        "
3 5 1 > < < n 0 > * * * < 1 > < < n 0 > * * * < 3 > % 0 > < % < n 0 > * * * < 3 > % 2 > < % < n 0 > * * * < n "
        .to_string(),
    )
    .unwrap();
}

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
// 101 ^ 011 = 110
