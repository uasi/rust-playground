extern crate ncurses;

use std::ascii::AsciiExt;
use std::char;
use ncurses as nc;

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------

const CTRL_A: char = '\x01';
const CTRL_B: char = '\x02';
const CTRL_D: char = '\x04';
const CTRL_E: char = '\x05';
const CTRL_F: char = '\x06';
const CTRL_G: char = '\x07';
const CTRL_H: char = '\x08';
const CTRL_K: char = '\x0B';
const CTRL_W: char = '\x17';
const ESC: char    = '\x1B';
const DEL: char    = '\x7F';

fn getchar() -> char {
    char::from_u32(nc::getch() as u32).unwrap_or('\0')
}

fn adjust_screen_cursor_pos(win: &Window, ctx: &Context) {
    let mut beg_y = 0i32;
    let mut beg_x = 0i32;
    nc::getbegyx(win.win, &mut beg_y, &mut beg_x);
    let mut cur_y = 0i32;
    let mut cur_x = 0i32;
    nc::getyx(win.win, &mut cur_y, &mut cur_x);
    let x_offset = cur_x - (ctx.input.len() as i32);
    let mut scr_cur_y = beg_y + cur_y;
    let mut scr_cur_x = beg_x + x_offset + (ctx.cursor as i32);
    nc::setsyx(&mut scr_cur_y, &mut scr_cur_x);
}

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

struct Context {
    input: String,
    cursor: usize
}

// ---------------------------------------------------------------------------
// Window
// ---------------------------------------------------------------------------

struct Window {
    win: nc::WINDOW,
    win_impl: Box<WindowImpl + Send>
}

impl Window {
    fn new<WI>(win_impl: WI, lines: i32, cols: i32, y: i32, x: i32) -> Window
    where WI: WindowImpl + Send + 'static {
        let win = nc::newwin(lines, cols, y, x);
        nc::leaveok(win, true);
        Window {
            win: win,
            win_impl: Box::new(win_impl)
        }
    }

    fn draw(&self, ctx: &Context) {
        self.win_impl.draw(self.win, ctx);
    }

    fn clear(&self) {
        nc::wclear(self.win);
    }

    fn noutrefresh(&self) {
        nc::wnoutrefresh(self.win);
    }
}

// ---------------------------------------------------------------------------
// Window Impls
// ---------------------------------------------------------------------------

trait WindowImpl {
    fn draw(&self, win: nc::WINDOW, ctx: &Context);
}

struct MiniBufImpl;

impl WindowImpl for MiniBufImpl {
    fn draw(&self, win: nc::WINDOW, ctx: &Context) {
        nc::mvwaddstr(win, 0, 0, "input> ");
        nc::waddstr(win, ctx.input.as_slice());
    }
}

struct LeftPaneImpl;

impl WindowImpl for LeftPaneImpl {
    fn draw(&self, win: nc::WINDOW, ctx: &Context) {
        nc::mvwaddstr(win, 0, 0, "lower> ");
        nc::waddstr(win, ctx.input.as_slice().to_ascii_lowercase().as_slice());
    }
}

struct RightPaneImpl;

impl WindowImpl for RightPaneImpl {
    fn draw(&self, win: nc::WINDOW, ctx: &Context) {
        nc::mvwaddstr(win, 0, 0, "upper> ");
        nc::waddstr(win, ctx.input.as_slice().to_ascii_uppercase().as_slice());
    }
}

// ---------------------------------------------------------------------------
// Screen
// ---------------------------------------------------------------------------

struct Screen {
    ctx: Box<Context>,
    max_y: i32,
    max_x: i32,
    mini_buf: Window,
    left_pane: Window,
    right_pane: Window,
}

impl Screen {
    fn new() -> Self {
        let mut max_y = 0i32;
        let mut max_x = 0i32;
        nc::getmaxyx(nc::stdscr, &mut max_y, &mut max_x);

        let mini_buf_height = 1;

        let mini_buf = Window::new(MiniBufImpl, mini_buf_height, max_x, 0, 0);
        let left_pane = Window::new(LeftPaneImpl, max_y - mini_buf_height, max_x / 2, mini_buf_height, 0);
        let right_pane = Window::new(RightPaneImpl, max_y - mini_buf_height, max_x - (max_x / 2), mini_buf_height, max_x / 2);

        Screen {
            ctx: Box::new(Context { input: "".to_string(), cursor: 0 }),
            max_y: max_y,
            max_x: max_x,
            mini_buf: mini_buf,
            left_pane: left_pane,
            right_pane: right_pane,
        }
    }

    fn initialize() {
        nc::initscr();
        nc::cbreak();
        nc::noecho();
    }

    fn finalize() {
        nc::endwin();
    }

    fn begin_loop(&mut self) {
        loop {
            for win in [&self.mini_buf, &self.left_pane, &self.right_pane].iter() {
                win.clear();
                win.draw(&*self.ctx);
                win.noutrefresh();
            }
            adjust_screen_cursor_pos(&self.mini_buf, &*self.ctx);
            nc::doupdate();

            match getchar() {
                CTRL_A => {
                    self.ctx.cursor = 0;
                }
                CTRL_B => {
                    if self.ctx.cursor > 0 {
                        self.ctx.cursor -= 1;
                    }
                }
                CTRL_D => {
                    if self.ctx.cursor < self.ctx.input.len() {
                        let cursor = self.ctx.cursor;
                        self.ctx.input.remove(cursor);
                    }
                }
                CTRL_E => {
                    self.ctx.cursor = self.ctx.input.len();
                }
                CTRL_F => {
                    if self.ctx.cursor < self.ctx.input.len() {
                        self.ctx.cursor += 1;
                    }
                }
                CTRL_G => {
                    break;
                }
                CTRL_H | DEL => {
                    if self.ctx.cursor > 0 {
                        let cursor = self.ctx.cursor;
                        self.ctx.input.remove(cursor - 1);
                        self.ctx.cursor -= 1;
                    }
                }
                CTRL_K => {
                    let cursor = self.ctx.cursor;
                    self.ctx.input.truncate(cursor);
                }
                CTRL_W => {
                    let cursor = self.ctx.cursor;
                    let mut wordbreak_pos = {
                        let s = self.ctx.input.slice_chars(0, cursor);
                        s.rfind(' ').unwrap_or(0)
                    };
                    while wordbreak_pos > 0 && self.ctx.input.char_at(wordbreak_pos - 1) == ' ' {
                        wordbreak_pos -= 1;
                    }
                    self.ctx.input.truncate(wordbreak_pos);
                    self.ctx.cursor = wordbreak_pos;
                }
                ESC => {
                    break;
                }
                ch => {
                    let cursor = self.ctx.cursor;
                    self.ctx.input.insert(cursor, ch);
                    self.ctx.cursor += 1;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    Screen::initialize();

    nc::clear();
    nc::printw("Press any key to begin");
    nc::refresh();
    getchar();

    nc::clear();
    nc::refresh();

    let mut scr = Screen::new();
    scr.begin_loop();

    Screen::finalize();
}
