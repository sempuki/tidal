struct S { int v = 0; int get() const; };
int S::get() const { return v; }
int main() { return S{}.get(); }
