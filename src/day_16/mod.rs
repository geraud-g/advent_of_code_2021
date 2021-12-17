use advent_of_code_2021::utils::inputs::get_file;

pub fn day_16() {
    let bits = get_input();

    let packets = get_packets(&bits);
    let solution_a = get_version_sum(&packets);
    println!("Solution for Day 16, part A is: {}", solution_a);

    let solution_b = eval_packets(&packets[0]);
    println!("Solution for Day 16, part B is: {}", solution_b);
}


#[derive(Debug, Default)]
struct Packet {
    version: usize,
    type_id: usize,
    value: usize,
    length: usize,
    sub_packets: Vec<Self>,
}


fn get_input() -> Vec<bool> {
    let mut binary_vec = vec![];
    for c in get_file("./src/day_16/input.txt").chars() {
        let u8_val = u8::from_str_radix(&c.to_string(), 16).unwrap();
        for bit in format!("{:04b}", u8_val).chars() {
            binary_vec.push(!'0'.eq(&bit));
        }
    }
    binary_vec
}


fn get_packets(global_packet: &[bool]) -> Vec<Packet> {
    let mut packets = vec![];
    let mut current_idx = 0;
    while let Some(packet) = get_next_packet(&global_packet[current_idx..]) {
        current_idx += packet.length;
        packets.push(packet);
    }
    packets
}


fn get_next_packet(global_packet: &[bool]) -> Option<Packet> {
    if global_packet.iter().all(|p| !(*p)) {
        return None;
    }
    let version = get_val_from_bin(&global_packet[..3]);
    let type_id = get_val_from_bin(&global_packet[3..6]);

    if type_id == 4 {
        return Some(get_packet_with_literal(&global_packet[6..], version, type_id));
    }
    let length_type_id = get_val_from_bin(&global_packet[6..7]);
    if length_type_id == 0 {
        Some(get_packet_by_length(&global_packet[7..], version, type_id))
    } else if length_type_id == 1 {
        Some(get_packet_by_numbers(&global_packet[7..], version, type_id))
    } else {
        panic!("Unknown packet ID: {}", type_id)
    }
}


fn get_packet_with_literal(global_packet: &[bool], version: usize, type_id: usize) -> Packet {
    let mut packet = Packet {
        version,
        type_id,
        value: 0,
        length: 6,
        sub_packets: vec![],
    };
    for chunk in global_packet.chunks(5) {
        let is_last_packet = !chunk[0];
        let chunk_val = get_val_from_bin(&chunk[1..]);
        packet.length += 5;
        packet.value <<= 4;
        packet.value += chunk_val;
        if is_last_packet {
            break;
        }
    }
    packet
}

/// If the `length type ID` of an operator is `1`,
/// then the next 11 bits are a number
/// that represents the number of sub-packets immediately contained by this packet.
fn get_packet_by_numbers(global_packet: &[bool], version: usize, type_id: usize) -> Packet {
    let mut packet = Packet {
        version,
        type_id,
        value: 0,
        length: 7,
        sub_packets: vec![],
    };
    let packet_nbr = get_val_from_bin(&global_packet[..11]);
    packet.length += 11;
    for _ in 0..packet_nbr {
        if let Some(new_packet) = get_next_packet(&global_packet[(packet.length - 7)..]) {
            packet.length += new_packet.length;
            packet.sub_packets.push(new_packet)
        }
    }
    packet
}


/// If the `length type ID` is `0`,
/// then the next 15 bits are a number
/// that represents the total length in bits of the sub-packets contained by this packet.
fn get_packet_by_length(global_packet: &[bool], version: usize, type_id: usize) -> Packet {
    let mut packet = Packet {
        version,
        type_id,
        value: 0,
        length: 7,
        sub_packets: vec![],
    };
    let total_length = get_val_from_bin(&global_packet[..15]);
    packet.length += 15 + total_length;
    packet.sub_packets.extend(get_packets(&global_packet[15..15 + total_length]));
    packet
}


fn get_val_from_bin(packet: &[bool]) -> usize {
    let mut value = 0;
    for val in packet.iter() {
        if *val {
            value |= 1;
        }
        value <<= 1;
    }
    value >>= 1;
    value
}


fn get_version_sum(packets: &[Packet]) -> usize {
    packets.iter().map(|packet| get_count_rec(packet)).sum()
}

fn get_count_rec(packet: &Packet) -> usize {
    packet.version + packet.sub_packets.iter().map(|c| get_count_rec(c)).sum::<usize>()
}

fn eval_packets(packet: &Packet) -> usize {
    match packet.type_id {
        // sum packets - their value is the sum of the values of their sub-packets.
        // If they only have a single sub-packet, their value is the value of the sub-packet.
        0 => packet.sub_packets.iter().map(|s| eval_packets(s)).sum(),
        // product packets - their value is the result of multiplying together the values of their sub-packets.
        // If they only have a single sub-packet, their value is the value of the sub-packet.
        1 => packet.sub_packets.iter().map(|s| eval_packets(s)).product(),
        //minimum packets - their value is the minimum of the values of their sub-packets.
        2 => packet.sub_packets.iter().map(|s| eval_packets(s)).min().unwrap(),
        // maximum packets - their value is the maximum of the values of their sub-packets.
        3 => packet.sub_packets.iter().map(|s| eval_packets(s)).max().unwrap(),
        // literal packets - they only carry one value
        4 => packet.value,
        // greater than packets - their value is 1 if the value of the first sub-packet is greater than the value of the second sub-packet;
        // otherwise, their value is 0. These packets always have exactly two sub-packets.
        5 => if eval_packets(&packet.sub_packets[0]) > eval_packets(&packet.sub_packets[1]) { 1 } else { 0 },
        // less than packets - their value is 1 if the value of the first sub-packet is less than the value of the second sub-packet;
        // otherwise, their value is 0. These packets always have exactly two sub-packets.
        6 => if eval_packets(&packet.sub_packets[0]) < eval_packets(&packet.sub_packets[1]) { 1 } else { 0 },
        // equal to packets - their value is 1 if the value of the first sub-packet is equal to the value of the second sub-packet;
        // otherwise, their value is 0. These packets always have exactly two sub-packets.
        7 => if eval_packets(&packet.sub_packets[0]) == eval_packets(&packet.sub_packets[1]) { 1 } else { 0 },
        _ => panic!("Unknown type ID: {}", packet.type_id)
    }
}
