struct S { int get() const { return 3; } };
int use_other() { return S{}.get(); }
