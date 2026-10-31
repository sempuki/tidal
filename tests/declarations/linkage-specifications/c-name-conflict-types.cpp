// tidal-rules: LSP-4
// tidal-expect: reject
namespace a { extern "C" void conflict(int); }
namespace b { extern "C" void conflict(double); }
