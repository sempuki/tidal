template <class... T> auto bind(T... t) { return [...xs = t] { return (xs + ...); }; }
int main() { return bind(1, 2, 3)() == 6 ? 0 : 1; }
