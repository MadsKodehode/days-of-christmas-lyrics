fn main() {
    let lyrics_arr = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut day = 1; //Start at day 1
    for _lyrics in lyrics_arr {
        //Use correct suffix
        let suffix = match day {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };

        println!("\n[Verse {day}]");
        println!("On the {day}{suffix} day of Christmas, my true love sent to me");
        //Loop in reverse and print the lyrics
        for i in (0..day).rev() {
            println!("{}", lyrics_arr[i]);
        }

        day += 1; //Next day
    }
}
