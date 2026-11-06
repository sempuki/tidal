constexpr unsigned set(unsigned m, int k) { return m | (1u << k); }
constexpr bool has(unsigned m, int k) { return (m >> k) & 1u; }
int main() { unsigned m = set(set(0, 3), 31); return has(m, 31) && has(m, 3) && !has(m, 4) && m == 0x80000008u ? 0 : 1; }
