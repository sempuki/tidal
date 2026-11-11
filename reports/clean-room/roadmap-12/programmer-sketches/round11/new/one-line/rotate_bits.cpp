// A rotate on unsigned values, with the shift count masked.
constexpr unsigned rotl(unsigned v, int s) { s &= 31; return s ? (v << s) | (v >> (32 - s)) : v; }
static_assert(sizeof(unsigned) == 4);
int main() { return rotl(0x80000001u, 1) == 3u && rotl(5u, 32) == 5u ? 0 : 1; }
