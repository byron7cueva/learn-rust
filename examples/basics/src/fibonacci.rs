pub fn calcular_fibonnaci(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    return calcular_fibonnaci(n - 1) + calcular_fibonnaci(n - 2);
}