use text_io::read;

fn main() {
    println!("Please enter the number of values:");
    let amount: u8 = read!();

    let mut data: Vec<f64> = vec![];
    println!("Please enter values");
    for _ in 0..amount {
        data.push(read!());
    }
    println!("The values you entered are: {data:#?}");
    let data_avg:f64 = data.iter().sum::<f64>()/data.len() as f64;
    let myresult:f64 = (data.iter().map(|x| (data_avg - x).powf(2.0)).sum::<f64>()/(data.len() as f64 - 1.0)).sqrt();
    println!("The experimental error is: {myresult}");
    println!("The avg of the data is: {data_avg}");

    // Optional for window to not close.
    loop {} 
}
