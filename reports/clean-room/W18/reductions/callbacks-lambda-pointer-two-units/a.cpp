int (*p)(int) = [](int x) { return x; };
int main() { return p(0); }
