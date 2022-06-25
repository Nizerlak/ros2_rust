use crate::rcl_bindings::*;
use crate::{Subscription, ToResult};

use rosidl_runtime_rs::RmwMessage;

use std::ops::Deref;

/// A message owned by the middleware.
///
/// It dereferences to a `&T`.
///
/// In `rclcpp`, this is referred to as a "loaned message".
pub struct ReadOnlyMessage<'a, T>
where
    T: RmwMessage,
{
    pub(super) msg_ptr: *const T,
    pub(super) subscription: &'a Subscription<T>,
}

impl<'a, T> Deref for ReadOnlyMessage<'a, T>
where
    T: RmwMessage,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.msg_ptr }
    }
}

impl<'a, T> Drop for ReadOnlyMessage<'a, T>
where
    T: RmwMessage,
{
    fn drop(&mut self) {
        unsafe {
            rcl_return_loaned_message_from_subscription(
                &*self.subscription.handle.lock(),
                self.msg_ptr as *mut _,
            )
            .ok()
            .unwrap();
        }
    }
}
