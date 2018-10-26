mod input;

use self::input::{Event, Events};
use cpu;

use std::error::Error;
use std::io;
use std::ops::Range;

use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::{Frame, Terminal};

type Backend = TermionBackend<RawTerminal<io::Stdout>>;

struct DisassemblyViewState {
    // Disassembly lines now in scope
    range: Range<u16>,
}

pub struct App {
    // Size of the view
    size: Rect,
    dvs: DisassemblyViewState,

    // Emulator state
    cpu: cpu::CPU,
}

impl App {
    pub fn new(cpu: cpu::CPU) -> App {
        App {
            size: Rect::default(),
            dvs: DisassemblyViewState { range: 0..0 },
            cpu,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<Error>> {
        // Create terminal backend
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        // Event handling
        let events = Events::new();

        loop {
            // Handle terminal resizing
            let size = terminal.size()?;
            if self.size != size {
                terminal.resize(size)?;
                self.size = size;
            }

            terminal.draw(|mut f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(self.size);

                self.draw_left_panel(&mut f, chunks[0]);
                self.draw_right_panel(&mut f, chunks[1]);
            })?;

            match events.next()? {
                Event::Input(key) => if key == Key::Char('q') {
                    break;
                },
                _ => {}
            }
        }

        Ok(())
    }

    fn draw_left_panel(&mut self, f: &mut Frame<Backend>, area: Rect) {
        let pc = self.cpu.pc;
        let rng = {
            let rng = &mut self.dvs.range;

            // Recompute disassembly range based on PC and current window size
            let lb = pc.checked_sub(5).unwrap_or(0);
            let ub = (pc + 5).min(0x2000);
            let h = area.inner(1).height;

            // Adjust visible range so that PC always lies inside, with a bit of context too.
            if (rng.len() as u16) < h {
                // We can expand the range. To do this, we move the end forward.
                rng.end = rng.start + h;
            } else if (rng.len() as u16) > h {
                // We need to shrink the range. Again, move the start forward,
                // but we then need to check if PC is still in range and translate it if not.
                rng.start = rng.end - h;
                if !rng.contains(&pc) {
                    rng.start = lb;
                    rng.end = rng.start + h;
                }
            } else {
                // No resize is needed. We just need to adjust the range to contain PC.
                if rng.start > lb {
                    rng.start = lb;
                    rng.end = rng.start + h;
                } else if rng.end < ub {
                    rng.end = ub;
                    rng.start = rng.end - h;
                }
            }

            // Return a copy of the range iterator
            rng.clone()
        };

        // Create disassembly
        let text =
            rng.map(|line| {
                let mut disasm = self.cpu.disassemble(line).unwrap();
                disasm.push('\n');

                if line != pc {
                    Text::raw(disasm)
                } else {
                    Text::styled(
                        disasm,
                        Style::default().bg(Color::LightCyan).fg(Color::Black),
                    )
                }
            }).collect::<Vec<_>>();

        // TODO: is there any way to avoid the ::collect() above?
        Paragraph::new(text.iter())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Footer")
                    .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold)),
            )
            .wrap(true)
            .render(f, area);
    }

    fn draw_right_panel(&mut self, f: &mut Frame<Backend>, area: Rect) {}
}
