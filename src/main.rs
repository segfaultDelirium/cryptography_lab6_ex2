
fn get_sbox() -> Vec<u8>{
    vec![0xE, 0x4, 0xD, 0x1, 0x2, 0xF, 0xB, 0x8, 0x3, 0xA, 0x6, 0xC, 0x5, 0x9, 0x0, 0x7]
}


fn functional_push_right(vec: Vec<u8>, value: u8) -> Vec<u8> {
    // vec.into_iter().chain([value].into_iter()).collect()
    let mut vec_clone = vec.clone();
    vec_clone.push(value);
    vec_clone
}


fn create_hex_binary(hex_value: u8) -> Vec<u8>{
    fn create_hex_binary_rec(hex_value: u8, counter: i32, acc: Vec<u8>) -> Vec<u8>{
        if counter < 0 {
            return acc;
        }
        let two_value = (2 as u32).pow(counter as u32) as u8;
        let new_counter = counter - 1;
        if hex_value >= two_value{
            let new_acc = functional_push_right(acc, 1);
            create_hex_binary_rec(hex_value - two_value, new_counter, new_acc)
        }else{
            let new_acc = functional_push_right(acc, 0);
            create_hex_binary_rec(hex_value, new_counter, new_acc)
        }
    }
    create_hex_binary_rec(hex_value, 3, vec![])
}

fn binary_hex_to_value(binary_hex: &Vec<u8>) -> u8{
    8 * binary_hex.get(0).unwrap() + 4 * binary_hex.get(1).unwrap() + 2 * binary_hex.get(2).unwrap() + binary_hex.get(3).unwrap()
}

fn main() {
    println!("Hello, world!");

    let ab_pairs: Vec<Vec<(u8, u8)>> = (0..=15).into_iter().map(|a| {

        let res: Vec<(u8, u8)> = (0..=15).into_iter().map(|b| {
            (a, b)
        }).collect();
        res
    }).collect();
    // println!("ab_pairs: {:?}", ab_pairs);

    let x_array: Vec<Vec<u8>> = (0..=15).into_iter()
        .map(|x| create_hex_binary(x))
        .collect();
    println!("x_array: {:?}", x_array);

    let sbox = get_sbox();
    let y_array: Vec<Vec<u8>> = x_array.clone().into_iter().map(|x| {
        let x_value = binary_hex_to_value(&x);
        let y: u8 = sbox.get(x_value as usize).unwrap().to_owned();
        create_hex_binary(y)
    }).collect();
    println!("y_array: {:?}", y_array);

    let res_table: Vec<Vec<u8>> = ab_pairs.into_iter().map(|row| {
        let res_row: Vec<u8> = row.clone().into_iter().map(|(a, b)| {
            let a_binary = create_hex_binary(a);
            let b_binary = create_hex_binary(b);

            let mut x_indexes_to_xor: Vec<usize> = vec![];
            let mut y_indexes_to_xor: Vec<usize> = vec![];
            if a_binary[0] == 1 {x_indexes_to_xor.push(0)}
            if a_binary[1] == 1 {x_indexes_to_xor.push(1)}
            if a_binary[2] == 1 {x_indexes_to_xor.push(2)}
            if a_binary[3] == 1 {x_indexes_to_xor.push(3)}

            if b_binary[0] == 1 {y_indexes_to_xor.push(0)}
            if b_binary[1] == 1 {y_indexes_to_xor.push(1)}
            if b_binary[2] == 1 {y_indexes_to_xor.push(2)}
            if b_binary[3] == 1 {y_indexes_to_xor.push(3)}
            let x_bits_to_xor_list: Vec<Option<u8>> = x_array.clone().into_iter().map(|x| {
                let x_bits_to_xor: Vec<u8> = x_indexes_to_xor.clone().into_iter()
                    .map(|i| x[i]).collect();
                // println!("x_bits_to_xor: {:?}", x_bits_to_xor);
                let res = x_bits_to_xor.into_iter().reduce(|acc, x| acc ^ x);
                res

            }).collect();
            // println!("a = {a}, b = {b}");
            // println!("x_bits_to_xor_list: {:?}", x_bits_to_xor_list);
            let y_bits_to_xor_list: Vec<Option<u8>> = y_array.clone().into_iter().map(|y| {
                let y_bits_to_xor: Vec<u8> = y_indexes_to_xor.clone().into_iter()
                    .map(|i| y[i]).collect();
                // y_bits_to_xor
                let res = y_bits_to_xor.into_iter().reduce(|acc, x| acc ^ x);
                res
            }).collect();
            // println!("y_bits_to_xor_list: {:?}", y_bits_to_xor_list);

            let zipped: Vec<(Option<u8>, Option<u8>)> = x_bits_to_xor_list
                .into_iter()
                .zip(y_bits_to_xor_list.into_iter()).collect();
            let values_to_count: Vec<u8> = zipped.into_iter()
                .map(| (x, y)| {
                    if x.is_some() && y.is_some() {
                        return x.unwrap() ^ y.unwrap()
                    }
                    if x.is_some() {
                        return x.unwrap();
                    }
                    if y.is_some(){
                        return y.unwrap();
                    }
                    15

                }).collect();
            // println!("values_to_count: {:?}", values_to_count);
            let count: u8 = values_to_count.into_iter()
                .filter(|x| *x == 0).collect::<Vec<u8>>().len() as u8;

            let count_to_return = if a == 0 && b == 0 {16}
            else if a == 0 || b == 0 {
                8
            }else {count};
            // println!("count = {}", count_to_return);
            // println!();

            count_to_return
        }).collect();
        res_row
    }).collect();

    res_table.clone().into_iter().for_each(|row| {
        row.into_iter().for_each(|value| {
            print!("{:4.3}", value);
        });
        println!()
    });

    println!("epsilon table:");
    let epsilon_table = res_table.clone().into_iter().map(|row| {
        row.into_iter().map(|count| {
            (count as i32 - 8) as f32 / 16.0
        }).collect::<Vec<f32>>()
    }).collect::<Vec<Vec<f32>>>();

    epsilon_table.into_iter().for_each(|row| {
        row.into_iter().for_each(|value| {
            print!("{:8.3}", value);
        });
        println!()
    });

    println!("bye");
}
