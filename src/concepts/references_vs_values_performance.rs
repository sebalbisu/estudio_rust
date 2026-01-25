#[allow(unused_imports)]
use std::hint::black_box;
#[allow(unused_imports)]
use std::time::Instant;

/*
 ============================================================================
 INDEX - Run all demos
 ============================================================================
*/
#[test]
fn index() {
    benchmarks::medium_strings();
    benchmarks::small_struct();
    benchmarks::large_struct();
    benchmarks::vectors();
}

/*
============================================================================
10. BENCHMARKS - Performance comparisons
============================================================================

    String 1000 chars
    --------------------------------
    transfer is 3.1x faster by reference than cloning the value

    Small struct (16 bytes)
    --------------------------------
    transfer is the SAME SPEED by reference than cloning the value

    Large struct (with Strings)
    --------------------------------
    transfer is 17x faster by reference than cloning the value

    Vector of 1000 i32s
    --------------------------------
    transfer is 20x faster by reference than cloning the value
*/
#[cfg(test)]
mod benchmarks {
    use super::*;

    /*
    String 1000 chars transfer is 3.1x faster by reference than cloning the value
     */
    #[test]
    pub fn medium_strings() {
        fn process_string_by_ref(s: &String) -> usize {
            s.len()
        }
    
        fn process_string_by_value(s: String) -> usize {
            s.len()
        }

        let iterations = 10_000;
        let s = "x".repeat(1000);

        // By reference
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = black_box(process_string_by_ref(black_box(&s)));
        }
        let duration_ref = start.elapsed();

        // By value (with clone)
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = black_box(process_string_by_value(black_box(s.clone())));
        }
        let duration_value = start.elapsed();

        let ratio = duration_value.as_nanos() as f64 / duration_ref.as_nanos().max(1) as f64;

        println!("    By reference: {:?}", duration_ref);
        println!("    By value (clone): {:?}", duration_value);
        println!("    Factor: {:.1}x", ratio);
        assert!(ratio > 1.0, "clone should be slower");
    }

    /*
    Small struct (16 bytes) transfer is the SAME SPEED by reference than cloning the value
     */
    #[test]
    pub fn small_struct() {
        #[derive(Clone, Copy)]
        struct Point {
            _x: f64,
            _y: f64,
        }

        fn process_point_by_ref(point: &Point) -> f64 {
            point._x + point._y
        }
    
        fn process_point_by_value(point: Point) -> f64 {
            point._x + point._y
        }

        let iterations = 10_000;

        // Point (Copy, 16 bytes)
        let points: Vec<Point> = (0..iterations)
            .map(|i| Point {
                _x: i as f64,
                _y: (i as f64) * 2.0,
            })
            .collect();

        let start = Instant::now();
        for p in &points {
            let _ = black_box(process_point_by_ref(black_box(p)));
        }
        let duration_ref_point = start.elapsed();

        let start = Instant::now();
        for p in &points {
            let _ = black_box(process_point_by_value(black_box(*p)));
        }
        let duration_value_point = start.elapsed();
        let ratio_point = duration_value_point.as_nanos() as f64 / 
           duration_ref_point.as_nanos().max(1) as f64;

        println!("    By reference: {:?}", duration_ref_point);
        println!("    By value (copy): {:?}", duration_value_point);
        println!("    Factor: {:.1}x", ratio_point);
        assert!(duration_value_point > duration_ref_point);
    
    }

    /*
    Large struct transfer is 17x faster by reference than cloning the value
     */
    #[test]
    pub fn large_struct() {

        #[derive(Clone)]
        struct User {
            _id: u64,
            _text1: String,
            _text2: String,
            age: u32,
        }

        fn process_user_by_ref(user: &User) -> bool {
            user.age > 18
        }
    
        fn process_user_by_value(user: User) -> bool {
            user.age > 18
        }

        let iterations = 1_000;
        // User (Clone, with Strings)
        let users: Vec<User> = (0..iterations)
            .map(|i| User {
                _id: i,
                _text1: (0..1000).map(|_| 'a').collect(),
                _text2: (0..1000).map(|_| 'a').collect(),
                age: 30,
            })
            .collect();

        let start = Instant::now();
        for user in &users {
            let _ = black_box(process_user_by_ref(black_box(user)));
        }
        let duration_ref_user = start.elapsed();

        let start = Instant::now();
        for user in &users {
            let _ = black_box(process_user_by_value(black_box(user.clone())));
        }
        let duration_value_user = start.elapsed();

        let ratio_user =
            duration_value_user.as_nanos() as f64 / duration_ref_user.as_nanos().max(1) as f64;

        println!("    By reference: {:?}", duration_ref_user);
        println!("    By value (clone): {:?}", duration_value_user);
        println!("    Factor: {:.1}x", ratio_user);
    }

    /*
    Vector of 1000 i32s transfer is 20x faster by reference than cloning the value
     */
    #[test]
    pub fn vectors() {

        fn process_vec_by_ref(vec: &Vec<i32>) -> usize {
            vec.len()
        }
    
        fn process_vec_by_value(vec: Vec<i32>) -> usize {
            vec.len()
        }

        let iterations = 1_000;
        let vecs: Vec<Vec<i32>> = (0..iterations).map(|_| (0..1000).collect()).collect();

        let start = Instant::now();
        for vec in &vecs {
            let _ = black_box(process_vec_by_ref(black_box(vec)));
        }
        let duration_ref = start.elapsed();

        let start = Instant::now();
        for vec in &vecs {
            let _ = black_box(process_vec_by_value(black_box(vec.clone())));
        }
        let duration_value = start.elapsed();

        let ratio = duration_value.as_nanos() as f64 / duration_ref.as_nanos().max(1) as f64;

        println!("    By reference: {:?}", duration_ref);
        println!("    By value (clone): {:?}", duration_value);
        println!("    Factor: {:.1}x", ratio);
    }
}
