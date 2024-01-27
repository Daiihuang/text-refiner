use crate::providers::{self, base::get_provider};
use providers::base::Provider;

const DEFAULT_TRANSLATION_PROMPT: &str = "
    You're a good translator.
    Only answer the translated text with the following rules and do not showing your thought:
    - put your answer in <ans></ans> block and do not include anything else because only the text inside <ans></ans> block will be extracted.
    Example 1:
        Original langue: English, Target language: Tiếng Việt
        Input: I'm running
        Your answer: <ans>Tôi đang 