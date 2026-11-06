// ICS-26: Derived* to Middle* is a subsequence of Derived* to const Middle*.
struct Base {};
struct Middle : Base {};
struct Derived : Middle {};
Derived* v = nullptr;
int f(Middle*) { return 1; }
int f(const Middle*) { return 2; }
int main() { return f(v) == 1 ? 0 : 1; }
