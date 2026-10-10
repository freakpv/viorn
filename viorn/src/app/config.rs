use serde::Deserialize;
use std::{error::Error, fmt};

#[derive(Debug, Deserialize)]
pub(super) struct Config {
    filter: Filter,
    _mgmt_server: MgmtServer,
}

#[derive(Debug, Deserialize)]
pub(super) struct Filter {
    _dev: String,
    _queues: Vec<u32>,
    tcp_ports: Vec<PortRange>,
    udp_ports: Vec<PortRange>,
}

#[derive(Debug, Deserialize)]
pub(super) struct MgmtServer {
    _addr: std::net::IpAddr,
    _port: u16,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Open ended port range
#[derive(Clone, Copy, Deserialize)]
pub(super) struct PortRange {
    beg: u16,
    end: u16,
}

impl PortRange {
    pub fn contains(&self, port: u16) -> bool {
        return self.beg <= port && port < self.end;
    }
}

impl fmt::Display for PortRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}-{})", self.beg, self.end)
    }
}

impl fmt::Debug for PortRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum ConfigError {
    InvalidPortRange(PortRange),
    OverlappingPortRanges { first: PortRange, next: PortRange },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidPortRange(rng) => write!(f, "invalid port range: {}", rng),
            ConfigError::OverlappingPortRanges { first, next } => {
                write!(f, "overlapping port ranges: first:{} next:{}", first, next)
            }
        }
    }
}

impl Error for ConfigError {}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Config {
    pub(super) fn load(config_data: &str) -> anyhow::Result<Config> {
        let mut config: Config = toml::from_str(&config_data)?;

        config
            .filter
            .tcp_ports
            .sort_by(|lhs, rhs| lhs.beg.cmp(&rhs.beg));
        config
            .filter
            .udp_ports
            .sort_by(|lhs, rhs| lhs.beg.cmp(&rhs.beg));

        // Port range [0-0) means match all ports and thus there shouldn't be any other ranges
        // specified in this case.
        let check_match_all = |ports: &[PortRange]| -> anyhow::Result<()> {
            if ports.len() <= 1 {
                return Ok(());
            }
            if let PortRange { beg: 0, end: 0 } = ports[0] {
                return Err(ConfigError::OverlappingPortRanges {
                    first: ports[0],
                    next: ports[1],
                }
                .into());
            }
            Ok(())
        };
        check_match_all(&config.filter.tcp_ports)?;
        check_match_all(&config.filter.udp_ports)?;

        // Check the port ranges for correctness and overlapping
        let check_ranges = |ports: &[PortRange]| -> anyhow::Result<()> {
            if ports.is_empty() {
                return Ok(());
            }
            let is_valid = |rng: PortRange| rng.beg < rng.end;
            if ports.len() == 1 && !is_valid(ports[0]) {
                return Err(ConfigError::InvalidPortRange(ports[0]).into());
            }
            // There are repetitions in the checking logic but it's not performance critical and
            // the port ranges are checked only once and are not expected to be that many.
            for pair in ports.windows(2) {
                let [first, next] = pair else { unreachable!() };
                if !is_valid(*first) {
                    return Err(ConfigError::InvalidPortRange(*first).into());
                }
                if !is_valid(*next) {
                    return Err(ConfigError::InvalidPortRange(*next).into());
                }
                if first.contains(next.beg) {
                    return Err(ConfigError::OverlappingPortRanges {
                        first: *first,
                        next: *next,
                    }
                    .into());
                }
            }
            Ok(())
        };
        check_ranges(&config.filter.tcp_ports)?;
        check_ranges(&config.filter.udp_ports)?;

        return Ok(config);
    }
}
