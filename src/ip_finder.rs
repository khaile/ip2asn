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
        lines.push(line_to_ip_range(&line));
    }

    Ok(lines)
}

fn line_to_ip_range(line: &str) -> IpRange {
    let mut parts = line.split('\t');
    let start_ip = parts.next().unwrap().trim().parse().unwrap();
    let end_ip = parts.next().unwrap().trim().parse().unwrap();
    let number = parts.next().unwrap().trim().parse().unwrap();
    let country = parts.next().unwrap().trim().to_owned();
    let description = parts.next().unwrap().trim().to_owned();

    IpRange {
        start_ip,
        end_ip,
        number,
        country,
        description,
    }
}
