// This file was generated by gir (b798f4f) from gir-files (11e0e6d)
// DO NOT EDIT

use CellRenderer;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct CellRendererPixbuf(Object<ffi::GtkCellRendererPixbuf>): CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_pixbuf_get_type(),
    }
}

impl CellRendererPixbuf {
    pub fn new() -> CellRendererPixbuf {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_pixbuf_new()).downcast_unchecked()
        }
    }
}
