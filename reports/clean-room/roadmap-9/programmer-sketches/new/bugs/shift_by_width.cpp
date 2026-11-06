unsigned mask(int k) { return 1u << k; }
int main() { return mask(32) == 0 ? 0 : 1; }
