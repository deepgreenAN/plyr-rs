//!
//! RAII types which are used to manage Plyr event listeners.
//!
//! When the some kind of `EventListener` is dropped, it will automatically deregister the event listener and
//! clean up the closure's memory.
//!
//! ## Example
//!
//! ```rust
//! use plyr::events::{PlyrStandardEventType, PlyrYoutubeEventListener};
//! use plyr::Plyr;
//!
//! let player = Plyr::new("#player");
//!
//! let listener = PlyrYoutubeEventListener::new(
//!     &player,
//!     PlyrStandardEventType::playing.into(),
//!     move |_| log("playing")
//! );
//! ```

use crate::plyr::{Plyr, PlyrEvent, PlyrStateChangeEvent};
use crate::{Error as PlyrError, Provider};
use wasm_bindgen::{
    prelude::{Closure, JsValue},
    JsCast, UnwrapThrowExt,
};

use std::fmt::Display;
use strum_macros::Display as StrumDisplay;

// -------------------------------------------------------------------------------------------------
// PlyrEventListener

/// base plyr event listener
#[allow(clippy::type_complexity)]
struct PlyrEventListener {
    target: Plyr,
    event_type: String,
    callback: Option<Closure<dyn FnMut(&JsValue)>>,
}

impl PlyrEventListener {
    fn new<F: FnMut(&JsValue) + 'static>(target: &Plyr, event_type: String, callback: F) -> Self {
        let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(&JsValue)>);
        target.on(&event_type, callback.as_ref().unchecked_ref());
        Self {
            target: target.clone(),
            event_type,
            callback: Some(callback),
        }
    }
    fn once<F: FnOnce(&JsValue) + 'static>(target: &Plyr, event_type: String, callback: F) -> Self {
        let callback = Closure::once(Box::new(callback));
        target.once(&event_type, callback.as_ref().unchecked_ref());
        Self {
            target: target.clone(),
            event_type,
            callback: Some(callback),
        }
    }
    /// Forget inner closure. This should be use when you want the closure to last forever.
    fn forget(&mut self) {
        self.callback.take().unwrap_throw().forget();
    }
}

impl Drop for PlyrEventListener {
    fn drop(&mut self) {
        if let Some(callback) = &self.callback {
            self.target
                .off(&self.event_type, callback.as_ref().unchecked_ref());
        }
    }
}

// -------------------------------------------------------------------------------------------------
// PlyrStandardEventListener

/// EventType for Plyr standard events.
#[allow(non_camel_case_types)]
#[derive(StrumDisplay, Debug, Clone)]
pub enum PlyrStandardEventType {
    progress,
    playing,
    play,
    pause,
    timeupdate,
    volumechange,
    seeking,
    seeked,
    ratechange,
    ended,
    enterfullscreen,
    exitfullscreen,
    captionenabled,
    captiondisabled,
    languagechange,
    controlshidden,
    controlsshown,
    ready,
}

/// EventListener for Plyr standard events.
pub struct PlyrStandardEventListener {
    base_event_listener: PlyrEventListener,
}

impl PlyrStandardEventListener {
    /// Constructor of EventListener for Plyr standard events.
    pub fn new<F: FnMut(&PlyrEvent) + 'static>(
        target: &Plyr,
        standard_event_type: PlyrStandardEventType,
        mut callback: F,
    ) -> Self {
        let callback =
            move |event: &JsValue| callback(event.clone().unchecked_into::<PlyrEvent>().as_ref());
        Self {
            base_event_listener: PlyrEventListener::new(
                target,
                standard_event_type.to_string(),
                callback,
            ),
        }
    }
    /// Constructor of EventListener for Plyr standard events. The callback is called only once.
    pub fn once<F: FnOnce(&PlyrEvent) + 'static>(
        target: &Plyr,
        standard_event_type: PlyrStandardEventType,
        callback: F,
    ) -> Self {
        let callback =
            move |event: &JsValue| callback(event.clone().unchecked_into::<PlyrEvent>().as_ref());
        Self {
            base_event_listener: PlyrEventListener::once(
                target,
                standard_event_type.to_string(),
                callback,
            ),
        }
    }

    /// Forget inner closure. This should be use when you want the closure to last forever.
    pub fn forget(&mut self) {
        self.base_event_listener.forget();
    }
}

