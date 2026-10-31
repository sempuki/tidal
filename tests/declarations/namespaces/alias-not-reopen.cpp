// tidal-rules: NS-5
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
namespace long_name { int a; }
namespace ln = long_name;
namespace ln { int b; }
