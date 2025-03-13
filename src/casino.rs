use rand::Rng;

pub fn play(money: i32) -> i32 {
    if money > 0 {
        let mut rng = rand::rng();
        let slots: (i32, i32, i32) = (
            rng.random_range(0..=1),
            rng.random_range(0..=1),
            rng.random_range(0..=1),
        );

        match slots {
            (1, 1, 1) => ((money as f64) * 2.5) as i32,        
            (_, 1, 1) | (1, 1, _) | (1, _, 1) => ((money as f64) * 1.5).ceil() as i32, 
            _ => 0,                               
        }
    } else {
        0
    }
}
