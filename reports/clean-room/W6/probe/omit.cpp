struct S { const char* k; int v; bool e = true; };
S a{"x"};            // positional, v omitted
S b{.k = "y"};       // designated, v omitted
S c{};               // empty
S d{"z", 1};         // e omitted, has default
int main() { return a.v + b.v + c.v + (d.e ? 0 : 1); }
