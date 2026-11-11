// A mask built by shifting 1u by the full width.
unsigned mask(int bits) { return (1u << bits) - 1; }
int main() { return mask(32) == 0xffffffffu ? 0 : 1; }
