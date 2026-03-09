pub fn is_armstrong_number(num: u32) -> bool {
    // convert number to string s
    let s = num.to_string();
    println!("num is {num}. s is {s}");

    // find length of string l
    let l = s.len() as u32;
    println!("l is {l}");

    // initialize running_total to 0
    let mut running_total = 0;

    // loop through chars in string s
    for i in s.chars() {
        println!("i is {i}");
        let n = i.to_digit(10).expect("not a valid digit") as u32;

        // calculate n ^ l
        let e = n.pow(l);
        
        // add result to running_total
        running_total += e;
    }

    println!("running_total is {running_total}");
    running_total == num
}
