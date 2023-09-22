use std::thread::sleep;
use std::time::Duration;

const BUCKET_SIZE: i32 = 5;
const REFILL_RATE: i32 = 10;
const PROCESS_RATE: i32 = 3;

fn main() {
    /*
    How many requests the bucket can store without overflowing.
    overflowing = rejecting the request
    */
    let mut bucket: i32 = BUCKET_SIZE;

    // How many requests the bucket can process per second.

    let example_duration_in_seconds = 10;

    let mut request_id = 1;

    for s in 1..= example_duration_in_seconds {
        refill_bucket(&mut bucket);
        sleep(Duration::from_secs(1));

        process_request(&mut bucket, PROCESS_RATE, &mut request_id);

        println!("Bucket capacity at the end of second {}: {}", s, bucket);
        println!();
    }
}

fn process_request(bucket: &mut i32, process_rate: i32, request_id: &mut i32) {
    for _ in 0..process_rate {
        accessing_a_restrict_value(&request_id, bucket);
        *request_id += 1;
    }
}

fn accessing_a_restrict_value(request_id: &i32, bucket: &mut i32) {
    if *bucket == 0 {
        println!("Overflowing the request {}.", request_id);
        return;
    }

    println!("Processing the request {}.", request_id);
    *bucket -= 1;
}

fn refill_bucket(bucket: &mut i32) {
    // I'm purposely providing a value lower than the process_rate
    if *bucket < BUCKET_SIZE {
        *bucket += REFILL_RATE;
    }
}
