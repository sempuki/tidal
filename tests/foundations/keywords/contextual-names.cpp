// tidal-rules: KW-3
// tidal-expect: accept
int override = 1;
int final = 2;
int import = 3;
int module = 4;
struct Base { virtual int f() { return 0; } };
struct Leaf final : Base { int f() override { return override + final; } };
