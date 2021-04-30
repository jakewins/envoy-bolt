use log::{trace, info};

#[derive(Debug, Clone, Copy)]
pub enum ParserStatus {
    // Emitted by parser when it reaches the end of the current frame being parsed and it needs more input
    Suspend,
}

type Result<T> = std::result::Result<T, ParserStatus>;


#[derive(Debug, Clone)]
enum ParserState {
    HandshakeAwaitMagic{ position: u8 },
    HandshakeVersions{ position: u8, versions: [u8; 4 * 4] },

    UnknownProtocol,
}

// Parses traffic from downstream client, eg client -> server traffic
#[derive(Debug, Clone)]
pub struct DownstreamParser {
    state: ParserState,
}

impl DownstreamParser {

    pub fn new() -> Self {
        DownstreamParser {
            state: ParserState::HandshakeAwaitMagic{position: 0},
        }
    }

    pub fn parse(&mut self, data: &[u8]) -> Result<()> {
        let mut c = Cursor::new(data);
        info!("  PAYLOAD: {:?}", data);
        loop {
            info!("  STATE: {:?}", self.state);
            match &self.state {
                ParserState::HandshakeAwaitMagic{position} => {
                    let b = c.next()?;
                    info!("  b: {:?} / {:?}", b, self.state);
                    match (position, b) {
                        (0, 0x60) => {
                            self.state = ParserState::HandshakeAwaitMagic{position: 1};
                            continue
                        },
                        (1, 0x60) => {
                            self.state = ParserState::HandshakeAwaitMagic{position: 2};
                            continue
                        }
                        (2, 0xB0) => {
                            self.state = ParserState::HandshakeAwaitMagic{position: 3};
                            continue
                        }
                        (3, 0x17) => {
                            self.state = ParserState::HandshakeVersions{position: 0};
                            continue
                        }
                        _ => {
                            self.state = ParserState::UnknownProtocol;
                            info!("invalid magic preamble, moving to unknown protocol mode");
                            return Ok(())
                        }
                    }
                }
                UnknownProtocol => {
                    return Ok(())
                }
            }
        }
    }

}

struct Cursor<'i> {
    data: &'i [u8],
    position: usize,
}

impl<'i> Cursor<'i> {
    fn new(data: &[u8]) -> Cursor {
        Cursor {
            data,
            position: 0,
        }
    }

    fn next(&mut self) -> Result<u8> {
        if self.data.len() <= self.position + 1 {
            return Err(ParserStatus::Suspend);
        }
        self.position += 1;
        return Ok(self.data[self.position])
    }
}