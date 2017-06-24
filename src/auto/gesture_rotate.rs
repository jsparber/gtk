// This file was generated by gir (3c73dd9) from gir-files (71d73f0)
// DO NOT EDIT

use EventController;
use Gesture;
#[cfg(feature = "v3_14")]
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_14")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_14")]
use glib_ffi;
#[cfg(feature = "v3_14")]
use libc;
#[cfg(feature = "v3_14")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_14")]
use std::mem::transmute;

glib_wrapper! {
    pub struct GestureRotate(Object<ffi::GtkGestureRotate>): Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_rotate_get_type(),
    }
}

impl GestureRotate {
    #[cfg(feature = "v3_14")]
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureRotate {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_rotate_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait GestureRotateExt {
    #[cfg(feature = "v3_14")]
    fn get_angle_delta(&self) -> f64;

    #[cfg(feature = "v3_14")]
    fn connect_angle_changed<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<GestureRotate> + IsA<glib::object::Object>> GestureRotateExt for O {
    #[cfg(feature = "v3_14")]
    fn get_angle_delta(&self) -> f64 {
        unsafe {
            ffi::gtk_gesture_rotate_get_angle_delta(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_angle_changed<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "angle-changed",
                transmute(angle_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn angle_changed_trampoline<P>(this: *mut ffi::GtkGestureRotate, angle: libc::c_double, angle_delta: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureRotate> {
    callback_guard!();
    let f: &Box_<Fn(&P, f64, f64) + 'static> = transmute(f);
    f(&GestureRotate::from_glib_none(this).downcast_unchecked(), angle, angle_delta)
}
