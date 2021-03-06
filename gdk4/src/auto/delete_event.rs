// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Event;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct DeleteEvent(Object<ffi::GdkDeleteEvent>) @extends Event;

    match fn {
        get_type => || ffi::gdk_delete_event_get_type(),
    }
}

impl DeleteEvent {}

impl fmt::Display for DeleteEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeleteEvent")
    }
}
