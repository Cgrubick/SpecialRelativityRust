
use std::fs::File;
use std::io::prelude::*;
//what is the proper time and dilated time it would take to get to X planet at Y percentage the speed of c

//dilated time = (proper time)/(sqrt(1 - (v^2/c^2)))
mod time_dilation;
mod test_time_dilation;
fn main() {
    let star_systems = vec![("Alpha Centauri", 4.37), ("Barnards Star", 6.0)];

    let speed = 0.5;

    let mut file = File::create("time_dilation.txt").unwrap();

    for(name,distance) in star_systems.iter(){
        let (proper_time_year,dilated_time_year) = time_dilation::time_dilation_calc(*distance,speed);
        let result = format!("{}: Proper Time, {:.2} years, Dilated Time: {:.2} years \n", name,proper_time_year,dilated_time_year);
        file.write_all(result.as_bytes()).unwrap();
    }
   
}
