use napi_derive::napi;

use napi::Result;

#[napi(object)]
#[derive(Debug)]
struct Option1 {
  pub name: &'static str,
}

#[napi(object)]
#[derive(Debug)]
struct Option2 {
  pub name: &'static str,
}

#[napi(object)]
#[derive(Debug)]
struct Input {
  pub option1: Option<Option1>,
  pub option2: Option<Vec<Option2>>,
}

#[napi(object)]
#[derive(Debug)]
struct Option1String {
  pub name: String,
}

#[napi(object)]
#[derive(Debug)]
struct Option2String {
  pub name: String,
}

#[napi(object)]
#[derive(Debug)]
struct InputString {
  pub option1: Option<Option1String>,
  pub option2: Option<Vec<Option2String>>,
}

#[napi]
fn transform(option: Input) -> Result<Input> {
  Ok(option)
}

#[napi]
fn transform_with_string(option: InputString) -> Result<InputString> {
  Ok(option)
}

#[test]
fn should_work() -> Result<()> {
  let result = transform(Input {
    option1: Some(Option1 { name: "whh" }),
    option2: None,
  })?;

  println!("{:#?}", result);

  Ok(())
}
