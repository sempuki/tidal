// S lvalue and S xvalue give a prvalue; the move from the xvalue is deleted.
template <class T> T&& mv(T& t) { return static_cast<T&&>(t); }
struct S { S() = default; S(const S&) = default; S(S&&) = delete; };
S s1, s2;
int main() { bool c = true; S r = c ? s1 : mv(s2); return 0; }
