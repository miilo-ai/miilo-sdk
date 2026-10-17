//! Optional transport adapter. Attach only to explicit, non-secret read routes.
use http::{Extensions, HeaderMap, StatusCode, Version, header};
use tower_http::compression::{
    CompressionLayer, CompressionLevel,
    predicate::{Predicate, SizeAbove},
};
fn eligible(status: StatusCode, _: Version, headers: &HeaderMap, _: &Extensions) -> bool {
    let json = headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| {
            v.split(';')
                .next()
                .is_some_and(|v| v.trim() == "application/json")
        });
    let protected = headers.get_all(header::CACHE_CONTROL).iter().any(|v| {
        v.to_str().map_or(true, |v| {
            v.split(',')
                .any(|part| part.trim().eq_ignore_ascii_case("no-transform"))
        })
    });
    status.is_success()
        && json
        && !protected
        && !headers.contains_key(header::SET_COOKIE)
        && !headers.contains_key(header::CONTENT_ENCODING)
}
pub fn layer() -> CompressionLayer<impl Predicate + Send + Sync + 'static> {
    CompressionLayer::new()
        .quality(CompressionLevel::Fastest)
        .compress_when(SizeAbove::new(1024).and(eligible))
}
#[cfg(test)]
mod tests {
    use super::eligible;
    use http::{Extensions, HeaderMap, StatusCode, Version};
    #[test]
    fn rejects_streams_cookies_encoded_and_no_transform() {
        let mut headers = HeaderMap::new();
        for content in ["text/event-stream", "application/x-ndjson", "audio/wav"] {
            headers.insert("content-type", content.parse().unwrap());
            assert!(!eligible(
                StatusCode::OK,
                Version::HTTP_11,
                &headers,
                &Extensions::new()
            ));
        }
        headers.insert("content-type", "application/json".parse().unwrap());
        assert!(eligible(
            StatusCode::OK,
            Version::HTTP_11,
            &headers,
            &Extensions::new()
        ));
        for (name, value) in [
            ("set-cookie", "session=x"),
            ("content-encoding", "gzip"),
            ("cache-control", "private, no-transform"),
        ] {
            let mut h = headers.clone();
            h.insert(name, value.parse().unwrap());
            assert!(!eligible(
                StatusCode::OK,
                Version::HTTP_11,
                &h,
                &Extensions::new()
            ));
        }
    }
}
