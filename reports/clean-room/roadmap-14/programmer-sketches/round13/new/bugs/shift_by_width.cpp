// A mask is made by shifting 1 by the width of the type.
unsigned mask(int bits) { return (1u << bits) - 1; }
int main() { return mask(32) == 0xFFFFFFFFu ? 0 : 1; }
