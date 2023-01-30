// Contains bindings for XLib
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};

pub enum X11display {}
pub enum X11Window {}

pub enum XEvent {}

#[link(name = "X11")]
extern {
    

    /// The XOpenDisplay function returns a Display structure that serves as the connection to the 
    /// X server and that contains all the information about that X server.
    /// 
    /// # Reference(s)
    /// https://www.x.org/releases/X11R7.5/doc/man/man3/XOpenDisplay.3.html
    pub fn XOpenDisplay(display_name : *const c_char) -> *mut X11display;

    /// Returns the root window for the default screen. 
    /// 
    /// # Reference(s)
    /// https://tronche.com/gui/x/xlib/display/display-macros.html
    pub fn XDefaultRootWindow(display : *mut X11display) -> *mut X11Window;

    /// The XCreateSimpleWindow function creates an unmapped InputOutput subwindow for a specified parent window, 
    /// returns the window ID of the created window, and causes the X server to generate a CreateNotify event
    /// 
    /// # References(s)
    /// https://tronche.com/gui/x/xlib/window/XCreateWindow.html
    pub fn XCreateSimpleWindow(display : *mut X11display, parent : *mut X11Window, x : c_int, y : c_int, 
        width : c_uint, height : c_uint, border_width : c_uint, border : c_ulong, background : c_ulong) -> *mut X11Window;


    /// The XMapWindow() function maps the window and all of its subwindows that have had map requests.
    /// 
    /// # Reference(s)
    /// https://tronche.com/gui/x/xlib/window/XMapWindow.html
    pub fn XMapWindow(display : *mut X11display, w : *mut X11Window);

    /// The XSelectInput() function requests that the X server report the events associated 
    /// with the specified event mask. Initially, X will not report any of these events.
    /// 
    /// # Reference(s)
    /// https://tronche.com/gui/x/xlib/event-handling/XSelectInput.html
    pub fn XSelectInput(display : *mut X11display, w : *mut X11Window, event_mask: c_long);

    /// The XNextEvent() function copies the first event from the event queue into the specified 
    /// XEvent structure and then removes it from the queue.
    /// 
    /// # Reference(s)
    /// https://tronche.com/gui/x/xlib/event-handling/manipulating-event-queue/XNextEvent.html
    pub fn XNextEvent(display : *mut X11display, event_return : *mut XEvent);

}