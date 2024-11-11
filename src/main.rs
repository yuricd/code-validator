use scraper::Html;
use std::env;

#[derive(Debug)]
enum ValidatorKind {
    Html,
    JavaScript,
}

impl Default for ValidatorKind {
    fn default() -> Self {
        ValidatorKind::Html
    }
}

#[derive(Default, Debug)]
struct Validator {
    kind: ValidatorKind,
    template_code: String,
    user_code: String,
    expected_output: String,
}

const PLACEHOLDER: &str = "____";

fn main() {
    let mut validator = Validator::default();
    let mut args = env::args();

    while let Some(arg) = args.next() {
        match &arg[..] {
            "--kind" => {
                if let Some(arg_kind) = args.next() {
                    match arg_kind.as_str() {
                        "html" => validator.kind = ValidatorKind::Html,
                        "javascript" => validator.kind = ValidatorKind::JavaScript,
                        _ => {
                            println!("Allowed values for \"kind\" are \"html\" and \"javascript\".")
                        }
                    }
                } else {
                    println!("Allowed values for \"kind\" are \"html\" and \"javascript\".");
                }
            }
            "--template" => {
                if let Some(arg_template) = args.next() {
                    validator.template_code = arg_template
                } else {
                    println!("A template code must be provided.");
                }
            }
            "--user-code" => {
                if let Some(arg_user_code) = args.next() {
                    validator.user_code = arg_user_code
                } else {
                    println!("A user code must be provided.");
                }
            }
            "--expected" => {
                if let Some(arg_expected) = args.next() {
                    validator.expected_output = arg_expected
                } else {
                    println!("An expected output must be provided.");
                }
            }
            _ => println!("No such arg {}.", &arg),
        }
    }

    println!("{:?}", validator);
}

// cargo run -- --kind html --template "<div>____</div>" --user-code "<div>Mark</div>" --expected "<div>Mark H</div>"

impl Validator {
    fn validate_html(&self) -> bool {
        let user_replaced_code = &self.template_code.replace(PLACEHOLDER, &self.user_code);
        let parsed_user_code = Html::parse_fragment(&user_replaced_code);
        let parsed_expected = Html::parse_fragment(&self.expected_output);
        println!("{} == {}", parsed_user_code.html(), parsed_expected.html());
        parsed_user_code.html() == parsed_expected.html()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Validator, ValidatorKind};

    #[test]
    fn test_validate_html() {
        let validator = Validator {
            kind: ValidatorKind::Html,
            user_code: String::from("Mark"),
            template_code: String::from("<div>____</div>"),
            expected_output: String::from("<div>Mark</div>"),
        };

        assert_eq!(validator.validate_html(), true)
    }
}
