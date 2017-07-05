pub fn calc_distance(pos1: &[f64; 3], pos2: &[f64; 3]) -> f64 {
    ((pos2[0] - pos1[0]).powi(2)
     + (pos2[1] - pos1[1]).powi(2)
     + (pos2[2] - pos1[2]).powi(2)).sqrt()
}

