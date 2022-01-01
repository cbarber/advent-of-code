use std::str::FromStr;

use bitvec::{field::BitField, prelude::*};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::one_of,
    combinator::{map, map_res},
    multi::{length_count, length_value, many0, many1, many_m_n},
    sequence::{preceded, tuple},
    IResult,
};
use nom_bitvec::BSlice;

const INPUT: &str = include_str!("input");

fn literal_from_nibbles(head: Vec<BSlice<Msb0, u8>>, tail: BSlice<Msb0, u8>) -> u64 {
    let mut bv = BitVec::<Msb0, u8>::new();
    for nibble in head {
        bv.extend_from_bitslice(nibble.0);
    }
    bv.extend_from_bitslice(tail.0);
    bv.load_be::<u64>()
}

fn parse_literal(input: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, u64> {
    map(
        tuple((
            many0(preceded(tag(BSlice(bits![1])), take(4usize))),
            preceded(tag(BSlice(bits![0])), take(4usize)),
        )),
        |(head, tail)| literal_from_nibbles(head, tail),
    )(input)
}

fn parse_literal_packet(input: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, InnerPacket> {
    map(
        tuple((tag(BSlice(bits![1, 0, 0])), parse_literal)),
        |(_, literal)| InnerPacket::Literal(literal),
    )(input)
}

fn parse_operator_packet_by_length(
    input: BSlice<Msb0, u8>,
) -> IResult<BSlice<Msb0, u8>, Vec<Packet>> {
    preceded(
        tag(BSlice(bits![0])),
        length_value(
            map(take(15usize), |b: BSlice<Msb0, u8>| b.0.load_be::<usize>()),
            many0(parse_packet),
        ),
    )(input)
}

fn parse_operator_packet_by_count(
    input: BSlice<Msb0, u8>,
) -> IResult<BSlice<Msb0, u8>, Vec<Packet>> {
    preceded(
        tag(BSlice(bits![1])),
        length_count(
            map(take(11usize), |b: BSlice<Msb0, u8>| b.0.load_be::<usize>()),
            parse_packet,
        ),
    )(input)
}

fn parse_operator_packet(input: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, InnerPacket> {
    map(
        tuple((
            take(3usize),
            alt((
                parse_operator_packet_by_count,
                parse_operator_packet_by_length,
            )),
        )),
        |(operator, packets)| InnerPacket::Operator(operator.0.load_be::<u8>().into(), packets),
    )(input)
}

fn parse_packet(input: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, Packet> {
    map(
        tuple((
            take(3usize),
            alt((parse_literal_packet, parse_operator_packet)),
        )),
        |(version, packet)| Packet {
            version: version.0.load_be::<u8>(),
            packet,
        },
    )(input)
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    packet: InnerPacket,
}

impl Packet {
    fn from_hex(pair: Vec<char>) -> Result<u8, std::num::ParseIntError> {
        let pair = pair.iter().collect::<String>();
        let mut res = u8::from_str_radix(&pair, 16)?;
        if pair.len() == 1 {
            res = res << 4;
        }
        Ok(res)
    }

    fn parse_hex(input: &str) -> IResult<&str, Vec<u8>> {
        many1(map_res(
            many_m_n(1, 2, one_of("0123456789ABCDEF")),
            Self::from_hex,
        ))(input)
    }

    fn version_sum(&self) -> u32 {
        self.version as u32
            + match self.packet {
                InnerPacket::Literal(_) => 0,
                InnerPacket::Operator(_, ref packets) => {
                    packets.iter().map(|p| p.version_sum()).sum()
                }
            }
    }
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, bytes) = Packet::parse_hex(s).unwrap();
        let bits = bytes.view_bits::<Msb0>();
        let result = parse_packet(BSlice(bits));
        let (_, packet) = result.unwrap();
        Ok(packet)
    }
}

#[derive(Debug, PartialEq)]
enum InnerPacket {
    Literal(u64),
    Operator(Operation, Vec<Packet>),
}

#[derive(Debug, PartialEq)]
enum Operation {
    NoOp,
}

impl From<u8> for Operation {
    fn from(_value: u8) -> Self {
        Operation::NoOp
    }
}

fn main() {
    let packet = INPUT.parse::<Packet>().unwrap();
    println!("{}", packet.version_sum());
}

#[test]
fn test_literal_packet() {
    const TEST_INPUT: &str = "D2FE28";
    let packet = TEST_INPUT.parse::<Packet>().unwrap();
    assert_eq!(
        Packet {
            version: 6,
            packet: InnerPacket::Literal(2021u64),
        },
        packet
    )
}

#[test]
fn test_nested_operator_packet() {
    const TEST_INPUT: &str = "8A004A801A8002F478";
    let packet = TEST_INPUT.parse::<Packet>().unwrap();
    assert_eq!(
        Packet {
            version: 4,
            packet: InnerPacket::Operator(
                Operation::NoOp,
                vec![Packet {
                    version: 1,
                    packet: InnerPacket::Operator(
                        Operation::NoOp,
                        vec![Packet {
                            version: 5,
                            packet: InnerPacket::Operator(
                                Operation::NoOp,
                                vec![Packet {
                                    version: 6,
                                    packet: InnerPacket::Literal(15),
                                }]
                            )
                        }]
                    )
                }]
            )
        },
        packet
    );
    assert_eq!(16, packet.version_sum())
}

#[test]
fn test_operator_packet_by_length() {
    const TEST_INPUT: &str = "38006F45291200";
    let packet = TEST_INPUT.parse::<Packet>().unwrap();
    assert_eq!(
        Packet {
            version: 1,
            packet: InnerPacket::Operator(
                Operation::NoOp,
                vec![
                    Packet {
                        version: 6,
                        packet: InnerPacket::Literal(10),
                    },
                    Packet {
                        version: 2,
                        packet: InnerPacket::Literal(20),
                    }
                ]
            )
        },
        packet
    );
}

#[test]
fn test_nested_operator_packet_2() {
    const TEST_INPUT: &str = "620080001611562C8802118E34";
    let packet = TEST_INPUT.parse::<Packet>().unwrap();
    assert_eq!(12, packet.version_sum())
}

#[test]
fn test_nested_operator_packet_3() {
    const TEST_INPUT: &str = "C0015000016115A2E0802F182340";
    let packet = TEST_INPUT.parse::<Packet>().unwrap();
    assert_eq!(23, packet.version_sum())
}

#[test]
fn test_nested_operator_packet_4() {
    const TEST_INPUT: &str = "A0016C880162017C3686B18A3D4780";
    let packet = TEST_INPUT.parse::<Packet>().unwrap();
    assert_eq!(31, packet.version_sum())
}
