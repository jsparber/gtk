// This file was generated by gir (6bcd52a) from gir-files (1069259)
// DO NOT EDIT

use EventController;
use Gesture;
use ffi;
use glib;
#[cfg(feature = "v3_14")]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_14")]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "v3_14")]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(feature = "v3_14")]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct GestureSingle(Object<ffi::GtkGestureSingle>): Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_single_get_type(),
    }
}

pub trait GestureSingleExt {
    #[cfg(feature = "v3_14")]
    fn get_button(&self) -> u32;

    #[cfg(feature = "v3_14")]
    fn get_current_button(&self) -> u32;

    //#[cfg(feature = "v3_14")]
    //fn get_current_sequence(&self) -> /*Ignored*/Option<gdk::EventSequence>;

    #[cfg(feature = "v3_14")]
    fn get_exclusive(&self) -> bool;

    #[cfg(feature = "v3_14")]
    fn get_touch_only(&self) -> bool;

    #[cfg(feature = "v3_14")]
    fn set_button(&self, button: u32);

    #[cfg(feature = "v3_14")]
    fn set_exclusive(&self, exclusive: bool);

    #[cfg(feature = "v3_14")]
    fn set_touch_only(&self, touch_only: bool);

    #[cfg(feature = "v3_14")]
    fn connect_property_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_14")]
    fn connect_property_exclusive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_14")]
    fn connect_property_touch_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<GestureSingle> + IsA<glib::object::Object>> GestureSingleExt for O {
    #[cfg(feature = "v3_14")]
    fn get_button(&self) -> u32 {
        unsafe {
            ffi::gtk_gesture_single_get_button(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_current_button(&self) -> u32 {
        unsafe {
            ffi::gtk_gesture_single_get_current_button(self.to_glib_none().0)
        }
    }

    //#[cfg(feature = "v3_14")]
    //fn get_current_sequence(&self) -> /*Ignored*/Option<gdk::EventSequence> {
    //    unsafe { TODO: call ffi::gtk_gesture_single_get_current_sequence() }
    //}

    #[cfg(feature = "v3_14")]
    fn get_exclusive(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_single_get_exclusive(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_touch_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_single_get_touch_only(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_button(&self, button: u32) {
        unsafe {
            ffi::gtk_gesture_single_set_button(self.to_glib_none().0, button);
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_exclusive(&self, exclusive: bool) {
        unsafe {
            ffi::gtk_gesture_single_set_exclusive(self.to_glib_none().0, exclusive.to_glib());
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_touch_only(&self, touch_only: bool) {
        unsafe {
            ffi::gtk_gesture_single_set_touch_only(self.to_glib_none().0, touch_only.to_glib());
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_property_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::button",
                transmute(notify_button_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_property_exclusive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::exclusive",
                transmute(notify_exclusive_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_property_touch_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::touch-only",
                transmute(notify_touch_only_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn notify_button_trampoline<P>(this: *mut ffi::GtkGestureSingle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GestureSingle> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GestureSingle::from_glib_none(this).downcast_unchecked())
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn notify_exclusive_trampoline<P>(this: *mut ffi::GtkGestureSingle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GestureSingle> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GestureSingle::from_glib_none(this).downcast_unchecked())
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn notify_touch_only_trampoline<P>(this: *mut ffi::GtkGestureSingle, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GestureSingle> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GestureSingle::from_glib_none(this).downcast_unchecked())
}
