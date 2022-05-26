// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::PatternWhiteSpaceV1Marker>
    for super::super::BakedDataProvider
{
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::PatternWhiteSpaceV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions.with_req(
                    <icu_properties::provider::PatternWhiteSpaceV1Marker>::KEY,
                    req,
                )
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct =
    &'static <icu_properties::provider::PatternWhiteSpaceV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1 {
    inv_list: unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::UnicodeSet::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    9u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 32u8, 0u8, 0u8, 0u8, 33u8, 0u8, 0u8,
                    0u8, 133u8, 0u8, 0u8, 0u8, 134u8, 0u8, 0u8, 0u8, 14u8, 32u8, 0u8, 0u8, 16u8,
                    32u8, 0u8, 0u8, 40u8, 32u8, 0u8, 0u8, 42u8, 32u8, 0u8, 0u8,
                ])
            },
            11usize,
        )
    },
};