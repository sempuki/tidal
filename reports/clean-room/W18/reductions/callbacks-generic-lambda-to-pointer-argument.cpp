int call(int (*f)(int)) { return f(0); }
int main() { return call([](auto x) { return x; }); }
