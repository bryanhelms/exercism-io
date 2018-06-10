pub fn verse(n: i32) -> String {
    let bottles = |b: i32| -> String {
        let formatter = |amount: &str| -> String { format!("{} bottles", amount) };
        match b {
            0 => formatter("no more"),
            1 => format!("1 bottle"),
            _ => formatter(&b.to_string())
        }
    };
    let first_part = {
        let amount = bottles(n);
        format!("{} of beer on the wall, {} of beer.\n", uppercase_first(&amount), amount)
    };
    let second_part = {
        if n == 0 {
            format!("Go to the store and buy some more, 99 bottles of beer on the wall.\n")
        } else {
            format!("Take {} down and pass it around, {} of beer on the wall.\n",
                if n == 1 { "it" } else { "one" },
                bottles(n - 1))
        }
    };

    [first_part, second_part].concat()
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::new();
    for current in ((end)..(start+1)).rev() {
        song.push_str(&verse(current));
        if current != end { song.push_str("\n") };
    }

    song
}

fn uppercase_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect()
    }
}
