// Generated with "script/rustify_x11_event.sh"
use std::os::raw::{ c_int, c_long, c_uint, c_ulong, c_char, c_uchar, c_short, c_void };

// Types definition (ref : https://docs.rs/x11/latest/x11)
pub type Time = c_ulong;
pub type XID = c_ulong;
pub type Atom = XID;
pub type Colormap = XID;
pub type Drawable = XID;

/// Union 'data' of XClientMessageEvent struct.
#[repr(C)]
pub union XClientMessageEvent_data {
pub _b : [c_char; 20],
pub _s : [c_short; 10],
pub _l : [c_long; 5]
}
#[repr(C)]
pub struct XKeyEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_window:Window,
	_root:Window,
	_subwindow:Window,
	_time:Time,
	_x:c_int,
	_y:c_int,
	_x_root:c_int,
	_y_root:c_int,
	_state:c_uint,
	_keycode:c_uint,
	_same_screen:bool,
}
#[repr(C)]
pub struct XButtonEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_window:Window,
	_root:Window,
	_subwindow:Window,
	_time:Time,
	_x:c_int,
	_y:c_int,
	_x_root:c_int,
	_y_root:c_int,
	_state:c_uint,
	_button:c_uint,
	_same_screen:bool,
}
#[repr(C)]
pub struct XMotionEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_window:Window,
	_root:Window,
	_subwindow:Window,
	_time:Time,
	_x:c_int,
	_y:c_int,
	_x_root:c_int,
	_y_root:c_int,
	_state:c_uint,
	_is_hint:c_int,
	_same_screen:bool,
}
#[repr(C)]
pub struct XCrossingEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_window:Window,
	_root:Window,
	_subwindow:Window,
	_time:Time,
	_x:c_int,
	_y:c_int,
	_x_root:c_int,
	_y_root:c_int,
	_mode:c_int,
	_detail:c_int,
	_same_screen:bool,
	_focus:bool,
	_state:c_uint,
}
#[repr(C)]
pub struct XFocusChangeEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_window:Window,
	_mode:c_int,
	_detail:c_int,
}
#[repr(C)]
pub struct XKeymapEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_window:Window,
	_key_vector:[c_char; 32],
}
#[repr(C)]
pub struct XExposeEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_window:Window,
	_x:c_int,
	_y:c_int,
	_width:c_int,
	_height:c_int,
	_count:c_int,
}
#[repr(C)]
pub struct XGraphicsExposeEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_drawable:Drawable,
	_x:c_int,
	_y:c_int,
	_width:c_int,
	_height:c_int,
	_count:c_int,
	_major_code:c_int,
	_minor_code:c_int,
}
#[repr(C)]
pub struct XNoExposeEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_drawable:Drawable,
	_major_code:c_int,
	_minor_code:c_int,
}
#[repr(C)]
pub struct XVisibilityEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_window:Window,
	_state:c_int,
}
#[repr(C)]
pub struct XCreateWindowEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_parent:Window,
	_window:Window,
	_x:c_int,
	_y:c_int,
	_width:c_int,
	_height:c_int,
	_border_width:c_int,
	_override_redirect:bool,
}
#[repr(C)]
pub struct XDestroyWindowEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_event:Window,
	_window:Window,
}
#[repr(C)]
pub struct XUnmapEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_event:Window,
	_window:Window,
	_from_configure:bool,
}
#[repr(C)]
pub struct XMapEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_event:Window,
	_window:Window,
	_override_redirect:bool,
}
#[repr(C)]
pub struct XMapRequestEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_parent:Window,
	_window:Window,
}
#[repr(C)]
pub struct XReparentEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_event:Window,
	_window:Window,
	_parent:Window,
	_x:c_int,
	_y:c_int,
	_override_redirect:bool,
}
#[repr(C)]
pub struct XConfigureEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_event:Window,
	_window:Window,
	_x:c_int,
	_y:c_int,
	_width:c_int,
	_height:c_int,
	_border_width:c_int,
	_above:Window,
	_override_redirect:bool,
}
#[repr(C)]
pub struct XGravityEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_event:Window,
	_window:Window,
	_x:c_int,
	_y:c_int,
}
#[repr(C)]
pub struct XResizeRequestEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_window:Window,
	_width:c_int,
	_height:c_int,
}
#[repr(C)]
pub struct XConfigureRequestEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_parent:Window,
	_window:Window,
	_x:c_int,
	_y:c_int,
	_width:c_int,
	_height:c_int,
	_border_width:c_int,
	_above:Window,
	_detail:c_int,
	_value_mask:c_ulong,
}
#[repr(C)]
pub struct XCirculateEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_event:Window,
	_window:Window,
	_place:c_int,
}
#[repr(C)]
pub struct XCirculateRequestEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_parent:Window,
	_window:Window,
	_place:c_int,
}
#[repr(C)]
pub struct XPropertyEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_window:Window,
	_atom:Atom,
	_time:Time,
	_state:c_int,
}
#[repr(C)]
pub struct XSelectionClearEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_window:Window,
	_selection:Atom,
	_time:Time,
}
#[repr(C)]
pub struct XSelectionRequestEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_owner:Window,
	_requestor:Window,
	_selection:Atom,
	_target:Atom,
	_property:Atom,
	_time:Time,
}
#[repr(C)]
pub struct XSelectionEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_requestor:Window,
	_selection:Atom,
	_target:Atom,
	_property:Atom,
	_time:Time,
}
#[repr(C)]
pub struct XColormapEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_window:Window,
	_new:bool,
	_state:c_int,
}
#[repr(C)]
pub struct data {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_window:Window,
	_message_type:Atom,
	_format:c_int,
	_data:XClientMessageEvent_data,
}
#[repr(C)]
pub struct XMappingEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_window:Window,
	_request:c_int,
	_first_keycode:c_int,
	_count:c_int,
}
#[repr(C)]
pub struct XErrorEvent {
	_type:c_int,
	_display:*mut Display,
	_serial:c_ulong,
	_error_code:c_uchar,
	_request_code:c_uchar,
	_minor_code:c_uchar,
}
#[repr(C)]
pub struct XAnyEvent {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_Display:Display,
	_the:Display,
	_event:Display,
	_window:Window,
}
#[repr(C)]
pub struct XGenericEventCookie {
	_type:c_int,
	_serial:c_ulong,
	_send_event:bool,
	_display:*mut Display,
	_extension:c_int,
	_evtype:c_int,
	_cookie:c_uint,
	_data:*mut c_void,
}
Ending now
