struct Config { inline static int level = 3; static constexpr int max = 9; };
const int* other_level() { return &Config::level; }
