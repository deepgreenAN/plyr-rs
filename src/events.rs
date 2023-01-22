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

/// EventListener for Plyr Standard events.
pub struct PlyrStandardEventListener {
    base_event_listener: PlyrEventListener,
}

impl PlyrStandardEventListener {
    pub fn new<F: FnMut(&PlyrEvent) + 'static>(
        target: &Plyr,
        standard_event_type: PlyrStandardEventType,
        mut callback: F,
    ) -> Self {
        let callback =
            move |event: &JsValue| callback(Into::<PlyrEvent>::into(event.clone()).as_ref());
        Self {
            base_event_listener: PlyrEventListener::new(
                target,
                standard_event_type.to_string(),
                callback,
            ),
        }
    }
    pub fn once<F: FnOnce(&PlyrEvent) + 'static>(
        target: &Plyr,
        standard_event_type: PlyrStandardEventType,
        callback: F,
    ) -> Self {
        let callback =
            move |event: &JsValue| callback(Into::<PlyrEvent>::into(event.clone()).as_ref());
        Self {
            base_event_listener: PlyrEventListener::once(
                target,
                standard_event_type.to_string(),
                callback,
            ),
        }
    }
    pub fn forget(&mut self) {
        self.base_event_listener.forget();
    }
}

// -------------------------------------------------------------------------------------------------
// PlyrHtml5EventListener

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
        write!(f, "{}", formatted)
    }
}

/// EventListener for Standard and Html5 events.
#[cfg(feature = "html5")]
pub struct PlyrHtml5EventListener {
    base_event_listener: PlyrEventListener,
}

#[cfg(feature = "html5")]
impl PlyrHtml5EventListener {
    pub fn new<F: FnMut(&PlyrEvent) + 'static>(
        target: &Plyr,
        event_type: PlyrHtml5EventType,
        mut callback: F,
    ) -> Result<Self, PlyrError> {
        if let Provider::html5 = target.provider() {
            let callback =
                move |event: &JsValue| callback(Into::<PlyrEvent>::into(event.clone()).as_ref());
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
    pub fn once<F: FnOnce(&PlyrEvent) + 'static>(
        target: &Plyr,
        event_type: PlyrHtml5EventType,
        callback: F,
    ) -> Result<Self, PlyrError> {
        if let Provider::html5 = target.provider() {
            let callback =
                move |event: &JsValue| callback(Into::<PlyrEvent>::into(event.clone()).as_ref());
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
    pub fn forget(&mut self) {
        self.base_event_listener.forget();
    }
}

// -------------------------------------------------------------------------------------------------
// PlyrYoutubeEventListener

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
        write!(f, "{}", formatted)
    }
}

/// EventListener for Standard event type and Youtube event type with statechange event.
pub struct PlyrYoutubeEventListener {
    base_event_listener: PlyrEventListener,
}

impl PlyrYoutubeEventListener {
    pub fn new<F: FnMut(&PlyrEvent) + 'static>(
        target: &Plyr,
        event_type: PlyrYoutubeEventType,
        mut callback: F,
    ) -> Result<Self, PlyrError> {
        if let Provider::youtube = target.provider() {
            let callback =
                move |event: &JsValue| callback(Into::<PlyrEvent>::into(event.clone()).as_ref());
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
    pub fn once<F: FnOnce(&PlyrEvent) + 'static>(
        target: &Plyr,
        event_type: PlyrYoutubeEventType,
        callback: F,
    ) -> Result<Self, PlyrError> {
        if let Provider::youtube = target.provider() {
            let callback =
                move |event: &JsValue| callback(Into::<PlyrEvent>::into(event.clone()).as_ref());
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
    pub fn new_statechange<F: FnMut(&PlyrStateChangeEvent) + 'static>(
        target: &Plyr,
        mut callback: F,
    ) -> Result<Self, PlyrError> {
        if let Provider::youtube = target.provider() {
            let callback = move |event: &JsValue| {
                callback(Into::<PlyrStateChangeEvent>::into(event.clone()).as_ref())
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
    pub fn once_statechange<F: FnOnce(&PlyrStateChangeEvent) + 'static>(
        target: &Plyr,
        callback: F,
    ) -> Result<Self, PlyrError> {
        if let Provider::youtube = target.provider() {
            let callback = move |event: &JsValue| {
                callback(Into::<PlyrStateChangeEvent>::into(event.clone()).as_ref())
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
    pub fn forget(&mut self) {
        self.base_event_listener.forget();
    }
}
