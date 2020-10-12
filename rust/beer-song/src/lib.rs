macro_rules! str_verse {
    () => {
        "{0} of beer on the wall, {1} of beer.\n{2}{3} of beer on the wall.\n"
    };
}

pub fn verse(index: u32) -> String {
    let index_sub = if index == 0 { 99 } else { index - 1 };
    format!(
        str_verse!(),
        bottle(index, true),
        bottle(index, false),
        take(index),
        bottle(index_sub, false)
    )
}

pub fn sing(start: u32, end: u32) -> String {
    (end..start + 1)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}

fn bottle(index: u32, uppercase: bool) -> String {
    match uppercase {
        _ if index > 1 => format! {"{} bottles", index},
        _ if index == 1 => String::from("1 bottle"),
        true => String::from("No more bottles"),
        false => String::from("no more bottles"),
    }
}

macro_rules! str_take {
    () => {
        "Take {} down and pass it around, "
    };
}

fn take(index: u32) -> String {
    match () {
        _ if index > 1 => format! {str_take!(), "one"},
        _ if index == 1 => format! {str_take!(), "it"},
        _ => String::from("Go to the store and buy some more, "),
    }
}
