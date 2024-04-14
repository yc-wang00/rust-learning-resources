fn main() {
    let mut i = 1;
    loop {
        i *= 10; // panic：试图进行可能溢出的乘法（但只会在调试构建中出现）
    }
}
