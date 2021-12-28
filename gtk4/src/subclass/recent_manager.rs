// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`RecentManager`](crate::RecentManager).

use crate::subclass::prelude::*;
use crate::RecentManager;
use glib::translate::*;
use glib::Cast;

pub trait RecentManagerImpl: RecentManagerImplExt + ObjectImpl {
    fn changed(&self, recent_manager: &Self::Type) {
        self.parent_changed(recent_manager)
    }
}

pub trait RecentManagerImplExt: ObjectSubclass {
    fn parent_changed(&self, recent_manager: &Self::Type);
}

impl<T: RecentManagerImpl> RecentManagerImplExt for T {
    fn parent_changed(&self, recent_manager: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkRecentManagerClass;
            if let Some(f) = (*parent_class).changed {
                f(recent_manager
                    .unsafe_cast_ref::<RecentManager>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

unsafe impl<T: RecentManagerImpl> IsSubclassable<T> for RecentManager {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert!(
            crate::rt::is_initialized(),
            "GTK has to be initialized first"
        );

        let klass = class.as_mut();
        klass.changed = Some(recent_manager_changed::<T>);
    }
}

unsafe extern "C" fn recent_manager_changed<T: RecentManagerImpl>(ptr: *mut ffi::GtkRecentManager) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<RecentManager> = from_glib_borrow(ptr);

    imp.changed(wrap.unsafe_cast_ref())
}
