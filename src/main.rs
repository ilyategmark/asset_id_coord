use std::ops::{Range, RangeInclusive};

struct LatRange {
    index: u64,
    lat_min: f64,
    lat_max: f64,
    first_asset_id_min: u64,
    last_asset_id_max: u64, // this need to be updated in the table, because now it's first, not last
    k: f64,
    b: f64,
    assets_per_lat_circle: u64,
}

#[cfg_attr(rustfmt, rustfmt_skip)] // We don't want rustfmt to split it across several lines
    static LAT_RANGES: [LatRange; 21] = [
        LatRange {index: 1,     lat_min: 90.0 - 3.0 / 60.0,     lat_max: 90.0 - 2.0 / 60.0,     first_asset_id_min: 9, last_asset_id_max: 40, k: -960.0, b: 86377.0, assets_per_lat_circle: 16}, 
        LatRange {index: 2,     lat_min: 90.0 - 7.0 / 60.0,     lat_max: 90.0 - 4.0 / 60.0,     first_asset_id_min: 41, last_asset_id_max: 168, k: -1920.0, b: 172713.0, assets_per_lat_circle: 32}, 
        LatRange {index: 3,     lat_min: 90.0 - 14.0 / 60.0,    lat_max: 90.0 - 8.0 / 60.0,     first_asset_id_min: 169, last_asset_id_max: 616, k: -3840.0, b: 345257.0, assets_per_lat_circle: 64}, 
        LatRange {index: 4,     lat_min: 90.0 - 28.0 / 60.0,    lat_max: 90.0 - 15.0 / 60.0,    first_asset_id_min: 617, last_asset_id_max: 2_408, k: -7680.0, b: 689897.0, assets_per_lat_circle: 128}, 
        LatRange {index: 5,     lat_min: 89.0 + 3.0 / 60.0,     lat_max: 90.0 - 29.0 / 60.0,    first_asset_id_min: 2_409, last_asset_id_max: 9_832, k: -15360.0, b: 1377385.0, assets_per_lat_circle: 256}, 
        LatRange {index: 6,     lat_min: 88.0 + 6.0 / 60.0,     lat_max: 89.0 + 2.0 / 60.0,     first_asset_id_min: 9_833, last_asset_id_max: 39_016, k: -30720.0, b: 2744937.0, assets_per_lat_circle: 512}, 
        LatRange {index: 7,     lat_min: 86.0 + 11.0 / 60.0,    lat_max: 88.0 + 5.0 / 60.0,     first_asset_id_min: 39_017, last_asset_id_max: 156_776, k: -61440.0, b: 5450857.0, assets_per_lat_circle: 1024}, 
        LatRange {index: 8,     lat_min: 82.0 + 21.0 / 60.0,    lat_max: 86.0 + 10.0 / 60.0,    first_asset_id_min: 156_777, last_asset_id_max: 627_816, k: -122880.0, b: 10744937.0, assets_per_lat_circle: 2048}, 
        LatRange {index: 9,     lat_min: 75.0 - 27.0 / 60.0,    lat_max: 82.0 + 20.0 / 60.0,    first_asset_id_min: 627_817, last_asset_id_max: 2_544_744, k: -245760.0, b: 20862057.0, assets_per_lat_circle: 4096}, 
        LatRange {index: 10,    lat_min: 58.0 - 13.0 / 60.0,    lat_max: 75.0 - 28.0 / 60.0,    first_asset_id_min: 2_544_745, last_asset_id_max: 10_785_896, k: -491520.0, b: 39179369.0, assets_per_lat_circle: 8192}, 
        LatRange {index: 11,    lat_min: -58.0 + 14.0 / 60.0,   lat_max: 58.0 - 14.0 / 60.0,    first_asset_id_min: 10_785_897, last_asset_id_max: 124_376_168, k: -983040.0, b: 67572841.0, assets_per_lat_circle: 16384}, 
        LatRange {index: 12,    lat_min: -75.0 + 28.0 / 60.0,   lat_max: -58.0 + 13.0 / 60.0,   first_asset_id_min: 124_376_169, last_asset_id_max: 132_617_320, k: -491520.0, b: 95974505.0, assets_per_lat_circle: 8192}, 
        LatRange {index: 13,    lat_min: -82.0 - 20.0 / 60.0,   lat_max: -75.0 + 27.0 / 60.0,   first_asset_id_min: 132_617_321, last_asset_id_max: 134_534_248, k: -245760.0, b: 114295913.0, assets_per_lat_circle: 4096}, 
        LatRange {index: 14,    lat_min: -86.0 - 10.0 / 60.0,   lat_max: -82.0 - 21.0 / 60.0,   first_asset_id_min: 134_534_249, last_asset_id_max: 135_005_288, k: -122880.0, b: 124415081.0, assets_per_lat_circle: 2048}, 
        LatRange {index: 15,    lat_min: -88.0 - 5.0 / 60.0,    lat_max: -86.0 - 11.0 / 60.0,   first_asset_id_min: 135_005_289, last_asset_id_max: 135_123_048, k: -61440.0, b: 129710185.0, assets_per_lat_circle: 1024}, 
        LatRange {index: 16,    lat_min: -89.0 - 2.0 / 60.0,    lat_max: -88.0 - 6.0 / 60.0,    first_asset_id_min: 135_123_049, last_asset_id_max: 135_152_232, k: -30720.0, b: 132416617.0, assets_per_lat_circle: 512}, 
        LatRange {index: 17,    lat_min: -90.0 + 29.0 / 60.0,   lat_max: -89.0 - 3.0 / 60.0,    first_asset_id_min: 135_152_233, last_asset_id_max: 135_159_656, k: -15360.0, b: 133784425.0, assets_per_lat_circle: 256}, 
        LatRange {index: 18,    lat_min: -90.0 + 15.0 / 60.0,   lat_max: -90.0 + 28.0 / 60.0,   first_asset_id_min: 135_159_657, last_asset_id_max: 135_161_448, k: -7680.0, b: 134472041.0, assets_per_lat_circle: 128}, 
        LatRange {index: 19,    lat_min: -90.0 + 8.0 / 60.0,    lat_max: -90.0 + 14.0 / 60.0,   first_asset_id_min: 135_161_449, last_asset_id_max: 135_161_896, k: -3840.0, b: 134816745.0, assets_per_lat_circle: 64}, 
        LatRange {index: 20,    lat_min: -90.0 + 4.0 / 60.0,    lat_max: -90.0 + 7.0 / 60.0,    first_asset_id_min: 135_161_897, last_asset_id_max: 135_162_024, k: -1920.0, b: 134989321.0, assets_per_lat_circle: 32}, 
        LatRange {index: 21,    lat_min: -90.0 + 2.0 / 60.0,    lat_max: -90.0 + 3.0 / 60.0,    first_asset_id_min: 135_162_025, last_asset_id_max: 135_162_056, k: -960.0, b: 135075673.0, assets_per_lat_circle: 16}, 
    ];

