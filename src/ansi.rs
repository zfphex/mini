pub const FG_BLACK: &str = "\x1B[30m";
pub const FG_RED: &str = "\x1B[31m";
pub const FG_GREEN: &str = "\x1B[32m";
pub const FG_YELLOW: &str = "\x1B[33m";
pub const FG_BLUE: &str = "\x1B[34m";
pub const FG_MAGENTA: &str = "\x1B[35m";
pub const FG_CYAN: &str = "\x1B[36m";
pub const FG_WHITE: &str = "\x1B[37m";
pub const FG_BRIGHTBLACK: &str = "\x1B[90m";
pub const FG_BRIGHTRED: &str = "\x1B[91m";
pub const FG_BRIGHTGREEN: &str = "\x1B[92m";
pub const FG_BRIGHTYELLOW: &str = "\x1B[93m";
pub const FG_BRIGHTBLUE: &str = "\x1B[94m";
pub const FG_BRIGHTMAGENTA: &str = "\x1B[95m";
pub const FG_BRIGHTCYAN: &str = "\x1B[96m";
pub const FG_BRIGHTWHITE: &str = "\x1B[97m";
pub const FG_RESET: &str = "\x1B[37m";

pub const BG_BLACK: &str = "\x1B[40m";
pub const BG_RED: &str = "\x1B[41m";
pub const BG_GREEN: &str = "\x1B[42m";
pub const BG_YELLOW: &str = "\x1B[43m";
pub const BG_BLUE: &str = "\x1B[44m";
pub const BG_MAGENTA: &str = "\x1B[45m";
pub const BG_CYAN: &str = "\x1B[46m";
pub const BG_WHITE: &str = "\x1B[47m";
pub const BG_BRIGHTBLACK: &str = "\x1B[100m";
pub const BG_BRIGHTRED: &str = "\x1B[101m";
pub const BG_BRIGHTGREEN: &str = "\x1B[102m";
pub const BG_BRIGHTYELLOW: &str = "\x1B[103m";
pub const BG_BRIGHTBLUE: &str = "\x1B[104m";
pub const BG_BRIGHTMAGENTA: &str = "\x1B[105m";
pub const BG_BRIGHTCYAN: &str = "\x1B[106m";
pub const BG_BRIGHTWHITE: &str = "\x1B[107m";
pub const BG_RESET: &str = "\x1B[49m";

pub const RESET: &str = "\x1b[0m";

pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINE: &str = "\x1b[4m";
pub const FAST_BLINKING: &str = "\x1b[5m;1m";
pub const SLOW_BLINKING: &str = "\x1b[5m;2m";
pub const INVERT: &str = "\x1b[7m";
pub const HIDDEN: &str = "\x1b[8m";
pub const STRIKETHROUGH: &str = "\x1b[9m";

pub const NO_BOLD: &str = "\x1b[21m";
pub const NO_BOLD_OR_DIM: &str = "\x1b[22m";
pub const NO_ITALIC: &str = "\x1b[23m";
pub const NO_UNDERLINE: &str = "\x1b[24m";

//TODO: Test fast and slow blinking does "\x1b[25m;2" work?
//Or does this do both?
pub const NO_BLINKING: &str = "\x1b[25m";
pub const NO_INVERT: &str = "\x1b[27m";
pub const NO_HIDDEN: &str = "\x1b[28m";
pub const NO_STRIKETHROUGH: &str = "\x1b[29m";
