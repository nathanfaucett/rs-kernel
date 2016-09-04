

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ErrorCode {
    NoMem = 0,
}

static ERROR_MESSAGES: [&'static str; 1] = [
    "Out of memory"
];

create_errno!(Error, ErrorCode, ERROR_MESSAGES);
