// A pack in an init-capture (C++20).
template <class... T> auto bind_sum(T... xs) { return [... ys = xs] { return (ys + ... + 0); }; }
int main() { return bind_sum(1, 2, 3)() - 6; }
