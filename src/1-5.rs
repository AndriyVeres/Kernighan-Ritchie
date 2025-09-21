fn main()
{
    let lower: f32 =   0.0;
    let upper: f32 = 300.0;
    let step:  f32 =  20.0;
    let mut celsius: f32 = upper;
    let mut fahr:    f32;

    println!("\n🌡️Temperature");
    println!("+---+------+");
    println!("+°C +  °F  +");
    println!("+---+------+");
    while celsius >= lower {
        fahr = celsius / (5.0/9.0) + 32.0;
        println!("|{:>3.0}|{:>6.1}|", celsius, fahr);
        celsius -= step;
    }
    println!("+---+------+");
}