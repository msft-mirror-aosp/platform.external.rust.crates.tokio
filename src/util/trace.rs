cfg_trace! {
    cfg_rt! {
        pub(crate) use tracing::instrument::Instrumented;

        #[inline]
        #[cfg_attr(tokio_track_caller, track_caller)]
        pub(crate) fn task<F>(task: F, kind: &'static str, name: Option<&str>) -> Instrumented<F> {
            use tracing::instrument::Instrument;
            #[cfg(tokio_track_caller)]
            let location = std::panic::Location::caller();
            #[cfg(tokio_track_caller)]
            let span = tracing::trace_span!(
                target: "tokio::task",
                "runtime.spawn",
                %kind,
                task.name = %name.unwrap_or_default(),
                loc.file = location.file(),
                loc.line = location.line(),
                loc.col = location.column(),
            );
            #[cfg(not(tokio_track_caller))]
            let span = tracing::trace_span!(
                target: "tokio::task",
                "runtime.spawn",
                %kind,
                task.name = %name.unwrap_or_default(),
            );
            task.instrument(span)
        }
    }
}
cfg_time! {
    #[cfg_attr(tokio_track_caller, track_caller)]
    pub(crate) fn caller_location() -> Option<&'static std::panic::Location<'static>> {
        #[cfg(all(tokio_track_caller, tokio_unstable, feature = "tracing"))]
        return Some(std::panic::Location::caller());
        #[cfg(not(all(tokio_track_caller, tokio_unstable, feature = "tracing")))]
        None
    }
}

cfg_not_trace! {
    cfg_rt! {
        #[inline]
        pub(crate) fn task<F>(task: F, _: &'static str, _name: Option<&str>) -> F {
            // nop
            task
        }
    }
}
