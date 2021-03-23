pub mod hyvee;
use dotenv::dotenv;
use std::{env, time::Duration};
use tokio::time;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let lat = env::var("LAT")
        .map_err(|e| println!("could not get latitude, {:?}", e))
        .unwrap()
        .parse::<f32>()
        .unwrap();

    let long = env::var("LONG")
        .map_err(|e| println!("could not get longitude, {:?}", e))
        .unwrap()
        .parse::<f32>()
        .unwrap();

    let rad = env::var("RADIUS")
        .map_err(|e| println!("could not get radius, {:?}", e))
        .unwrap()
        .parse::<f32>()
        .unwrap();

    println!("{:?}, {:?}, {:?}", lat, long, rad);

    let mut current = hyvee::get_locations(lat, long, rad)
        .await
        .map_err(|e| panic!("could not get initial data, {:?}", e))
        .unwrap();

    loop {
        time::sleep(Duration::from_millis(5000)).await;
        let mut i = 0;
        let comparable =  hyvee::get_locations(lat, long, rad)
            .await
            .map_err(|e| println!("could not fetch data, {:?}", e))
            .unwrap();

        loop {
            if i == current.len() {
                break;
            }

            if comparable[i].location.is_covid_vaccine_available && !current[i].location.is_covid_vaccine_available {
                println!("Vaccine available: {:?}", comparable[i].location)
            }

            i = i + 1;
        }

        current = comparable;
    }
}
