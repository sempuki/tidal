// tidal-rules: FN-7
// tidal-expect: accept
// tidal-flags: -Werror -Wreturn-type
// tidal-note: A throw-expression and a call of a [[noreturn]] function end a path, as a return does. A try block and its handler that both return end every path.
[[noreturn]] void stop();
int thrown() { throw 0; }
int stopped() { stop(); }
int branch(bool b) { if (b) return 1; stop(); }
int handled() { try { return 1; } catch (...) { return 2; } }
