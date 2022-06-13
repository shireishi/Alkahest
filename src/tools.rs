use super::{
    Substring,
    info
};

pub fn from_hex(hex_string: String) -> (f32, f32, f32) {
    let hex_len: usize = hex_string.len();
    let mut hex_arr: Vec<String> = Vec::with_capacity(3usize);
    let mut hex_fin: [f32; 3] = [0., 0., 0.];

    match hex_len as i8 {
        3 => {
            for i in 0 .. 3 {
                hex_arr.push(
                    hex_string
                        .chars()
                        .nth(i)
                        .unwrap()
                        .to_string()
                    );
            }

            for i in 0 .. hex_len-1 {
                hex_fin[i] = i64::from_str_radix(
                    &hex_arr[i][..], 16
                ).unwrap() as f32 / 255.;
            }
        },
        6 => {
            for i in (0 .. hex_len).step_by(2) {
                hex_arr.push(hex_string.substring(i, i+2).to_string());
            }
            
            for i in 0 .. 3 {
                hex_fin[i] = i64::from_str_radix(&hex_arr[i], 16).unwrap() as f32 / 255.;
            }
        },
        _ => ()
    }
    info!("{}", hex_string);
    (hex_fin[0], hex_fin[1], hex_fin[2])
}