interface Language {
  code: string;
  label: string;
}

type LanguageConfig = {
  sourceLang: Language;
  targetLang: Language;
};

interface ResultTexts {
  translate?: string;
  correct?: string;
  refine?: string;
}

type Mode = "Trans