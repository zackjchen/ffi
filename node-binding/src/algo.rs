use algo::{Algo, AlgoType};
use napi_derive::napi;

#[napi(js_name = "Algo")]
pub struct JsAlgo {
  inner: Algo,
}

#[napi]
impl JsAlgo {
  #[napi(constructor)]
  pub fn new(name: String) -> Self {
    let algo_type = match name.as_str() {
      "blake3" => AlgoType::Blake3,
      _ => AlgoType::Default,
    };

    Self {
      inner: Algo::new(algo_type),
    }
  }

  #[napi]
  pub fn get_name(&self) -> String {
    self.inner.get_name().to_string()
  }

  #[napi]
  pub fn hash(&self, data: String) -> String {
    self.inner.hash(data)
  }
}
