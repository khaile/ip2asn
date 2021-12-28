use std::convert::Infallible as ConvertError;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct IpRange {
    pub start_ip: IpAddr,
    pub end_ip: IpAddr,
    pub number: u32,
    pub country: String,
    pub description: String,
}

pub fn load_ranges(ip: IpAddr) -> Vec<IpRange> {
    if ip.is_ipv4() {
        read_file("ip2asn-v4.tsv").expect("Could not read ASN V4 file")
    } else {
        read_file("ip2asn-v6.tsv").expect("Could not read ASN V6 file")
    }
}

pub fn find_asn(ranges: &[IpRange], ip: IpAddr) -> Option<IpRange> {
    let mut start = 0;
    let mut end = ranges.len() - 1;

    while start <= end {
        let mid = (start + end) / 2;
        if ip >= ranges[mid].start_ip && ip <= ranges[mid].end_ip {
            return Some(ranges[mid].clone());
        } else if ip < ranges[mid].start_ip {
            end = mid - 1
        } else {
            start = mid + 1
        }
    }

    None
}

fn read_file(filename: &str) -> Result<Vec<IpRange>, Error> {
    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);
    let mut lines: Vec<IpRange> = Vec::new();

    for line in buf_reader.lines().flatten() {
        match line_to_ip_range(&line) {
            Ok(range) => lines.push(range),
            Err(e) => println!("{}", e),
        }
    }

    Ok(lines)
}

fn line_to_ip_range(line: &str) -> Result<IpRange, ConvertError> {
    let mut parts = line.split('\t').map(|s| s.trim());

    Ok(IpRange {
        start_ip: parts.next().unwrap().parse().unwrap(),
        end_ip: parts.next().unwrap().parse().unwrap(),
        number: parts.next().unwrap().parse().unwrap(),
        country: parts.next().unwrap().to_string(),
        description: parts.next().unwrap().to_string(),
    })
}
