use super::*;

pub static DELIMITER: &str = " | ";
pub static BLOCKS: &[Block] = &[
    Block {
        icon: "",
        command: "date",
    },
    Block {
        icon: "battery: ",
        command: "apm -l",
    },
];
