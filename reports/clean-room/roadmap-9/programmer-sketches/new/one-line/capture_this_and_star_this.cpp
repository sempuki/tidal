struct C { int v = 4; auto by_ref() { return [this] { return v * 2; }; } auto by_copy() const { return [*this] { return v * 3; }; } };
int main() { C c; auto f = c.by_ref(); auto g = c.by_copy(); c.v = 5; return f() == 10 && g() == 12 ? 0 : 1; }
