// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use RevealerTransitionType;
use Widget;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Revealer(Object<ffi::GtkRevealer, ffi::GtkRevealerClass>): Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_revealer_get_type(),
    }
}

impl Revealer {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn new() -> Revealer {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_revealer_new()).downcast_unchecked()
        }
    }
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
impl Default for Revealer {
    fn default() -> Self {
        Self::new()
    }
}

pub trait RevealerExt {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_child_revealed(&self) -> bool;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_reveal_child(&self) -> bool;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_transition_duration(&self) -> u32;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_transition_type(&self) -> RevealerTransitionType;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_reveal_child(&self, reveal_child: bool);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_transition_duration(&self, duration: u32);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_transition_type(&self, transition: RevealerTransitionType);

    fn get_property_child_revealed(&self) -> bool;

    fn get_property_reveal_child(&self) -> bool;

    fn set_property_reveal_child(&self, reveal_child: bool);

    fn get_property_transition_duration(&self) -> u32;

    fn set_property_transition_duration(&self, transition_duration: u32);

    fn get_property_transition_type(&self) -> RevealerTransitionType;

    fn set_property_transition_type(&self, transition_type: RevealerTransitionType);

    fn connect_property_child_revealed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_reveal_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transition_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transition_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Revealer> + IsA<glib::object::Object>> RevealerExt for O {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_child_revealed(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_revealer_get_child_revealed(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_reveal_child(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_revealer_get_reveal_child(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_transition_duration(&self) -> u32 {
        unsafe {
            ffi::gtk_revealer_get_transition_duration(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_transition_type(&self) -> RevealerTransitionType {
        unsafe {
            from_glib(ffi::gtk_revealer_get_transition_type(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_reveal_child(&self, reveal_child: bool) {
        unsafe {
            ffi::gtk_revealer_set_reveal_child(self.to_glib_none().0, reveal_child.to_glib());
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_transition_duration(&self, duration: u32) {
        unsafe {
            ffi::gtk_revealer_set_transition_duration(self.to_glib_none().0, duration);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_transition_type(&self, transition: RevealerTransitionType) {
        unsafe {
            ffi::gtk_revealer_set_transition_type(self.to_glib_none().0, transition.to_glib());
        }
    }

    fn get_property_child_revealed(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "child-revealed".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_reveal_child(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "reveal-child".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_reveal_child(&self, reveal_child: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "reveal-child".to_glib_none().0, Value::from(&reveal_child).to_glib_none().0);
        }
    }

    fn get_property_transition_duration(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "transition-duration".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_transition_duration(&self, transition_duration: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "transition-duration".to_glib_none().0, Value::from(&transition_duration).to_glib_none().0);
        }
    }

    fn get_property_transition_type(&self) -> RevealerTransitionType {
        unsafe {
            let mut value = Value::from_type(<RevealerTransitionType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "transition-type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_transition_type(&self, transition_type: RevealerTransitionType) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "transition-type".to_glib_none().0, Value::from(&transition_type).to_glib_none().0);
        }
    }

    fn connect_property_child_revealed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::child-revealed",
                transmute(notify_child_revealed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_reveal_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::reveal-child",
                transmute(notify_reveal_child_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_transition_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::transition-duration",
                transmute(notify_transition_duration_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_transition_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::transition-type",
                transmute(notify_transition_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_child_revealed_trampoline<P>(this: *mut ffi::GtkRevealer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Revealer> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Revealer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_reveal_child_trampoline<P>(this: *mut ffi::GtkRevealer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Revealer> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Revealer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_transition_duration_trampoline<P>(this: *mut ffi::GtkRevealer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Revealer> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Revealer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_transition_type_trampoline<P>(this: *mut ffi::GtkRevealer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Revealer> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Revealer::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for Revealer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Revealer")
    }
}
