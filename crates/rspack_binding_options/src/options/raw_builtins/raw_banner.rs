use derivative::Derivative;
use napi::Either;
use napi_derive::napi;
use rspack_binding_values::{into_asset_conditions, JsChunk, RawAssetConditions};
use rspack_error::Result;
use rspack_napi::threadsafe_function::ThreadsafeFunction;
use rspack_plugin_banner::{BannerContent, BannerContentFnCtx, BannerPluginOptions};

#[napi(object)]
pub struct RawBannerContentFnCtx {
  pub hash: String,
  pub chunk: JsChunk,
  pub filename: String,
}

impl<'a> From<BannerContentFnCtx<'a>> for RawBannerContentFnCtx {
  fn from(value: BannerContentFnCtx) -> Self {
    Self {
      hash: value.hash.to_string(),
      chunk: JsChunk::from(value.chunk),
      filename: value.filename.to_string(),
    }
  }
}

type RawBannerContent = Either<String, ThreadsafeFunction<RawBannerContentFnCtx, String>>;
struct RawBannerContentWrapper(RawBannerContent);

impl TryFrom<RawBannerContentWrapper> for BannerContent {
  type Error = rspack_error::Error;
  fn try_from(value: RawBannerContentWrapper) -> Result<Self> {
    match value.0 {
      Either::A(s) => Ok(Self::String(s)),
      Either::B(f) => Ok(BannerContent::Fn(Box::new(
        move |ctx: BannerContentFnCtx| {
          let ctx = ctx.into();
          let f = f.clone();
          Box::pin(async move { f.call(ctx).await })
        },
      ))),
    }
  }
}

#[derive(Derivative)]
#[derivative(Debug)]
#[napi(object, object_to_js = false)]
pub struct RawBannerPluginOptions {
  #[derivative(Debug = "ignore")]
  #[napi(ts_type = "string | ((...args: any[]) => any)")]
  pub banner: RawBannerContent,
  pub entry_only: Option<bool>,
  pub footer: Option<bool>,
  pub raw: Option<bool>,
  pub stage: Option<i32>,
  #[napi(ts_type = "string | RegExp | (string | RegExp)[]")]
  pub test: Option<RawAssetConditions>,
  #[napi(ts_type = "string | RegExp | (string | RegExp)[]")]
  pub include: Option<RawAssetConditions>,
  #[napi(ts_type = "string | RegExp | (string | RegExp)[]")]
  pub exclude: Option<RawAssetConditions>,
}

impl TryFrom<RawBannerPluginOptions> for BannerPluginOptions {
  type Error = rspack_error::Error;
  fn try_from(value: RawBannerPluginOptions) -> Result<Self> {
    Ok(BannerPluginOptions {
      banner: RawBannerContentWrapper(value.banner).try_into()?,
      entry_only: value.entry_only,
      footer: value.footer,
      raw: value.raw,
      stage: value.stage,
      test: value.test.map(into_asset_conditions),
      include: value.include.map(into_asset_conditions),
      exclude: value.exclude.map(into_asset_conditions),
    })
  }
}
