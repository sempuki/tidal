// tidal-rules: LSP-7
// tidal-expect: accept
// tidal-note: A global function with C++ linkage can have the spelling of a C variable or a C function, in either order.
int before_variable();
namespace n { extern "C" int before_variable; }
namespace n { extern "C" int after_variable; }
int after_variable();
int before_function(int);
namespace n { extern "C" int before_function(); }
namespace n { extern "C" int after_function(); }
int after_function(int);