// -------------------------------------------------------------------------------------------------
// PlyrHtml5EventListener

/// EventType for Plyr html5 events.
#[cfg(feature = "html5")]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum PlyrHtml5EventType {
    loadstart,
    loadeddata,
    loadedmetadata,
    canplay,
    canplaythrough,
    stalled,
    waiting,
    emptied,
    cuechange,
    error,
    PlyrStandardEventType(PlyrStandardEventType),
}

#[cfg(feature = "html5")]
impl From<PlyrStandardEventType> for PlyrHtml5EventType {
    fn from(standard_event_type: PlyrStandardEventType) -> Self {
        Self::PlyrStandardEventType(standard_event_type)
    }
}

#[cfg(feature = "html5")]
impl Display for PlyrHtml5EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            Self::loadstart => "loadstart".to_string(),
            Self::loadeddata => "loadeddata".to_string(),
            Self::loadedmetadata => "loadedmetadata".to_string(),
            Self::canplay => "canplay".to_string(),
            Self::canplaythrough => "canplaythrough".to_string(),
            Self::stalled => "stalled".to_string(),
            Self::waiting => "waiting".to_string(),
            Self::emptied => "emptied".to_string(),
            Self::cuechange => "cuechange".to_string(),
            Self::error => "error".to_string(),
            Self::PlyrStandardEventType(event_type) => event_type.to_string(),
        };
        write!(f, "{formatted}")
    }
}

/// EventListener for standard and Html5 events. *This module requires the following crate features to be activated: `html5`.*
#[cfg(feature = "html5")]
pub struct PlyrHtml5EventListener {
    base_event_listener: PlyrEventListener,
}

#[cfg(feature = "html5")]
impl PlyrHtml5EventListener {
    /// Constructor of EventListener for Plyr standard and html5 events.
    pub fn new<F: FnMut(&PlyrEvent) + 'static>(
        target: &Plyr,
        event_type: PlyrHtml5EventType,
        mut callback: F,
    ) -> Result<Self, PlyrError> {
        if let Provider::html5 = target.provider() {
            let callback = move |event: &JsValue| {
                callback(event.clone().unchecked_into::<PlyrEvent>().as_ref())
            };
            Ok(Self {
                base_event_listener: PlyrEventListener::new(
                    target,
                    event_type.to_string(),
                    callback,
                ),
            })
        } else {
            Err(PlyrError::WrongEventProviderError)
        }
    }
    /// Constructor of EventListener for Plyr standard and html5 events. The callback is called only once.
    pub fn once<F: FnOnce(&PlyrEvent) + 'static>(
        target: &Plyr,
        event_type: PlyrHtml5EventType,
        callback: F,
    ) -> Result<Self, PlyrError> {
        if let Provider::html5 = target.provider() {
            let callback = move |event: &JsValue| {
                callback(event.clone().unchecked_into::<PlyrEvent>().as_ref())
            };
            Ok(Self {
                base_event_listener: PlyrEventListener::once(
                    target,
                    event_type.to_string(),
                    callback,
                ),
            })
        } else {
            Err(PlyrError::WrongEventProviderError)
        }
    }
    /// Forget inner closure. This should be use when you want the closure to last forever.
    pub fn forget(&mut self) {
        self.base_event_listener.forget();
    }
}

// -------------------------------------------------------------------------------------------------
// PlyrYoutubeEventListener

/// EventType for Plyr youtube events.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum PlyrYoutubeEventType {
    qualitychange,
    qualityrequested,
    PlyrStandardEventType(PlyrStandardEventType),
}

impl From<PlyrStandardEventType> for PlyrYoutubeEventType {
    fn from(event_type: PlyrStandardEventType) -> Self {
        Self::PlyrStandardEventType(event_type)
    }
}

impl Display for PlyrYoutubeEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            Self::qualitychange => "qualitychange".to_string(),
            Self::qualityrequested => "qualityrequested".to_string(),
            Self::PlyrStandardEventType(event_type) => event_type.to_string(),
        };
        write!(f, "{formatted}")
    }
}

/// EventListener for Plyr standard and youtube events, which includes statechange event .
pub struct PlyrYoutubeEventListener {
    base_event_listener: PlyrEventListener,
}

