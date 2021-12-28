use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error as IoError};
use std::net::IpAddr;

#[derive(Debug)]
struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone, PartialEq)]
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

fn read_file(filename: &str) -> Result<Vec<IpRange>, IoError> {
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

fn line_to_ip_range(line: &str) -> Result<IpRange, Box<dyn Error>> {
    match line
        .split('\t')
        .map(|s| s.trim())
        .collect::<Vec<_>>()
        .as_slice()
    {
        [start_ip, end_ip, number, country, description] => Ok(IpRange {
            start_ip: start_ip.parse()?,
            end_ip: end_ip.parse()?,
            number: number.parse()?,
            country: country.to_string(),
            description: description.to_string(),
        }),
        _ => Err(Box::new(ParseError {
            message: "Could not parse line".to_string(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_ip_range() {
        let line = "10.20.30.40\t10.20.30.50\t12345\tVN\tDescription Goes Here";
        let range = line_to_ip_range(line).unwrap();
        assert_eq!(range.start_ip, "10.20.30.40".parse::<IpAddr>().unwrap());
        assert_eq!(range.end_ip, "10.20.30.50".parse::<IpAddr>().unwrap());
        assert_eq!(range.number, "12345".parse::<u32>().unwrap());
        assert_eq!(range.country, "VN".to_string());
        assert_eq!(range.description, "Description Goes Here".to_string());
    }

    #[test]
    fn test_line_to_ip_range_when_parse_invalid_ip_address() {
        let line = "10.20.30.400\t10.20.30.50\t12345\tVN\tDescription Goes Here";
        assert_eq!(
            line_to_ip_range(line)
                .unwrap_err()
                .is::<std::net::AddrParseError>(),
            true
        );
    }

    #[test]
    fn test_line_to_ip_range_when_parse_invalid_number() {
        let line = "10.20.30.40\t10.20.30.50\tfoo\tVN\tDescription Goes Here";
        assert_eq!(
            line_to_ip_range(line)
                .unwrap_err()
                .is::<std::num::ParseIntError>(),
            true
        );
    }

    #[test]
    fn test_line_to_ip_range_when_parse_missing_fields() {
        let line = "10.20.30.40\t10.20.30.50\t12345";
        assert_eq!(line_to_ip_range(line).unwrap_err().is::<ParseError>(), true);
    }
}
