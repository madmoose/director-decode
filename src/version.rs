use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Version {
    version: u16,
    human_version: u16,
}

impl Version {
    pub fn new(version: u16) -> Self {
        static VERSIONS: [(u16, u16); 13] = [
            (0x404, 300),
            (0x405, 310),
            (0x45B, 400),
            (0x45D, 404),
            (0x4B1, 500),
            (0x4C2, 600),
            (0x4C8, 700),
            (0x582, 800),
            (0x6A4, 850),
            (0x73B, 1000),
            (0x781, 1100),
            (0x782, 1150),
            (0x79F, 1200),
        ];

        let mut human_version = VERSIONS[0].1;
        for v in VERSIONS {
            if version >= v.0 {
                human_version = v.1;
            }
        }

        Version {
            version,
            human_version,
        }
    }

    pub fn major(self) -> u16 {
        self.human_version / 100
    }

    pub fn minor(self) -> u16 {
        self.human_version % 100
    }
}

impl Default for Version {
    fn default() -> Self {
        Version::new(300)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:2}.{:02} ({:#x})",
            self.major(),
            self.minor(),
            self.version
        )
    }
}
