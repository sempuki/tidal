// tidal-rules: TYN-7
// tidal-expect: reject
// tidal-error-clang: incomplete type .Engine
// tidal-error-gcc: has incomplete type
// tidal-error-edg: incomplete type .Engine. is not allowed
class Engine;
void run(Engine e) {}
