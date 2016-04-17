// This file was generated by gir (b798f4f) from gir-files (11e0e6d)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
#[cfg(feature = "v3_14")]
use Object;
#[cfg(feature = "v3_14")]
use Widget;
use ffi;
#[cfg(feature = "v3_14")]
use ffi::GtkGestureDrag;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_14")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_14")]
use glib_ffi::gpointer;
#[cfg(feature = "v3_14")]
use libc::c_double;
#[cfg(feature = "v3_14")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_14")]
use std::mem;
#[cfg(feature = "v3_14")]
use std::mem::transmute;

glib_wrapper! {
    pub struct GestureDrag(Object<ffi::GtkGestureDrag>): GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_drag_get_type(),
    }
}

impl GestureDrag {
    #[cfg(feature = "v3_14")]
    pub fn new<T: IsA<Widget>>(widget: &T) -> GestureDrag {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_drag_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait GestureDragExt {
    #[cfg(feature = "v3_14")]
    fn get_offset(&self) -> Option<(f64, f64)>;

    #[cfg(feature = "v3_14")]
    fn get_start_point(&self) -> Option<(f64, f64)>;

    #[cfg(feature = "v3_14")]
    fn connect_drag_begin<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_14")]
    fn connect_drag_end<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_14")]
    fn connect_drag_update<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<GestureDrag> + IsA<Object>> GestureDragExt for O {
    #[cfg(feature = "v3_14")]
    fn get_offset(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_drag_get_offset(self.to_glib_none().0, &mut x, &mut y));
            if ret { Some((x, y)) } else { None }
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_start_point(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_drag_get_start_point(self.to_glib_none().0, &mut x, &mut y));
            if ret { Some((x, y)) } else { None }
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_drag_begin<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "drag-begin",
                transmute(drag_begin_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_drag_end<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "drag-end",
                transmute(drag_end_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_drag_update<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "drag-update",
                transmute(drag_update_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn drag_begin_trampoline<T>(this: *mut GtkGestureDrag, start_x: c_double, start_y: c_double, f: gpointer)
where T: IsA<GestureDrag> {
    callback_guard!();
    let f: &Box_<Fn(&T, f64, f64) + 'static> = transmute(f);
    f(&GestureDrag::from_glib_none(this).downcast_unchecked(), start_x, start_y)
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn drag_end_trampoline<T>(this: *mut GtkGestureDrag, offset_x: c_double, offset_y: c_double, f: gpointer)
where T: IsA<GestureDrag> {
    callback_guard!();
    let f: &Box_<Fn(&T, f64, f64) + 'static> = transmute(f);
    f(&GestureDrag::from_glib_none(this).downcast_unchecked(), offset_x, offset_y)
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn drag_update_trampoline<T>(this: *mut GtkGestureDrag, offset_x: c_double, offset_y: c_double, f: gpointer)
where T: IsA<GestureDrag> {
    callback_guard!();
    let f: &Box_<Fn(&T, f64, f64) + 'static> = transmute(f);
    f(&GestureDrag::from_glib_none(this).downcast_unchecked(), offset_x, offset_y)
}
