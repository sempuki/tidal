#include "registry.h"

namespace app {

namespace {
int add(int a, int b) { return a + b; }
AutoRegister r_add({.name = "add", .kind = Kind::add, .fn = add});
AutoRegister r_mul({"mul", Kind::mul, [](int a, int b) { return a * b; }});
AutoRegister r_neg({.name = "neg", .kind = Kind::neg, .fn = +[](int a, int) { return -a; }});
}  // namespace

template <class T>
T fold_apply(int (*)(int, int), T first) { return first; }

template int fold_apply<int>(int (*)(int, int), int);

const char* kind_name(Kind k) {
    switch (k) {
        using enum Kind;
        case add: return "add";
        case mul: return "mul";
        case neg: return "neg";
    }
    return "?";
}

bool same(const char* a, const char* b) {
    while (*a && *a == *b) { ++a; ++b; }
    return *a == *b;
}

const Command* Registry::find(const char* name) const {
    for (const Command& c : *this)
        if (same(c.name, name)) return &c;
    return nullptr;
}

}  // namespace app
