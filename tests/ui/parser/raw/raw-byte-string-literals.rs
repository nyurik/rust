// ignore-tidy-cr

pub fn main() {
    br"a
    br"é";  //~ ERROR non-ASCII character in raw byte string literal
    br##~"a"~##;  //~ ERROR only `#` is allowed in raw string delimitation
}