impl PlyrYoutubeEventListener {
    /// Constructor of EventListener for Plyr standard and youtube events.
    pub fn new<F: FnMut(&PlyrEvent) + 'static>(
        target: &Plyr,
        event_type: PlyrYoutubeEventType,
        mut callback: F,
    ) -> Result<Self, PlyrError> {
        if let Provider::youtube = target.provider() {
            let callback = move |event: &JsValue| {
                callback(event.clone().unchecked_into::<PlyrEvent>().as_ref())
            };
            Ok(Self {
                base_event_listener: PlyrEventListener::new(
                    target,
                    event_type.to_string(),
                    callback,
                ),
            })
        } else {
            Err(PlyrError::WrongEventProviderError)
        }
    }
    /// Constructor of EventListener for Plyr standard and youtube events. The callback is called only once.
    pub fn once<F: FnOnce(&PlyrEvent) + 'static>(
        target: &Plyr,
        event_type: PlyrYoutubeEventType,
        callback: F,
    ) -> Result<Self, PlyrError> {
        if let Provider::youtube = target.provider() {
            let callback = move |event: &JsValue| {
                callback(event.clone().unchecked_into::<PlyrEvent>().as_ref())
            };
            Ok(Self {
                base_event_listener: PlyrEventListener::once(
                    target,
                    event_type.to_string(),
                    callback,
                ),
            })
        } else {
            Err(PlyrError::WrongEventProviderError)
        }
    }
    /// Constructor of EventListener for youtube statechange event.
    pub fn new_on_statechange<F: FnMut(&PlyrStateChangeEvent) + 'static>(
        target: &Plyr,
        mut callback: F,
    ) -> Result<Self, PlyrError> {
        if let Provider::youtube = target.provider() {
            let callback = move |event: &JsValue| {
                callback(
                    event
                        .clone()
                        .unchecked_into::<PlyrStateChangeEvent>()
                        .as_ref(),
                )
            };
            Ok(Self {
                base_event_listener: PlyrEventListener::new(
                    target,
                    "statechange".to_string(),
                    callback,
                ),
            })
        } else {
            Err(PlyrError::WrongEventProviderError)
        }
    }
    /// Constructor of EventListener for youtube statechange event. The callback is called only once.
    pub fn once_on_statechange<F: FnOnce(&PlyrStateChangeEvent) + 'static>(
        target: &Plyr,
        callback: F,
    ) -> Result<Self, PlyrError> {
        if let Provider::youtube = target.provider() {
            let callback = move |event: &JsValue| {
                callback(
                    event
                        .clone()
                        .unchecked_into::<PlyrStateChangeEvent>()
                        .as_ref(),
                )
            };
            Ok(Self {
                base_event_listener: PlyrEventListener::once(
                    target,
                    "statechange".to_string(),
                    callback,
                ),
            })
        } else {
            Err(PlyrError::WrongEventProviderError)
        }
    }
    /// Forget inner closure. This should be use when you want the closure to last forever.
    pub fn forget(&mut self) {
        self.base_event_listener.forget();
    }
}

// -------------------------------------------------------------------------------------------------
// DestroyEventListener

/// EventListener for Plyr destroy event.
pub struct DestroyEventListener {
    callback: Option<Closure<dyn FnMut()>>,
}

impl DestroyEventListener {
    /// Constructor of EventListener for Plyr destroy event.
    pub fn new<F: FnOnce() + 'static>(target: &Plyr, callback: F) -> Self {
        let callback = Closure::once(Box::new(callback));
        target.destroy_with_callback(callback.as_ref().unchecked_ref());
        Self {
            callback: Some(callback),
        }
    }
    /// Constructor of EventListener for Plyr destroy event with a soft flag.
    pub fn new_with_soft<F: FnOnce() + 'static>(target: &Plyr, callback: F, soft: bool) -> Self {
        let callback = Closure::once(Box::new(callback));
        target.destroy_with_callback_and_soft(callback.as_ref().unchecked_ref(), soft);
        Self {
            callback: Some(callback),
        }
    }
    /// Forget inner closure. This should be use when you want the closure to last forever.
    pub fn forget(&mut self) {
        self.callback.take().unwrap_throw().forget();
    }
}
