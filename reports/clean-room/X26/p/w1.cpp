// Which left operands of a built-in comma get a warning today under -Wunused-value.
struct P { int id; P(int i) : id(i) {} ~P() {} };
struct S { int m; };
int f();
void g();
int (*pf)() = f;
int operator""_k(unsigned long long v) { return int(v); }
volatile int vol = 0;
int L01(int a, int b) { return (a, b); }                 // name
int L02(int b) { return (1, b); }                        // literal
int L03(int a, int b) { return (a + 1, b); }             // arithmetic
int L04(int a, int b) { return (a + f(), b); }           // arithmetic holding a call
int L05(int a, int b) { return (a == 1, b); }            // comparison
int L06(int b) { return (vol, b); }                      // volatile name
int L07(int b) { return (P(1), b); }                     // functional cast to a class
int L08(int b) { return (P{1}, b); }                     // braced temporary
int L09(int b) { return (new int, b); }                  // new expression
int L10(int b) { return (5_k, b); }                      // user-defined literal
int L11(int b) { return ([] { return 1; }, b); }         // lambda expression
int L12(int a, int b) { return (sizeof a, b); }          // sizeof
int L13(S s, int b) { return (s.m, b); }                 // member access
int L14(int* p, int b) { return (p[0], b); }             // subscript
int L15(int* p, int b) { return (*p, b); }               // indirection
int L16(int a, int b) { return ((long)a, b); }           // cast to another type
int L17(int a, int b) { return (static_cast<long>(a), b); }
int L18(int a, int b) { return (++a, b); }               // increment
int L19(int a, int b) { return (a = 1, b); }             // assignment
int L20(int a, int b) { return (a += 1, b); }            // compound assignment
int L21(int b) { return (f(), b); }                      // call
int L22(int b) { return (pf(), b); }                     // call through a pointer
int L23(int a, int b) { return ((void)a, b); }           // cast to void
int L24(bool c, int b) { return (c ? f() : f(), b); }    // conditional of calls
int L25(bool c, int a, int b) { return (c ? a : b, b); } // conditional of names
int L26(int b) { return ([] { return 1; }(), b); }       // call of a lambda
int L27(int b) { return (g(), b); }                      // void call
int L28(int a, int b) { return (a++, b); }               // postfix increment
int L29(int a, int b) { return ((a, f()), b); }          // comma whose right operand is a call
int L30(int a, int b) { return ((f(), a), b); }          // comma whose right operand is a name
int main() { return 0; }
