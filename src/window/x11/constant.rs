// Contains X.h contants definition



/// Contains XInput event masks
/// 
/// # Reference(s)
/// https://tronche.com/gui/x/xlib/events/mask.html
/// X11/X.h
#[allow(unused)]
pub mod xevent_masks {
    use std::os::raw::{ c_long};

    pub const  NO_EVENT_MASK : c_long = 0;
    pub const KEY_PRESS_MASK: c_long =0;
    pub const KEY_RELEASE_MASK: c_long =1;
    pub const BUTTON_PRESS_MASK: c_long =2;
    pub const BUTTON_RELEASE_MASK	: c_long =3;
    pub const ENTER_WINDOW_MASK: c_long =4;
    pub const LEAVE_WINDOW_MASK: c_long =5;
    pub const POINTER_MOTION_MASK	: c_long =6;
    pub const POINTER_MOTION_HINT_MASK	: c_long =7;
    pub const BUTTON1_MOTION_MASK	: c_long =8;
    pub const BUTTON2_MOTION_MASK	: c_long =9;
    pub const BUTTON3_MOTION_MASK	: c_long =10;
    pub const BUTTON4_MOTION_MASK	: c_long =11;
    pub const BUTTON5_MOTION_MASK	: c_long =12;
    pub const BUTTON_MOTION_MASK	: c_long =13;
    pub const KEYMAP_STATE_MASK: c_long =14;
    pub const EXPOSURE_MASK: c_long =15;
    pub const VISIBILITY_CHANGE_MASK	: c_long =16;
    pub const STRUCTURE_NOTIFY_MASK	: c_long =17;
    pub const RESIZE_REDIRECT_MASK	: c_long =18;
    pub const SUBSTRUCTURE_NOTIFY_MASK	: c_long =19;
    pub const SUBSTRUCTURE_REDIRECT_MASK: c_long =20;
    pub const FOCUS_CHANGE_MASK: c_long =21;
    pub const PROPERTY_CHANGE_MASK	: c_long =22;
    pub const COLORMAP_CHANGE_MASK	: c_long =23;
    pub const OWNER_GRAB_BUTTON_MASK	: c_long =24;

}
