// This file was generated by gir (e912ccf) from gir-files (469db10)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use Container;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct LinkButton(Object<ffi::GtkLinkButton, ffi::GtkLinkButtonClass>): Button, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_link_button_get_type(),
    }
}

impl LinkButton {
    pub fn new(uri: &str) -> LinkButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_link_button_new(uri.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_label<'a, P: Into<Option<&'a str>>>(uri: &str, label: P) -> LinkButton {
        assert_initialized_main_thread!();
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_link_button_new_with_label(uri.to_glib_none().0, label.0)).downcast_unchecked()
        }
    }
}

pub trait LinkButtonExt {
    fn get_uri(&self) -> Option<String>;

    fn get_visited(&self) -> bool;

    fn set_uri(&self, uri: &str);

    fn set_visited(&self, visited: bool);

    fn connect_activate_link<F: Fn(&Self) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visited_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<LinkButton> + IsA<glib::object::Object>> LinkButtonExt for O {
    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_link_button_get_uri(self.to_glib_none().0))
        }
    }

    fn get_visited(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_link_button_get_visited(self.to_glib_none().0))
        }
    }

    fn set_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_link_button_set_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    fn set_visited(&self, visited: bool) {
        unsafe {
            ffi::gtk_link_button_set_visited(self.to_glib_none().0, visited.to_glib());
        }
    }

    fn connect_activate_link<F: Fn(&Self) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-link",
                transmute(activate_link_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::uri",
                transmute(notify_uri_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_visited_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::visited",
                transmute(notify_visited_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_link_trampoline<P>(this: *mut ffi::GtkLinkButton, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<LinkButton> {
    callback_guard!();
    let f: &&(Fn(&P) -> Inhibit + 'static) = transmute(f);
    f(&LinkButton::from_glib_borrow(this).downcast_unchecked()).to_glib()
}

unsafe extern "C" fn notify_uri_trampoline<P>(this: *mut ffi::GtkLinkButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LinkButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LinkButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_visited_trampoline<P>(this: *mut ffi::GtkLinkButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LinkButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LinkButton::from_glib_borrow(this).downcast_unchecked())
}
