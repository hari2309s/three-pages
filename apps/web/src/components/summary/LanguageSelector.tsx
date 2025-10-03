import { SUPPORTED_LANGUAGES, SUMMARY_STYLES } from "@/lib/constants";

interface LanguageSelectorProps {
  selectedLanguage: string;
  selectedStyle: string;
  onLanguageChange: (language: string) => void;
  onStyleChange: (style: string) => void;
}

export const LanguageSelector = ({
  selectedLanguage,
  selectedStyle,
  onLanguageChange,
  onStyleChange,
}: LanguageSelectorProps) => {
  return (
    <div className="flex flex-col gap-4 sm:flex-row">
      <div className="flex-1">
        <label htmlFor="language" className="mb-2 block text-sm font-medium">
          Summary Language
        </label>
        <select
          id="language"
          value={selectedLanguage}
          onChange={(e) => onLanguageChange(e.target.value)}
          className="w-full rounded-lg border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
        >
          {SUPPORTED_LANGUAGES.map((lang) => (
            <option key={lang.code} value={lang.code}>
              {lang.flag} {lang.name}
            </option>
          ))}
        </select>
      </div>

      <div className="flex-1">
        <label htmlFor="style" className="mb-2 block text-sm font-medium">
          Summary Style
        </label>
        <select
          id="style"
          value={selectedStyle}
          onChange={(e) => onStyleChange(e.target.value)}
          className="w-full rounded-lg border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
        >
          {SUMMARY_STYLES.map((style) => (
            <option key={style.value} value={style.value}>
              {style.label} - {style.description}
            </option>
          ))}
        </select>
      </div>
    </div>
  );
};
