struct S { int v = 0; };
int main(int argc, char**) { S x, y; auto& r = argc > 0 ? x : y; r.v = 3; return x.v == 3 ? 0 : 1; }
