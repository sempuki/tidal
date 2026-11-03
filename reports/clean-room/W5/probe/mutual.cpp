bool is_odd(int);
constexpr auto is_even(int n) { if (n == 0) return true; return is_odd(n - 1); }
bool is_odd(int n) { if (n == 0) return false; return is_even(n - 1); }
int main(int argc, char**) { return is_even(argc + 3) ? 0 : 1; }
