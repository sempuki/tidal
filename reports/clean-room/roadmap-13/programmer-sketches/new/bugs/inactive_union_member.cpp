// A union is written through one member and read through another.
union Value { int i; float f; };
int read(Value v) { return v.i; }
int main() { Value v; v.f = 1.0f; return read(v) == 0 ? 1 : 0; }
