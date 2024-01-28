use crate::providers::{self, base::get_provider};
use providers::base::Provider;

const DEFAULT_TRANSLATION_PROMPT: &str = "
    You're a good translator.
    Only answer the translated text with the following rules and do not showing your thought:
    - put your answer in <ans></ans> block and do not include anything else because only the text inside <ans></ans> block will be extracted.
    Example 1:
        Original langue: English, Target language: Tiếng Việt
        Input: I'm running
        Your answer: <ans>Tôi đang chạy bộ</ans>
    Example 2:
        Original langue: English, Target language: Español
        Input: I'm running
        Your answer: <ans>Estoy corriendo</ans>
    Please translate the text in original language {original_lang} in the the text block below to {target_lang}:
";
const DEFAULT_CORRECTION_PROMPT: &str = "
    You're a teacher and you're correcting a student's homework.
    Only answer the correct text without explanation,
    put your answer in <ans></ans> block and do not include anything else because only the text inside <ans></ans> block will be extracted.
    Example:
        I