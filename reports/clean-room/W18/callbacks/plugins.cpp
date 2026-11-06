// The second unit. It sees only its own declarations (TR-4), so it
// declares the registration function that main.cpp defines.
using Callback = int (*)(int);
bool add_callback(Callback f);

namespace {
int negate(int x) { return -x; }
}

bool plugins_registered =
    add_callback(negate) &&                                // a function name
    add_callback([](auto x) { return x * x; }) &&          // generic lambda: the
                                                           // target type picks int (LAM-10)
    add_callback([](int x) static { return x - 3; });      // static lambda (LAM-13)
