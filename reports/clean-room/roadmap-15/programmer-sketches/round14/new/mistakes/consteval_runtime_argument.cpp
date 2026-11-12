consteval int sq(int x) { return x * x; }
int main(int argc, char**) { return sq(argc) - 1; }
