use std::env;
use std::net::IpAddr;
mod ip_finder;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).unwrap();

    let ip = input.parse::<IpAddr>().expect("Invalid IP address");
    let ranges = ip_finder::load_ranges(ip);
    let range = ip_finder::find_asn(&ranges, ip).expect("Could not find ASN");

    println!(
        "{}",
        format!(
            "{} found!\nnumber({})|country({})|description({})",
            ip, range.number, range.country, range.description
        )
    );
}
