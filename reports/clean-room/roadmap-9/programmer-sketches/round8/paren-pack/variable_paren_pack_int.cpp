template <class... A> int one(A... a) { int v(a...); return v; }
int main() { return one(3) == 3 ? 0 : 1; }
