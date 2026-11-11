using L = decltype([] { return 3; });
int main() { L l; return l() == 3 ? 0 : 1; }
