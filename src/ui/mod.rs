mod input;

use self::input::{Event, Events};
use cpu;

use std::error::Error;
use std::io;
use std::ops::Range;

use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::{Frame, Terminal};

type Backend = TermionBackend<RawTerminal<io::Stdout>>;

struct DisassemblyViewState {
    // Disassembly output
    dasm: Vec<String>,
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
    /// Crreate a new application using an existing CPU.
    pub fn new(cpu: cpu::CPU) -> App {
        App {
            size: Rect::default(),
            dvs: DisassemblyViewState {
                dasm: (0..0x2000)
                    .map(|pc| {
                        let mut s = cpu.disassemble(pc).unwrap();
                        s.push('\n');
                        s
                    })
                    .collect::<Vec<_>>(),
                range: 0..0,
            },
            cpu,
        }
    }

    /// Run the application main loop and execute the program.
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
                    .constraints([Constraint::Length(44), Constraint::Min(80)].as_ref())
                    .split(self.size);

                self.draw_disassembly(&mut f, chunks[0]);
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

    /// Draw the disassembly panel on the left of the GUI.
    /// The disassembly will also highlight the current instruction.
    fn draw_disassembly(&mut self, f: &mut Frame<Backend>, area: Rect) {
        let inner = area.inner(2);
        let pc = self.cpu.pc;
        let rng = {
            let rng = &mut self.dvs.range;

            // Recompute disassembly range based on PC and current window size
            let lb = pc.checked_sub(5).unwrap_or(0);
            let ub = (pc + 5).min(0x2000);
            let h = inner.height;

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
                let asm = &self.dvs.dasm[line as usize];

                if line != pc {
                    Text::raw(asm)
                } else {
                    Text::styled(
                        asm,
                        Style::default()
                            .bg(Color::LightCyan)
                            .fg(Color::Black)
                            .modifier(Modifier::Bold),
                    )
                }
            }).collect::<Vec<_>>();

