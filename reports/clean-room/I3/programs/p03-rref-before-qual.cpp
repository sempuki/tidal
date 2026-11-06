// ICS-28 before ICS-31: both bind a temporary pointer; the rvalue reference wins.
int n = 0;
int take(const int* const&) { return 1; }
int take(const volatile int*&&) { return 2; }
int main() { return take(&n) == 2 ? 0 : 1; }
