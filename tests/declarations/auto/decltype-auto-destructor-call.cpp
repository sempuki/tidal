// tidal-rules: AUTO-11
// tidal-expect: reject
// tidal-error-clang: decltype\(auto\). not allowed here
// tidal-error-gcc: expected class-name before .decltype
// tidal-error-edg: expected an identifier
// tidal-note: RV10-1. decltype(auto) follows the tilde of an explicit destructor call. Only decltype of an expression can stand there, and every compiler rejects the placeholder.
struct Item {};
void finish(Item* item) { item->~decltype(auto)(); }
