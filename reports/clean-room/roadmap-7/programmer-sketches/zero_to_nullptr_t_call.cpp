bool neg(signed char v, decltype(nullptr)) { return v < 0; }
int main() { return neg(-1, 0) ? 0 : 1; }