fn main() {
    println!("asset_id is {}", from_coord(19.37, 61.7));

    let s = to_coord(48545633);
    println!("coord is {} {}", s.0, s.1);
}

fn to_coord(asset_id: u64) -> (f64, f64) {
    let mut s = &LAT_RANGES[0]; // We assign it here to make sure it's initialised.

    for i in &LAT_RANGES {
        let id_range = RangeInclusive::new(i.first_asset_id_min, i.last_asset_id_max);

        if id_range.contains(&asset_id) {
            s = i;
            break;
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)] // We don't want rustfmt to split it across several lines
    let lat = s.lat_min + (((&asset_id - s.first_asset_id_min) / s.assets_per_lat_circle) as f64) / 60.0; // Maybe we need to add 1 here. Will check.

    #[cfg_attr(rustfmt, rustfmt_skip)] // We don't want rustfmt to split it across several lines
    let long = (((&asset_id - s.first_asset_id_min) % s.assets_per_lat_circle) as f64) / (s.assets_per_lat_circle as f64) * 360.0 - 180.0;

    return (lat, long);
}

fn from_coord(lat: f64, long: f64) -> u64 {
    /// Latitude is in range [-90 .. +90]
    /// Longitude is in range [-180 .. +180]

    // Length of equator in km
    const EQUATOR_KM: f64 = 21_340.0;

    // Height of an assets in km
    const ASSET_HEIGHT_KM: f64 = 0.9821;

    // One degree of latitude is divided by this number of rows
    const ROWS_IN_LAT_DEG: f64 = 60.0;

    // Finding the closest actual latitude
    let actual_latitude: f64 = (lat * ROWS_IN_LAT_DEG).round() / ROWS_IN_LAT_DEG;
    println!("actual latitude is {}", actual_latitude);

    // Length of a circle at a given Latitude
    let circumference_km: f64 = EQUATOR_KM * actual_latitude.to_radians().cos();
    println!("circumference is {} km", circumference_km);

    // Desired number of approximately squared assets at this Latitude
    let desired_num_of_assets_per_lat_circle: f64 = circumference_km / ASSET_HEIGHT_KM;
    println!(
        "desired_num_of_assets_per_lat_circle is {}",
        desired_num_of_assets_per_lat_circle
    );

    // Actual number of assets on latitude is a power of 2
    let actual_num_of_assets_per_lat_circle: f64 =
        desired_num_of_assets_per_lat_circle.log2().round().exp2();
    println!(
        "actual_num_of_assets_per_lat_circle is {}",
        actual_num_of_assets_per_lat_circle
    );

    // Actual asset length on this latitude in km
    let asset_length_km: f64 = circumference_km / actual_num_of_assets_per_lat_circle;
    println!("asset_length is {} km", asset_length_km);

    // Actual area of assets on this latitude in squared km
    let asset_area_km2: f64 = asset_length_km * ASSET_HEIGHT_KM;
    println!("asset_area is {} km2", asset_area_km2);

    // Total area of all assets on this latitude
    let total_area_lat_km2: f64 = asset_area_km2 * actual_num_of_assets_per_lat_circle;
    println!("total_area_lat is {} km2", total_area_lat_km2);

    // Angular length of an asset on this Latitude, degrees
    let ang_len_asset_deg: f64 = 360.0 / actual_num_of_assets_per_lat_circle;
    println!("ang_len_asset_deg is {}", ang_len_asset_deg);

    // Finding closest actual longitude
    let actual_longitude_deg: f64 = (long / ang_len_asset_deg).round() * ang_len_asset_deg;
    println!("actual_longitude is {} deg", actual_longitude_deg);

    let mut s = &LAT_RANGES[0]; // We assign it here to make sure it's initialised.

    for i in &LAT_RANGES {
        let lat_range = Range {
            start: i.lat_min - 0.005,
            end: i.lat_max + 0.005,
        };

        if lat_range.contains(&actual_latitude) {
            s = &i;
            break;
        }
    }

    let first_asset_id = s.b + s.k * actual_latitude;
    println!("first_asset_id is {}", first_asset_id);

    assert_eq!(
        actual_num_of_assets_per_lat_circle as u64,
        s.assets_per_lat_circle
    ); // This check shouldn't be in production, because blockchain should never panic.

    // The first asset is located at longitude -180 deg.
    let asset_id = first_asset_id + (actual_longitude_deg + 180.0) / ang_len_asset_deg;
    println!("result_asset_id is {}", asset_id);

    return asset_id as u64;
}
