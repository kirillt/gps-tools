const EARTH_DIAMETER: f64 = 12_742_000.0;

fn radians(angle: f64) -> f64 {
    angle * std::f64::consts::PI / 180.0
}

type Latitude = f64;
type Longtitude = f64;
type Point = (Latitude, Longtitude);

fn distance(point1: Point, point2: Point) -> f64 {
    let ((lat1, lon1), (lat2, lon2)) = (point1, point2);
    let lat1 = radians(lat1);
    let lat2 = radians(lat2);
    let lon1 = radians(lon1);
    let lon2 = radians(lon2);

    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;
    
    let a = (dlat / 2.0).sin().powi(2) +
            (dlon / 2.0).sin().powi(2) * lat1.cos() * lat2.cos();
    
    a.sqrt().asin() * EARTH_DIAMETER
}

fn main() {
    println!(
        "Distance between Saint-Petersburg and Moscow: {:?}m",
        distance((59.9311, 30.3609), (55.7558, 37.6173))
    )
}

//todo: read input from CSV
//todo: use hAcc/vAcc columns
//todo: sAcc/cAcc?