        // TODO: PR to add margins to Layout elements?
        Block::default()
            .borders(Borders::ALL)
            .title("Disassembly")
            .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold))
            .render(f, area);

        Paragraph::new(text.iter()).wrap(true).render(f, inner);
    }

    /// Draw the right panel of the GUI, which includes the CPU state and a memory view.
    fn draw_right_panel(&mut self, f: &mut Frame<Backend>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(13), Constraint::Min(0)].as_ref())
            .split(area);

        let top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(55), Constraint::Min(1)].as_ref())
            .split(chunks[0]);

        self.draw_cpu_state(f, top[0]);
        self.draw_stats(f, top[1]);
        self.draw_memory_map(f, chunks[1]);
    }

    /// Draw the CPU state panel, which includes current register and flag values.
    fn draw_cpu_state(&mut self, f: &mut Frame<Backend>, area: Rect) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(7), Constraint::Min(2)].as_ref())
            .split(area.inner(2));

        let reg_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(22), Constraint::Length(32)].as_ref())
            .split(rows[0]);

        let flag_area = Rect {
            y: rows[1].y + 1,
            ..rows[1]
        };

        // Format 8-bit registers
        let text_r8 = vec![
            format_r8("A", self.cpu.regs.A()),
            format_r8("B", self.cpu.regs.B()),
            format_r8("C", self.cpu.regs.C()),
            format_r8("D", self.cpu.regs.D()),
            format_r8("E", self.cpu.regs.E()),
            format_r8("H", self.cpu.regs.H()),
            format_r8("L", self.cpu.regs.L()),
        ].into_iter()
            .flat_map(|mut v| {
                v.push(Text::raw("\n"));
                v
            })
            .collect::<Vec<_>>();

        // Format 16-bit registers
        let text_r16 = vec![
            format_r16("BC", self.cpu.regs.BC()),
            format_r16("DE", self.cpu.regs.DE()),
            format_r16("HL", self.cpu.regs.HL()),
            vec![Text::raw("")], // separator
            format_r16("SP", self.cpu.sp),
            vec![Text::raw("")], // separator
            format_r16("PC", self.cpu.pc),
        ].into_iter()
            .flat_map(|mut v| {
                v.push(Text::raw("\n"));
                v
            })
            .collect::<Vec<_>>();

        // Format CPU flags
        let text_flags = format_flags(&self.cpu.flags);

        // Draw border around widget
        Block::default()
            .borders(Borders::ALL)
            .title("CPU")
            .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold))
            .render(f, area);

        // Draw registers
        Paragraph::new(text_r8.iter()).render(f, reg_area[0]);
        Paragraph::new(text_r16.iter()).render(f, reg_area[1]);
        Paragraph::new(text_flags.iter())
            .alignment(Alignment::Center)
            .render(f, flag_area);
    }

    // Draw some statistics about the emulator
    fn draw_stats(&mut self, f: &mut Frame<Backend>, area: Rect) {
        Block::default()
            .borders(Borders::ALL)
            .title("Stats")
            .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold))
            .render(f, area);
    }

    /// Draw the memory view, centered around the recently accessed memory locations.
    fn draw_memory_map(&mut self, f: &mut Frame<Backend>, area: Rect) {
        let rows = area.inner(2).height as usize;
        let cols = f32::powf(
            2.0,
            f32::log2(((area.inner(2).width - 9) / 4) as f32).floor(),
        ) as usize;

        let text = self
            .cpu
            .mem
            .wram
            .chunks(cols)
            .take(rows)
            .enumerate()
            .map(|(i, line)| {
                vec![
                    // Address
                    Text::raw(format!("{:04x}  ", i * cols)),
                    // Hex represention of single line
                    Text::raw(
                        line.iter()
                            .map(|&b| format!("{:02x} ", b))
                            .collect::<Vec<_>>()
                            .join(""),
                    ),
                    // ASCII decoding
                    Text::raw(" |"),
                    Text::raw(unsafe {
                        String::from_utf8_unchecked(
                            line.iter().map(|&b| format_hex(b)).collect::<Vec<_>>(),
                        )
                    }),
                    Text::raw("|\n"),
                ]
            })
            .flatten()
            .collect::<Vec<_>>();

        Block::default()
            .borders(Borders::ALL)
            .title("Memory")
            .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold))
            .render(f, area);

        Paragraph::new(text.iter()).render(f, area.inner(2));
    }
}

/// Create the formatted text for an 8-bit register.
fn format_r8(name: &str, val: u8) -> Vec<Text> {
    let style = Style::default().fg(Color::Magenta).modifier(Modifier::Bold);

    vec![
        Text::styled(format!("{:<2}: ", name), style),
        Text::raw(format!("b{:08b} [h{:02x}]", val, val)),
    ]
}

/// Create the formatted text for a 16-bit register.
fn format_r16(name: &str, val: u16) -> Vec<Text> {
    let style = Style::default().fg(Color::Magenta).modifier(Modifier::Bold);

    vec![
        Text::styled(format!("{:<2}: ", name), style),
        Text::raw(format!("b{:016b} [h{:04x}]", val, val)),
    ]
}

/// Create the formatted text for the CPU flags
fn format_flags(flags: &cpu::Flags) -> Vec<Text> {
    let title_style = Style::default().fg(Color::Magenta).modifier(Modifier::Bold);
    let off_style = Style::default();
    let on_style = Style::default()
        .fg(Color::Black)
        .bg(Color::LightBlue)
        .modifier(Modifier::Bold);

    let mut text = Vec::with_capacity(6);

    text.push(Text::styled("PSW: ", title_style));
    text.extend(
        [
            (" S ", flags.S),
            (" Z ", flags.Z),
            (" P ", flags.P),
            (" AC ", flags.AC),
            (" CY ", flags.CY),
        ].into_iter()
            .map(|&(fmt, val)| Text::styled(fmt, if val { on_style } else { off_style })),
    );

    text
}

/// Convert a byte in its ASCII representation if it is a graphical character
/// or a space, otherwise return a single '.' char.
fn format_hex(c: u8) -> u8 {
    if c.is_ascii_graphic() || c == 0x20 {
        c
    } else {
        b'.'
    }
}
