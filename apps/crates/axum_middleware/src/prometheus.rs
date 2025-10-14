//!  middleware to collect HTTP metrics for Axum applications.

use axum_prometheus::{
    AXUM_HTTP_REQUESTS_DURATION_SECONDS, GenericMetricLayer, Handle, PrometheusMetricLayerBuilder,
    metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle},
    utils::SECONDS_DURATION_BUCKETS,
};

/// prometheus middleware
pub fn prometheus_layer_metric_handle<'a>() -> (
    GenericMetricLayer<'a, PrometheusHandle, Handle>,
    PrometheusHandle,
) {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayerBuilder::new()
        .with_prefix("builder-example")
        // ignore reporting requests that match "/metrics"
        .with_ignore_pattern("/metrics")
        // if the any of the second argument matches, report them at the `/foo` endpoint
        .with_group_patterns_as("/foo", &["/foo/{bar}", "/foo/{bar}/{baz}"])
        // build a custom PrometheusHandle
        .with_metrics_from_fn(|| {
            PrometheusBuilder::new()
                .set_buckets_for_metric(
                    Matcher::Full(AXUM_HTTP_REQUESTS_DURATION_SECONDS.to_string()),
                    SECONDS_DURATION_BUCKETS,
                )
                .unwrap()
                .install_recorder()
                .unwrap()
        })
        .build_pair();

    (prometheus_layer, metric_handle)
}
