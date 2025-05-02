use crate::hid_state::HidModifiers;

pub mod display;

#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutputDeviceEvent {
    LayerChange(u8),
    Modifier(HidModifiers),
}

pub trait OutputDevice {
    async fn process_event(&mut self, e: OutputDeviceEvent);
}

/// Macro to bind input devices to event channels and run all of them.
///
/// This macro simplifies the creation of a task that reads events from multiple input devices
/// and sends them to specified channels. It allows for efficient handling of
/// input events in a concurrent manner.
///
/// # Arguments
///
/// * `dev`: A list of input devices grouped in parentheses.
/// * `channel`: The channel that devices send the events to.
///
/// # Example
/// ```rust
/// use rmk::channel::{blocking_mutex::raw::NoopRawMutex, channel::Channel, EVENT_CHANNEL};
/// // Initialize channel
/// let local_channel: Channel<NoopRawMutex, Event, 16> = Channel::new();
///
/// // Define your input devices, both MyInputDevice and MyInputDevice2 should implement `InputDevice] trait
/// struct MyInputDevice;
/// struct MyInputDevice2;
///
/// let d1 = MyInputDevice{};
/// let d2 = MyInputDevice2{};
/// // Bind devices to channels and run, RMK also provides EVENT_CHANNEL for general use
/// let device_future = run_devices! {
///     (d1, d2) => local_channel,
///     (matrix) => rmk::EVENT_CHANNEL,
/// };
///
/// ```
#[macro_export]
macro_rules! run_output_devices {
    ( $( ( $( $dev:ident ),* ) => $channel:expr),+ $(,)? ) => {{
        use $crate::output_device::OutputDevice;
        $crate::join_all!(
            $(
                $crate::join_all!(
                    $(
                        async {
                            let mut sub = ::defmt::unwrap!($channel.subscriber());
                            loop {
                                let e = sub.next_message_pure().await;
                                $dev.process_event(e).await;
                            }
                        }
                    ),*
                )
            ),+
        )
    }};
}
