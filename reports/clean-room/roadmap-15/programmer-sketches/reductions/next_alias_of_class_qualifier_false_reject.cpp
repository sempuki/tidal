struct S { static constexpr int n = 3; };
using B = S;
int main() { return B::n - 3; }
