// tidal-rules: VIRT-14
// tidal-expect: reject
// tidal-error-clang: consteval function .f. cannot override a non-consteval function
// tidal-error-gcc: overriding non-.consteval. function
// tidal-error-edg: consteval member cannot override non-consteval function
struct B { virtual int f() const { return 1; } };
struct D : B { consteval int f() const override { return 2; } };
