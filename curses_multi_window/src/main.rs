extern crate ncurses;

use std::ascii::AsciiExt;
use std::char;
use ncurses as nc;

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------

fn getchar() -> char {
    char::from_u32(nc::getch() as u32).unwrap_or('\0')
}

fn adjust_screen_cursor_pos(win: &Window) {
    let mut beg_y = 0i32;
    let mut beg_x = 0i32;
    nc::getbegyx(win.win, &mut beg_y, &mut beg_x);
    let mut cur_y = 0i32;
    let mut cur_x = 0i32;
    nc::getyx(win.win, &mut cur_y, &mut cur_x);
    let mut scr_cur_y = beg_y + cur_y;
    let mut scr_cur_x = beg_x + cur_x;
    nc::setsyx(&mut scr_cur_y, &mut scr_cur_x);
}

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

struct Context {
    input: String
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
    where WI: WindowImpl + Send {
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
            ctx: Box::new(Context { input: "".to_string() }),
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
            adjust_screen_cursor_pos(&self.mini_buf);
            nc::doupdate();

            match getchar() {
                'q' => { break }
                ch  => {
                    self.ctx.input.push(ch);
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
